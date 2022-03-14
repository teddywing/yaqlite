pub fn select(
    dbconn: &rusqlite::Connection,
    table_name: &str,
    record_id: &str,
) -> yaml_rust::Yaml {
    let mut stmt = dbconn.prepare(r#"
        SELECT
            x
        FROM :table
        WHERE :pk_column = :pk;
    "#).unwrap();

    let rows = stmt.query_map(
        &[
            (":table", table_name),
            (":pk_column", "id"),
            (":pk", record_id),
        ],
        |row| {
            Ok(())
        },
    ).unwrap();

    // sqlite3 -header test.db '
    // SELECT "name"
    // FROM pragma_table_info("test")
    // WHERE "pk" != 0;'

    todo!();
}
