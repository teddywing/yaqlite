pub fn select(
    dbconn: &rusqlite::Connection,
    table_name: &str,
    record_id: &str,
) -> Result<yaml_rust::Yaml, crate::Error> {
    select_by_column(
        dbconn,
        table_name,
        &crate::sqlite::table_primary_key_column(dbconn, table_name)?,
        record_id,
    )
}

pub fn select_by_column(
    dbconn: &rusqlite::Connection,
    table_name: &str,
    primary_key_column: &str,
    record_id: &str,
) -> Result<yaml_rust::Yaml, crate::Error> {
    use crate::yaml::Yaml;

    let mut stmt = dbconn.prepare(
        &format!(
            r#"
                SELECT
                    *
                FROM "{}"
                WHERE "{}" = :pk;
            "#,
            table_name,
            primary_key_column,
        ),
    )?;

    let column_names: Vec<String> = stmt
        .column_names()
        .into_iter()
        .map(String::from)
        .collect();

    let rows = stmt.query_map(
        rusqlite::named_params! {
            ":pk": record_id,
        },
        |row| {
            let mut data = yaml_rust::yaml::Hash::new();

            for (i, column) in column_names.iter().enumerate() {
                // Don't include the primary key column in the resulting hash as
                // it should not be editable.
                if column == primary_key_column {
                    continue
                }

                let column_name = column.to_owned();
                let column_value: Yaml = row.get(i)?;

                data.insert(
                    yaml_rust::Yaml::String(column_name),
                    column_value.into_inner(),
                );
            }

            Ok(data)
        },
    )?;

    let mut row = None;
    for row_result in rows {
        row = Some(yaml_rust::Yaml::Hash(row_result?));
    }

    if let Some(r) = row {
        return Ok(r);
    }

    Ok(yaml_rust::Yaml::Null)
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

        let mut yaml_hash = yaml_rust::yaml::Hash::new();
        yaml_hash.insert(
            yaml_rust::Yaml::String("count".to_owned()),
            yaml_rust::Yaml::Integer(record.count.into()),
        );
        yaml_hash.insert(
            yaml_rust::Yaml::String("description".to_owned()),
            yaml_rust::Yaml::String(record.description.clone()),
        );

        let expected = yaml_rust::Yaml::Hash(yaml_hash);

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

            let got = select(&conn, "test", "1").unwrap();

            dbg!(&got);

            assert_eq!(expected, got);
        }

        conn.close().unwrap();
    }
}
