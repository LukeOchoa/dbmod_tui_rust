mod files;

pub use files::handle_files::*;

mod err_tools {
    #[derive(Debug)]
    pub struct ErrorX {
        details: String,
    }

    impl ErrorX {
        pub fn _new(msg: &str) -> ErrorX {
            ErrorX {
                details: msg.to_string(),
            }
        }
        pub fn new_box(msg: &str) -> Box<ErrorX> {
            Box::new(ErrorX {
                details: msg.to_string(),
            })
        }
    }

    impl std::fmt::Display for ErrorX {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl std::error::Error for ErrorX {
        fn description(&self) -> &str {
            &self.details
        }
    }
}

mod file_tools {
    // use crate::err_tools::ErrorX;
    // use std::error::Error;

    // pub fn file_to_string(path: &str) -> Result<String, std::io::Error> {
    //     let result = std::fs::read_to_string(path);

    //     result
    // }

    // pub fn load_file_as_string(relative_path: &str) -> Result<String, String> {
    //     match file_to_string(relative_path) {
    //         Ok(some_string) => return Ok(some_string),
    //         Err(error) => return Err(error.to_string()),
    //     }
    // }

    // pub fn current_dir_string() -> Result<String, Box<dyn Error>> {
    //     match std::env::current_dir() {
    //         Ok(current_dir) => option_str_to_string(current_dir.to_str()),
    //         Err(_) => Err(ErrorX::new_box("Could not obtain the current directory")),
    //     }
    // }

    // fn option_str_to_string(
    //     option_str: Option<&str>,
    // ) -> Result<String, Box<dyn std::error::Error>> {
    //     match option_str {
    //         Some(a_str) => return Ok(a_str.to_string()),
    //         None => {
    //             return Err(ErrorX::new_box(
    //                 "Could not convert current_dir(type: PathBuf to &str",
    //             ))
    //         }
    //     }
    // }

    // pub fn file_names(directory: String, folder: String) -> Result<Vec<String>, String> {
    //     //! returns a list of filenames from the specified folder of a specific directory
    //     let path = format!("{}{}", directory, folder);
    //     let mut filenames = Vec::new();

    //     let subfn = || -> Result<Vec<String>, Box<dyn std::error::Error>> {
    //         let dir_iter = std::fs::read_dir(path)?;
    //         for maybe_entry in dir_iter {
    //             let os_filename = maybe_entry?.file_name();
    //             let filename = option_str_to_string(os_filename.to_str())?;
    //             filenames.push(filename)
    //         }
    //         Ok(filenames)
    //     };

    //     match subfn() {
    //         Ok(some_filenames) => Ok(some_filenames),
    //         Err(err) => Err(err.to_string()),
    //     }
    // }
}

pub mod db_tools {
    //file_tools::{current_dir_string, file_names},
    use crate::err_tools::ErrorX;
    use postgres::{Client, NoTls};
    use std::{collections::HashMap, error::Error};

    type MagicError = Box<dyn Error>;
    type Tables = HashMap<String, String>;
    type Rows = HashMap<String, Vec<String>>;

    pub fn db_connection() -> Result<Client, Box<dyn std::error::Error>> {
        let result = Client::connect(
            "host=localhost port=5432 dbname=breaker user=luke password=free144",
            NoTls,
        )?;

        Ok(result)
    }

    pub fn postgresql_json_table_names(table_names: Rows) -> Rows {
        //let current_dir = current_dir_string()?;
        //let filenames = file_names(current_dir, String::from("/postgresql_json/"))?;

        table_names
    }

    fn check_if_table_exists(
        client: &mut postgres::Client,
        table: &String,
    ) -> Result<bool, String> {
        let if_table_exists = format!(
            "SELECT EXISTS ( SELECT FROM pg_tables WHERE schemaname = 'public' AND tablename = '{}');",
            table
        );

        match client.query(&if_table_exists, &[]) {
            Ok(res) => {
                let some_bool: bool = res[0].get(0);
                return Ok(some_bool);
            }
            Err(error) => return Err(error.to_string()),
        }
    }

    fn drop_table(client: &mut postgres::Client, table: &String) -> Result<(), MagicError> {
        let drop_some_table = format!("DROP TABLE {};", table);
        match client.batch_execute(&drop_some_table) {
            Ok(_) => return Ok(()),
            Err(error) => {
                // Some tables are not allowed to be dropped. (i.e. table: 'userprofile')
                // Instead of dropping them, currently i will just allow it
                if "Some(SqlState(E2BP01))" == &format!("{:?}", error.code()) {
                    return Ok(());
                }
                return Err(Box::new(error));
            }
        }
    }

    pub fn execute_rebuild(
        tables: Tables,
        maybe_rows: Option<Rows>,
        selected_table: &String,
    ) -> Result<(), MagicError> {
        let mut client = db_connection()?;

        // make sure (selected_table) has its ".json" removed before
        // you send it to (check_if_table_exists) because its not named
        // that way in the database i think lol
        if check_if_table_exists(&mut client, selected_table)? {
            drop_table(&mut client, &selected_table)?;
        }

        // Build SQL Tables
        let table_to_build = tables.get(selected_table).ok_or(ErrorX::new_box(&format!(
            "failed to get value from table: bad key:{}",
            selected_table
        )))?;
        client.batch_execute(table_to_build)?;

        // Insert Rows Into SQL Table
        if let Some(rows) = maybe_rows {
            let rows_to_insert = rows.get(selected_table).ok_or(ErrorX::new_box(&format!(
                "failed to get value from table: bad key:{}",
                selected_table
            )))?;
            for query in rows_to_insert.iter() {
                client.batch_execute(query)?;
            }
        }

        client.close().unwrap();
        Ok(())
    }
}
