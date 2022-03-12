use yaml_rust::yaml;


pub struct Yaml<'a>(pub &'a yaml::Yaml);

impl<'a> rusqlite::ToSql for Yaml<'a> {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::ToSqlOutput;

        let sql_output = match self.0 {
            yaml::Yaml::Real(_) => ToSqlOutput::from(self.0.as_f64().unwrap()),
            yaml::Yaml::Integer(_) => ToSqlOutput::from(self.0.as_i64().unwrap()),
            yaml::Yaml::String(_) => ToSqlOutput::from(self.0.as_str().unwrap()),
            yaml::Yaml::Boolean(_) => ToSqlOutput::from(self.0.as_bool().unwrap()),
            yaml::Yaml::Array(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Hash(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Alias(_) => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::Null => ToSqlOutput::from(rusqlite::types::Null),
            yaml::Yaml::BadValue => ToSqlOutput::from(rusqlite::types::Null),
        };

        Ok(sql_output)
    }
}
