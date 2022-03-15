pub fn select(
    dbconn: &rusqlite::Connection,
    table_name: &str,
    record_id: &str,
) -> yaml_rust::Yaml {
    use crate::yaml::Yaml;

    let mut stmt = dbconn.prepare(
        &format!(
            r#"
                SELECT
                    *
                FROM {}
                WHERE :pk_column = :pk;
            "#,
            table_name,
        ),
    ).unwrap();

    let column_count = stmt.column_count();

    let rows = stmt.query_map(
        rusqlite::named_params! {
            ":pk_column": "id",
            ":pk": record_id,
        },
        |row| {
            // let data: [dyn rusqlite::types::FromSql; column_count] = [rusqlite::types::Null; column_count];
            // let data: Vec<dyn rusqlite::types::FromSql>
            //     = Vec::with_capacity(column_count);
            // let data = Vec::with_capacity(column_count);
            let data: Vec<Yaml> = Vec::with_capacity(column_count);

            // for i in 0..=column_count {
            //     data.push(row.get(i));
            // }

            // TODO: column values must be converted to yaml_rust::Yaml in this
            // closure.

            for i in 0..=column_count {
                data.push(row.get(i)?);
            }

            Ok(data)
        },
    ).unwrap();

    // sqlite3 -header test.db '
    // SELECT "name"
    // FROM pragma_table_info("test")
    // WHERE "pk" != 0;'

    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_extracts_a_database_record_as_yaml() {
        struct TestRecord {
            count: i16,
            description: String,
        }

        let record = TestRecord {
            count: 99,
            description: "This is a test.

With multiple paragraphs.".to_owned(),
        };

        let conn = rusqlite::Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
                CREATE TABLE "test" (
                    id INTEGER PRIMARY KEY,
                    count INTEGER,
                    description TEXT
                );
            "#,
            []
        ).unwrap();

        {
            let mut stmt = conn.prepare(r#"
                INSERT INTO "test"
                    (count, description)
                VALUES
                    (?, ?);
            "#).unwrap();

            stmt.insert(
                rusqlite::params![record.count, record.description],
            ).unwrap();

            let got = select(&conn, "test", "1");

            dbg!(&got);
        }

        conn.close().unwrap();
    }
}
