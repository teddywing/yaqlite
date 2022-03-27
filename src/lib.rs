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

pub mod insert;
pub mod select;
pub mod sqlite;
pub mod yaml;


pub use insert::*;
pub use select::*;


/// Yaqlite errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SQL error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}
