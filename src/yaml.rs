use rusqlite;
use yaml_rust::yaml;

use std::collections::HashMap;


mod sql;

pub use sql::*;


pub fn extract(
    doc: &mut yaml::Yaml,
    tx: &rusqlite::Transaction,
    table_columns: &HashMap<String, crate::sqlite::Zero>,
) {
    match doc {
        yaml::Yaml::Array(ref mut array) => {
            for yaml_value in array {
                extract(yaml_value, tx, table_columns);
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

            let values = hash.values().map(|v| Yaml(v));
            stmt.insert(rusqlite::params_from_iter(values)).unwrap();
        }
        _ => {}
    }
}
