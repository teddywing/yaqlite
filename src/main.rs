use rusqlite;
use yaml_rust::yaml;

fn main() {
    println!("Hello, world!");

    // Get column names from SQLite

    let mut dbconn = rusqlite::Connection::open("./test.db").unwrap();

    let table_columns = get_column_names(&dbconn);
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
    table_columns: &HashMap<String, rusqlite::types::Type>,
) {
    match doc {
        yaml::Yaml::Array(ref mut array) => {
            for yaml_value in array {
                yaml_extract(yaml_value, tx, table_columns);
            }
        }
        yaml::Yaml::Hash(ref mut hash) => {
            // Begin transaction
            // for (k, v) in hash {
            //     // TODO: Put k,v in a HashMap prepared for SQLite interfacing
            //     // Each hash is a new record for SQLite insertion
            //
            //     // If key matches a column name, add it to the insert statement
            //
            //     if table_columns.contains_key(k.as_str().unwrap()) {
            //         dbg!(k, v);
            //     }
            // }

            // let keys: Vec<&str> = hash
            //     .keys()
            //     .map(|k| k.as_str().unwrap())
            //     .collect();
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

struct Yaml<'a>(&'a yaml::Yaml);

impl<'a> rusqlite::ToSql for Yaml<'a> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::ToSqlOutput;

        let sql_output = match self.0 {
            yaml::Yaml::Real(_) => ToSqlOutput::from(self.0.as_f64().unwrap()),
            yaml::Yaml::Integer(_) => ToSqlOutput::from(self.0.as_i64().unwrap()),
            yaml::Yaml::String(_) => ToSqlOutput::from(self.0.as_str().unwrap()),
            yaml::Yaml::Boolean(_) => ToSqlOutput::from(self.0.as_bool().unwrap()),
            yaml::Yaml::Array(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Hash(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Alias(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Null => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::BadValue => ToSqlOutput::from(rusqlite::types::Null),
        };

        Ok(sql_output)
    }
}
