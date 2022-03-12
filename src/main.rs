use rusqlite;
use yaml_rust::yaml;


fn main() {
    println!("Hello, world!");

    // Get column names from SQLite

    let mut dbconn = rusqlite::Connection::open("./test.db").unwrap();

    let table_columns = yaqlite::sqlite::get_column_names(&dbconn, "people");
    dbg!(&table_columns);

    let text_data = std::fs::read_to_string("test2.yml").unwrap();

    let mut yaml_data = yaml::YamlLoader::load_from_str(&text_data).unwrap();

    for mut doc in &mut yaml_data {
        let tx = dbconn.transaction().unwrap();

        yaqlite::yaml::extract(&mut doc, &tx, &table_columns);

        tx.commit().unwrap();
    }

    dbg!(yaml_data);

    dbconn.close().unwrap();
}
