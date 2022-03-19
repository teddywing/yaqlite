use std::collections::HashMap;


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

    let column_count = stmt.column_count();
    // let column_names: Vec<yaml_rust::Yaml> = stmt.column_names()
    //     .iter()
    //     .map(|col| yaml_rust::Yaml::String((*col).to_owned()))
    //     .collect();
    // dbg!(column_names);
    // let column_names = stmt.column_names();

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
            // let mut data: Vec<Yaml> = Vec::with_capacity(column_count);
            //
            // for i in 0..column_count {
            //     data.push(row.get(i)?);
            // }

            // let mut data: HashMap<&yaml_rust::Yaml, Yaml> = HashMap::new();
            let mut data = yaml_rust::yaml::Hash::new();

            for (i, column) in column_names.iter().enumerate() {
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

            // let mut data: Vec<yaml_rust::Yaml> = Vec::with_capacity(column_count);
            //
            // for i in 0..column_count {
            //     let column_value: Yaml = row.get(i)?;
            //     data.push(column_value.into_inner());
            // }

            Ok(data)
        },
    )?;

    let mut row = None;
    for row_result in rows {
        // let mut data = yaml_rust::yaml::Hash::new();
        // let column_values = row_result.unwrap();
        //
        // for (i, column) in column_names.iter().enumerate() {
        //     data.insert(
        //         yaml_rust::Yaml::String(column.to_string()),
        //         column_values[i],
        //     );
        // }
        //
        // row = Some(yaml_rust::Yaml::Hash(data));

        row = Some(yaml_rust::Yaml::Hash(row_result?));
        dbg!(&row);
    }

    if let Some(r) = row {
        return Ok(r);
    }

    // todo!();
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
