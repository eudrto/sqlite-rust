use std::fs::File;

use crate::sql::SelectStmt;
use crate::sqlite_file::SQLiteFile;
use crate::sqlite_schema::SQLiteSchema;
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
        println!(
            "database page size: {}",
            self.storage.get_db_header().page_size
        );
        println!(
            "number of tables: {}",
            self.storage.get_page(1).page_header.cell_cnt
        );
    }

    fn exec_tables(&mut self) {
        let tables = self.load_sqlite_schema_table().dot_tables();
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

    fn load_sqlite_schema_table(&mut self) -> SQLiteSchema {
        let page = self.storage.get_page(1);
        SQLiteSchema::new(page.read_records())
    }

    fn load_table(&mut self, name: &str) -> Result<Table, &str> {
        let sqlite_schema = self.load_sqlite_schema_table();
        let sqlite_object = sqlite_schema.get_sqlite_object(name);
        let Some(sqlite_object) = sqlite_object else {
            return Err("table not found");
        };

        let columns = sqlite_object.get_columns();

        let root_page = self.storage.get_page(sqlite_object.rootpage as u32);
        let records = root_page.read_records();
        Ok(Table::new(&columns, records))
    }
}
