// Copyright (c) 2022  Teddy Wing
//
// This file is part of Yaqlite.
//
// Yaqlite is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Yaqlite is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Yaqlite. If not, see <https://www.gnu.org/licenses/>.

// Copyright (c) 2022  Teddy Wing
//
// This file is part of Yaqlite.
//
// Yaqlite is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Yaqlite is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Yaqlite. If not, see <https://www.gnu.org/licenses/>.

use rusqlite;

use std::collections::HashSet;


/// Get a list of the column names in the given table.
pub fn get_column_names(
    dbconn: &rusqlite::Connection,
    table_name: &str,
) -> Result<HashSet<String>, crate::Error> {
    let mut column_names = HashSet::new();

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

        column_names.insert(row);
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
