use yaml_rust::yaml;


pub struct Yaml<'a>(pub &'a yaml::Yaml);

impl<'a> rusqlite::ToSql for Yaml<'a> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::ToSqlOutput;

        let sql_output = match self.0 {
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
            ValueRef::Integer(i) => Ok(Yaml(&yaml_rust::Yaml::Integer(i))),
            ValueRef::Real(f) =>
                Ok(Yaml(&yaml_rust::Yaml::Real(f.to_string()))),
            ValueRef::Text(_) => {
                let s = value.as_str()?;

                Ok(Yaml(&yaml_rust::Yaml::String(s.to_owned())))
            }
            ValueRef::Blob(_) => {
                // TODO: How should we handle blobs? Parsing as string might not
                // make the most sense.
                let b = value.as_str()?;

                Ok(Yaml(&yaml_rust::Yaml::String(b.to_owned())))
            }
            ValueRef::Null => Ok(Yaml(&yaml_rust::Yaml::Null)),
        }
    }
}
