use rusqlite;

use std::collections::HashMap;


#[derive(Debug)]
pub struct Zero;


pub fn get_column_names(
    dbconn: &rusqlite::Connection,
    table_name: &str,
// TODO: Use a HashSet instead
) -> Result<HashMap<String, Zero>, crate::Error> {
    let mut column_names = HashMap::new();

    let mut stmt = dbconn.prepare(
        &format!(
            r#"
                SELECT "name"
                FROM pragma_table_info("{}");
            "#,
            table_name,
        ),
    )?;

    let rows = stmt.query_map(
        [],
        |row| row.get(0),
    )?;

    for row_result in rows {
        let row = row_result?;

        column_names.insert(row, Zero{});
    }

    Ok(column_names)
}


/// Get the name of the given table's primary key.
pub fn table_primary_key_column(
    dbconn: &rusqlite::Connection,
    table_name: &str,
) -> Result<String, crate::Error> {
    let mut stmt = dbconn.prepare(r#"
        SELECT "name"
        FROM pragma_table_info(:table)
        WHERE "pk" != 0;'
    "#)?;

    let pk_column: String = stmt.query_row(
        &[(":table", table_name)],
        |row| row.get(0),
    )?;

    Ok(pk_column)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_primary_key_column_gets_primary_key_name() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
                CREATE TABLE "test" (
                    id INTEGER PRIMARY KEY,
                    count INTEGER
                );
            "#,
            []
        ).unwrap();

        let column_name = table_primary_key_column(&conn, "test").unwrap();

        assert_eq!("id", column_name);

        conn.close().unwrap();
    }
}
