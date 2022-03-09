use rusqlite;


/// Get the fundamental SQLite datatype for a given type name.
///
/// Use the SQLite rules for type affinity described in:
/// https://sqlite.org/datatype3.html#determination_of_column_affinity
pub fn affinity(type_name: &str) -> rusqlite::types::Type {
}
