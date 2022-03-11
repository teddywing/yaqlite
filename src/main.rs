use rusqlite;
use yaml_rust::yaml;

fn main() {
    println!("Hello, world!");

    // Get column names from SQLite

    let mut dbconn = rusqlite::Connection::open("./test.db").unwrap();

    let table_columns = get_column_names(&dbconn);
    dbg!(&table_columns);

    let text_data = std::fs::read_to_string("test.yml").unwrap();

    let yaml_data = yaml::YamlLoader::load_from_str(&text_data).unwrap();

    for doc in &yaml_data {
        let tx = dbconn.transaction().unwrap();

        yaml_extract(&doc, &tx, &table_columns);

        tx.commit().unwrap();
    }

    dbg!(yaml_data);

    dbconn.close().unwrap();
}

fn yaml_extract(
    doc: &yaml::Yaml,
    tx: &rusqlite::Transaction,
    table_columns: &HashMap<String, rusqlite::types::Type>,
) {
    match doc {
        yaml::Yaml::Array(ref array) => {
            for yaml_value in array {
                yaml_extract(yaml_value, tx, table_columns);
            }
        }
        yaml::Yaml::Hash(ref hash) => {
            // Begin transaction
            for (k, v) in hash {
                // TODO: Put k,v in a HashMap prepared for SQLite interfacing
                // Each hash is a new record for SQLite insertion

                // If key matches a column name, add it to the insert statement

                if table_columns.contains_key(k.as_str().unwrap()) {
                    dbg!(k, v);
                }
            }

            let mut stmt = tx.prepare(
                &format!(
                    r#"
                        INSERT INTO "people"
                            ({})
                        VALUES
                            ({});
                    "#,
                    // Wrap column names in quotes.
                    hash.keys()
                        .map(|k| format!(r#""{}""#, k.as_str().unwrap()))
                        .collect::<Vec<String>>()
                        .join(", "),
                    // TODO: get len "?"s
                    format!("{}?", "?, ".repeat(hash.len() - 1)),
                )
            ).unwrap();

            // stmt.insert(rusqlite::params_from_iter(hash.values())).unwrap();

            // tx.execute(
            //     r#"
            //         INSERT INTO "people"
            //             ()
            //         VALUES
            //             ();
            //     "#,
            //     []
            // ).unwrap();
        }
        _ => {}
    }
}

#[derive(Debug)]
struct Zero {}

use std::collections::HashMap;

fn get_column_names(dbconn: &rusqlite::Connection) -> HashMap<String, rusqlite::types::Type> {
    let mut column_names = HashMap::new();

    let mut stmt = dbconn.prepare(r#"
        SELECT
            "name",
            "type"
        FROM pragma_table_info("people");
    "#).unwrap();

    let rows = stmt.query_map(
        [],
        |row| Ok((row.get(0).unwrap(), row.get(1).unwrap())),
    ).unwrap();

    for row_result in rows {
        // TODO: Get the type of the column.
        // $ sqlite3 -header test.db "select type from pragma_table_info(\"people\");"
        // type
        // text
        // text
        // text
        // integer
        // $ sqlite3 -header test.db "select type from pragma_table_info(\"test\");"
        // type
        // INTEGER
        // TEXT
        // DATETIME
        // INTEGER

        let row = row_result.unwrap();

        let type_name: String = row.1;

        let type_affinity = yaqlite::sqlite::affinity(&type_name);

        column_names.insert(row.0, type_affinity);
    }

    column_names
}
