use rusqlite;

use std::collections::HashMap;


/// Get the fundamental SQLite datatype for a given type name.
///
/// Use the SQLite rules for type affinity described in:
/// https://sqlite.org/datatype3.html#determination_of_column_affinity
pub fn affinity(type_name: &str) -> rusqlite::types::Type {
    use rusqlite::types::Type;

    let type_name = type_name.to_uppercase();

    if type_name.contains("INT") {
        return Type::Integer;
    } else if type_name.contains("CHAR")
        || type_name.contains("CLOB")
        || type_name.contains("TEXT")
    {
        return Type::Text;
    } else if type_name.contains("BLOB")
        || type_name.is_empty()
    {
        return Type::Blob;
    } else if type_name.contains("REAL")
        || type_name.contains("FLOA")
        || type_name.contains("DOUB")
    {
        return Type::Real;
    }

    // TODO: Numeric affinity

    Type::Text
}


#[derive(Debug)]
pub struct Zero {}


pub fn get_column_names(dbconn: &rusqlite::Connection) -> HashMap<String, Zero> {
    let mut column_names = HashMap::new();

    let mut stmt = dbconn.prepare(r#"
        SELECT "name"
        FROM pragma_table_info("people");
    "#).unwrap();

    let rows = stmt.query_map(
        [],
        |row| row.get(0),
    ).unwrap();

    for row_result in rows {
        let row = row_result.unwrap();

        column_names.insert(row, Zero{});
    }

    column_names
}
