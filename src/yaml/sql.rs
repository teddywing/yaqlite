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
