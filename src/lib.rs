pub mod insert;
pub mod select;
pub mod sqlite;
pub mod yaml;


pub use insert::*;
pub use select::*;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SQL error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}
