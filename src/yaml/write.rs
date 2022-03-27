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

/// Adapt a `std::io::Write` type into a `std::fmt::Write`.
pub struct IoAdapter<'a, T: std::io::Write> {
    inner: &'a mut T,
}

impl<'a, T: std::io::Write> IoAdapter<'a, T> {
    /// Create a new `IoAdapter` that wraps the given `std::io::Write` type.
    pub fn new(writer: &'a mut T) -> Self {
        IoAdapter { inner: writer }
    }
}

impl<T: std::io::Write> std::fmt::Write for IoAdapter<'_, T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self.inner.write_all(s.as_bytes()) {
            Ok(()) => Ok(()),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
