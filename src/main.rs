use rusqlite;
use yaml_rust::yaml;

fn main() {
    println!("Hello, world!");

    // Get column names from SQLite

    let text_data = std::fs::read_to_string("test.yml").unwrap();

    let yaml_data = yaml::YamlLoader::load_from_str(&text_data).unwrap();

    for doc in &yaml_data {
        yaml_extract(&doc);
    }

    dbg!(yaml_data);
}

fn yaml_extract(doc: &yaml::Yaml) {
    match doc {
        yaml::Yaml::Array(ref array) => {
            for yaml_value in array {
                yaml_extract(yaml_value);
            }
        }
        yaml::Yaml::Hash(ref hash) => {
            // Begin transaction
            for (k, v) in hash {
                // TODO: Put k,v in a HashMap prepared for SQLite interfacing
                dbg!(k, v);

                // Each hash is a new record for SQLite insertion

                // If key matches a column name, add it to the insert statement
            }
        }
        _ => {}
    }
}
