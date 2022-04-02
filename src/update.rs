// Copyright (c) 2022  Teddy Wing
//
// This file is part of Yaqlite.
//
// Yaqlite is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Yaqlite is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Yaqlite. If not, see <https://www.gnu.org/licenses/>.

/// Update a YAML record in the given database.
pub fn update(
    dbconn: &mut rusqlite::Connection,
    table_name: &str,
    record_id: &str,
    data: &mut yaml_rust::Yaml,
) -> Result<(), crate::Error> {
    let table_columns = crate::sqlite::get_column_names(&dbconn, table_name)?;

    let tx = dbconn.transaction()?;

    crate::yaml::db_update(
        data,
        &tx,
        &table_name,
        &table_columns,
        // TODO: dynamic or user-supplied
        "id",
        record_id,
    )?;

    tx.commit()?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_database_record_from_yaml() {
        #[derive(Debug, PartialEq)]
        struct TestRecord {
            id: i8,
            count: i16,
            weight: f32,
            description: String,
        }

        let mut conn = rusqlite::Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
                CREATE TABLE "test" (
                    id INTEGER PRIMARY KEY,
                    count INTEGER,
                    weight REAL,
                    description TEXT
                );
            "#,
            [],
        ).unwrap();

        {
            let mut stmt = conn.prepare(r#"
                INSERT INTO "test"
                    (count, weight, description)
                VALUES
                    (?, ?, ?);
            "#).unwrap();

            stmt.insert(
                rusqlite::params![
                    55_i16,
                    0.8_f32,
                    "Ounces or grams?",
                ],
            ).unwrap();
        }

        let expected = TestRecord {
            id: 1,
            count: 28,
            weight: 1.2,
            description: r#"This is a multiline

description."#.to_owned(),
        };

        let mut yaml_data = yaml_rust::YamlLoader::load_from_str(
            &format!(
r#"count: {}
weight: {}
description: |-
  This is a multiline

  description.
"#,
                expected.count,
                expected.weight,
            ),
        ).unwrap();
        let mut yaml_record = yaml_data.get_mut(0).unwrap();

        update(&mut conn, "test", "1", &mut yaml_record).unwrap();

        {
            let mut stmt = conn.prepare(r#"
                SELECT
                    "id", "count", "weight", "description"
                FROM "test"
                WHERE "id" = 1;
            "#).unwrap();

            let got = stmt.query_row(
                [],
                |row| {
                    Ok(
                        TestRecord {
                            id: row.get(0).unwrap(),
                            count: row.get(1).unwrap(),
                            weight: row.get(2).unwrap(),
                            description: row.get(3).unwrap(),
                        }
                    )
                }
            ).unwrap();

            assert_eq!(expected, got);
        }

        conn.close().unwrap();
    }
}
