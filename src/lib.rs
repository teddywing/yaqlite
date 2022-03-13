pub mod sqlite;
pub mod yaml;


pub fn insert(
    dbconn: &mut rusqlite::Connection,
    table_name: &str,
    data: &mut [yaml_rust::Yaml],
) {
    let table_columns = crate::sqlite::get_column_names(&dbconn, table_name);

    for mut doc in data {
        let tx = dbconn.transaction().unwrap();

        crate::yaml::extract(&mut doc, &tx, &table_name, &table_columns);

        tx.commit().unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inserts_yaml_in_database() {
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

        #[derive(Debug, PartialEq)]
        struct TestRecord {
            id: i8,
            count: i16,
            weight: f32,
            description: String,
        }

        let description = r#"This is a test.

    Another paragraph
    with a flowed line."#;

        let expected = TestRecord {
            id: 1,
            count: 99,
            weight: 3.14,
            description: r#"This is a test.
Another paragraph with a flowed line."#.to_owned(),
        };

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

        let mut data = yaml_rust::YamlLoader::load_from_str(&yaml_str).unwrap();

        insert(&mut conn, "test", &mut data);

        {
            let mut stmt = conn.prepare(r#"
                SELECT
                    id, count, weight, description
                FROM "test"
                LIMIT 1;
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

    #[test]
    fn ignores_yaml_fields_that_are_not_column_names() {
    }
}
