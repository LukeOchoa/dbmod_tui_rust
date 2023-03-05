use std::{collections::HashMap, error::Error};

type MagicError = Box<dyn Error>;

pub fn test_load_files() -> Result<(String, String), MagicError> {
    let tables = std::fs::read_to_string("postgresql_json/breaker_db_tables.json")?;
    let rows = std::fs::read_to_string("postgresql_json/breaker_db_table_values.json")?;

    //Ok((String::from(""), String::from("")))
    Ok((tables, rows))
}

type Tables = HashMap<String, String>;
type Rows = HashMap<String, Vec<String>>;

// ) -> Result<(HashMap<String, String>, HashMap<String, Vec<String>>), MagicError> {

pub fn make_tables_and_rows(tables: String, rows: String) -> Result<(Tables, Rows), MagicError> {
    let tables_map = serde_json::from_str(&tables)?;
    let rows_map_array = serde_json::from_str(&rows)?;

    Ok((tables_map, rows_map_array))
}
