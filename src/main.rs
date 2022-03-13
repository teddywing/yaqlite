use rusqlite;
use yaml_rust::yaml;


fn main() {
    println!("Hello, world!");

    // Get column names from SQLite

    let mut dbconn = rusqlite::Connection::open("./test.db").unwrap();

    let text_data = std::fs::read_to_string("test2.yml").unwrap();

    let mut yaml_data = yaml::YamlLoader::load_from_str(&text_data).unwrap();

    yaqlite::insert(&mut dbconn, "people", &mut yaml_data).unwrap();

    dbg!(yaml_data);

    dbconn.close().unwrap();
}
