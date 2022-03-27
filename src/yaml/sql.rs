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

use yaml_rust::yaml;

use std::borrow::Cow;


/// Wrap `yaml_rust::Yaml`, adding implementations for `rusqlite::ToSql` and
/// `rusqlite::types::FromSql`.
#[derive(Debug)]
pub(crate) struct Yaml<'a>(pub Cow<'a, yaml::Yaml>);

impl<'a> Yaml<'a> {
    /// Extracts the wrapped `yaml_rust::Yaml`.
    pub fn into_inner(self) -> yaml::Yaml {
        self.0.into_owned()
    }
}

impl<'a> rusqlite::ToSql for Yaml<'a> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::ToSqlOutput;

        let sql_output = match *self.0 {
            yaml::Yaml::Real(_) => match self.0.as_f64() {
                Some(v) => ToSqlOutput::from(v),
                None => ToSqlOutput::from(rusqlite::types::Null),
            },
            yaml::Yaml::Integer(_) => match self.0.as_i64() {
                Some(v) => ToSqlOutput::from(v),
                None => ToSqlOutput::from(rusqlite::types::Null),
            },
            yaml::Yaml::String(_) => match self.0.as_str() {
                Some(v) => ToSqlOutput::from(v),
                None => ToSqlOutput::from(rusqlite::types::Null),
            },
            yaml::Yaml::Boolean(_) => match self.0.as_bool() {
                Some(v) => ToSqlOutput::from(v),
                None => ToSqlOutput::from(rusqlite::types::Null),
            },
            yaml::Yaml::Array(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Hash(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Alias(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Null => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::BadValue => ToSqlOutput::from(rusqlite::types::Null),
        };

        Ok(sql_output)
    }
}

impl<'a> rusqlite::types::FromSql for Yaml<'a> {
    fn column_result(
        value: rusqlite::types::ValueRef<'_>,
    ) -> rusqlite::types::FromSqlResult<Self> {
        use rusqlite::types::ValueRef;

        match value {
            ValueRef::Integer(i) => Ok(
                Yaml(Cow::Owned(yaml_rust::Yaml::Integer(i))),
            ),
            ValueRef::Real(f) =>
                Ok(Yaml(Cow::Owned(yaml_rust::Yaml::Real(f.to_string())))),
            ValueRef::Text(_) => {
                let s = value.as_str()?;

                Ok(Yaml(Cow::Owned(yaml_rust::Yaml::String(s.to_owned()))))
            }
            ValueRef::Blob(_) => {
                // TODO: How should we handle blobs? Parsing as string might not
                // make the most sense.
                let b = value.as_str()?;

                Ok(Yaml(Cow::Owned(yaml_rust::Yaml::String(b.to_owned()))))
            }
            ValueRef::Null => Ok(Yaml(Cow::Owned(yaml_rust::Yaml::Null))),
        }
    }
}
