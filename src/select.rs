pub fn select(
    dbconn: &rusqlite::Connection,
    table_name: &str,
    record_id: &str,
) -> yaml_rust::Yaml {
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

    let rows = stmt.query_map(
        rusqlite::named_params! {
            ":pk_column": "id",
            ":pk": record_id,
        },
        |row| {
            Ok(())
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
