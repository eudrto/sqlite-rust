use std::fs::File;

use crate::sql::SelectStmt;
use crate::sqlite_file::SQLiteFile;
use crate::sqlite_storage::SQLiteStorage;
use crate::table::Table;

#[derive(Debug)]
pub struct Database {
    storage: SQLiteStorage,
}

impl Database {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let sqlite_file = SQLiteFile::new(file);
        let storage = SQLiteStorage::new(sqlite_file);
        Self { storage }
    }

    pub fn exec(&mut self, cmd: &str) {
        match cmd {
            ".dbinfo" => self.exec_dbinfo(),
            ".tables" => self.exec_tables(),
            _ => self.exec_sql(cmd),
        }
    }

    fn exec_dbinfo(&mut self) {
        let dbinfo = self.storage.get_dbinfo();
        println!("{}", dbinfo);
    }

    fn exec_tables(&mut self) {
        let tables = self.storage.get_schema().dot_tables();
        println!("{tables}");
    }

    fn exec_sql(&mut self, sql: &str) {
        let stmt = SelectStmt::parse(sql);

        let table = match self.load_table(&stmt.from) {
            Ok(table) => table,
            Err(msg) => {
                println!("{}", msg);
                return;
            }
        };

        if stmt.select.len() == 1 && stmt.select[0].to_lowercase() == "count(*)" {
            println!("{}", table.size())
        } else {
            println!("{}", table.select(&stmt.select));
        }
    }

    fn load_table(&mut self, name: &str) -> Result<Table, String> {
        let records = self.storage.get_table(name)?;

        let sqlite_schema = self.storage.get_schema();
        let sqlite_object = sqlite_schema.get_sqlite_object(name).unwrap();
        let columns = sqlite_object.get_columns();
        Ok(Table::new(&columns, records))
    }
}
