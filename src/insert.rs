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

/// Insert YAML `data` records into the given database.
pub fn insert(
    dbconn: &mut rusqlite::Connection,
    table_name: &str,
    data: &mut [yaml_rust::Yaml],
) -> Result<(), crate::Error> {
    let table_columns = crate::sqlite::get_column_names(&dbconn, table_name)?;

    for mut doc in data {
        let tx = dbconn.transaction()?;

        crate::yaml::db_insert(&mut doc, &tx, &table_name, &table_columns)?;

        tx.commit()?;
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestRecord {
        id: i8,
        count: i16,
        weight: f32,
        description: String,
    }

    fn test_yaml_insert(yaml_str: &str, expected: &[TestRecord]) {
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
            []
        ).unwrap();

        let mut data = yaml_rust::YamlLoader::load_from_str(&yaml_str).unwrap();

        insert(&mut conn, "test", &mut data).unwrap();

        {
            let mut stmt = conn.prepare(r#"
                SELECT
                    id, count, weight, description
                FROM "test";
            "#).unwrap();

            let rows = stmt.query_map(
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

            let got: Vec<TestRecord> = rows.map(|r| r.unwrap()).collect();

            assert_eq!(expected, got);
        }

        conn.close().unwrap();
    }

    #[test]
    fn inserts_yaml_in_database() {
        let expected = TestRecord {
            id: 1,
            count: 99,
            weight: 3.14,
            description: r#"This is a test.
Another paragraph with a flowed line."#.to_owned(),
        };

        let description = r#"This is a test.

    Another paragraph
    with a flowed line."#;

        let yaml_str = format!(
r#"- description: >-
    {}
  count: {}
  weight: {}
"#,
            description,
            expected.count,
            expected.weight,
        );

        test_yaml_insert(&yaml_str, &vec![expected]);
    }

    #[test]
    fn ignores_yaml_fields_that_are_not_column_names() {
        let expected = TestRecord {
            id: 1,
            count: 55,
            weight: 7.65,
            description: "Some text content.".to_owned(),
        };

        let yaml_str = format!(
r#"- description: >-
    {}
  count: {}
  weight: {}
  nonexistent_column: Must not be inserted.
"#,
            expected.description,
            expected.count,
            expected.weight,
        );

        test_yaml_insert(&yaml_str, &vec![expected]);
    }

    #[test]
    fn inserts_multiple_records() {
        let expected = vec![
            TestRecord {
                id: 1,
                count: 10,
                weight: 33.2,
                description: "First".to_owned(),
            },
            TestRecord {
                id: 2,
                count: 12,
                weight: 180.5,
                description: "Second".to_owned(),
            },
        ];

        let yaml_str = format!(
r#"- description: {}
  count: {}
  weight: {}
- description: {}
  count: {}
  weight: {}
"#,
            expected[0].description,
            expected[0].count,
            expected[0].weight,
            expected[1].description,
            expected[1].count,
            expected[1].weight,
        );

        test_yaml_insert(&yaml_str, &expected);
    }

    #[test]
    fn inserts_yaml_hash() {
        let expected = TestRecord {
            id: 1,
            count: 255,
            weight: 86.6,
            description: "Some text content.".to_owned(),
        };

        let yaml_str = format!(
r#"description: {}
count: {}
weight: {}
"#,
            expected.description,
            expected.count,
            expected.weight,
        );

        test_yaml_insert(&yaml_str, &vec![expected]);
    }

    #[test]
    fn insert_ignores_non_column_yaml_fields() {
        let expected = TestRecord {
            id: 1,
            count: 102,
            weight: 79.4,
            description: "Some text content.".to_owned(),
        };

        let yaml_str = format!(
r#"description: {}
count: {}
weight: {}
key_is_not_a_column: 44
not-a-column: Text that shouldn't be inserted.
"#,
            expected.description,
            expected.count,
            expected.weight,
        );

        test_yaml_insert(&yaml_str, &vec![expected]);
    }
}
