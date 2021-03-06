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
use yaml_rust::yaml;

use std::collections::HashSet;


mod sql;
mod write;

pub(crate) use sql::*;

pub use write::*;


/// Insert a YAML document into the given table in the database.
pub fn db_insert(
    doc: &mut yaml::Yaml,
    tx: &rusqlite::Transaction,
    table_name: &str,
    table_columns: &HashSet<String>,
) -> Result<(), crate::Error> {
    match doc {
        yaml::Yaml::Array(ref mut array) => {
            for yaml_value in array {
                db_insert(yaml_value, tx, table_name, table_columns)?;
            }
        }
        yaml::Yaml::Hash(ref mut hash) => {
            use std::borrow::Cow;

            let keys: Vec<yaml::Yaml> = hash.keys().map(|k| k.clone()).collect();
            let columns_as_yaml: Vec<yaml::Yaml> = table_columns.iter()
                .map(|c| yaml::Yaml::from_str(c))
                .collect();

            for key in keys.iter() {
                if !columns_as_yaml.contains(key) {
                    hash.remove(key);
                }
            }

            let mut stmt = tx.prepare(
                &format!(
                    r#"
                        INSERT INTO "{}"
                            ({})
                        VALUES
                            ({});
                    "#,
                    table_name,

                    // Wrap column names in quotes.
                    hash.keys()
                        .map(|k| k.as_str())
                        .filter(|k| k.is_some())

                        // Always `Some`.
                        .map(|k| format!(r#""{}""#, k.unwrap()))
                        .collect::<Vec<String>>()
                        .join(", "),
                    format!("{}?", "?, ".repeat(hash.len() - 1)),
                )
            )?;

            let values = hash.values().map(|v| Yaml(Cow::Borrowed(v)));
            stmt.insert(rusqlite::params_from_iter(values))?;
        }
        _ => {}
    }

    Ok(())
}
