use std::fs::File;

use crate::db_header::DBHeader;
use crate::page::Page;
use crate::sqlite_file::SQLiteFile;
use crate::sqlite_schema::SQLiteSchema;
use crate::table::Table;

#[derive(Debug)]
pub struct Database {
    sqlite_file: SQLiteFile,
}

impl Database {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let sqlite_file = SQLiteFile::new(file);
        Self { sqlite_file }
    }

    pub fn get_db_header(&mut self) -> DBHeader {
        DBHeader::new(self.sqlite_file.load_db_header())
    }

    pub fn read_page(&mut self, page_no: u32) -> Page {
        let page_size = self.get_db_header().page_size as usize;
        let page_bytes = self.sqlite_file.load_page(page_no, page_size);
        Page::new(page_bytes, page_no)
    }

    pub fn load_sqlite_schema_table(&mut self) -> SQLiteSchema {
        let page = self.read_page(1);
        SQLiteSchema::new(page.read_records())
    }

    pub fn load_table(&mut self, name: &str) -> Result<Table, &str> {
        let sqlite_schema = self.load_sqlite_schema_table();
        let sqlite_object = sqlite_schema.get_sqlite_object(name);
        let Some(sqlite_object) = sqlite_object else {
            return Err("table not found");
        };

        let columns = sqlite_object.get_columns();

        let root_page = self.read_page(sqlite_object.rootpage as u32);
        let records = root_page.read_records();
        Ok(Table::new(&columns, records))
    }
}
