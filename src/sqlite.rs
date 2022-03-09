use rusqlite;


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
