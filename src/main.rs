use rusqlite;
use yaml_rust::yaml;

use std::collections::HashMap;


fn main() {
    println!("Hello, world!");

    // Get column names from SQLite

    let mut dbconn = rusqlite::Connection::open("./test.db").unwrap();

    let table_columns = yaqlite::sqlite::get_column_names(&dbconn);
    dbg!(&table_columns);

    let text_data = std::fs::read_to_string("test2.yml").unwrap();

    let mut yaml_data = yaml::YamlLoader::load_from_str(&text_data).unwrap();

    for mut doc in &mut yaml_data {
        let tx = dbconn.transaction().unwrap();

        yaml_extract(&mut doc, &tx, &table_columns);

        tx.commit().unwrap();
    }

    dbg!(yaml_data);

    dbconn.close().unwrap();
}

fn yaml_extract(
    doc: &mut yaml::Yaml,
    tx: &rusqlite::Transaction,
    table_columns: &HashMap<String, yaqlite::sqlite::Zero>,
) {
    match doc {
        yaml::Yaml::Array(ref mut array) => {
            for yaml_value in array {
                yaml_extract(yaml_value, tx, table_columns);
            }
        }
        yaml::Yaml::Hash(ref mut hash) => {
            let keys: Vec<yaml::Yaml> = hash.keys().map(|k| k.clone()).collect();
            let columns_as_yaml: Vec<yaml::Yaml> = table_columns.keys()
                .map(|c| yaml::Yaml::from_str(c))
                .collect();

            for key in keys.iter() {
                if !columns_as_yaml.contains(key) {
                    hash.remove(key);
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

            let values = hash.values().map(|v| yaqlite::yaml::Yaml(v));
            stmt.insert(rusqlite::params_from_iter(values)).unwrap();
        }
        _ => {}
    }
}
