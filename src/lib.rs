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

        crate::yaml::extract(&mut doc, &tx, &table_columns);

        tx.commit().unwrap();
    }
}
