use super::{db_header::DBHeader, page::Page};
use crate::engine::{DBInfo, Record, SQLiteSchema};
use crate::sqlite_file::SQLiteFile;

#[derive(Debug)]
pub struct SQLiteStorage {
    sqlite_file: SQLiteFile,
}

impl SQLiteStorage {
    pub fn new(sqlite_file: SQLiteFile) -> Self {
        Self { sqlite_file }
    }

    fn get_db_header(&mut self) -> DBHeader {
        DBHeader::parse(self.sqlite_file.load_db_header())
    }

    fn get_page(&mut self, page_no: u32) -> Page {
        let page_size = self.get_db_header().page_size as usize;
        let page = self.sqlite_file.load_page(page_no, page_size);
        Page::parse(page, page_no)
    }

    pub fn get_dbinfo(&mut self) -> DBInfo {
        let page_size = self.get_db_header().page_size;
        let sqlite_schema = self.get_schema();
        let table_cnt = sqlite_schema.sqlite_objects.len();
        DBInfo::new(page_size, table_cnt as u16)
    }

    pub fn get_schema(&mut self) -> SQLiteSchema {
        let records = self.get_page(1).get_records();
        let sqlite_objects = records.into_iter().map(|r| r.into()).collect();
        SQLiteSchema::new(sqlite_objects)
    }

    pub fn get_table(&mut self, name: &str) -> Result<Vec<Record>, String> {
        if let Some(sqlite_object) = self.get_schema().get_sqlite_object(name) {
            Ok(self.get_page(sqlite_object.rootpage).get_records())
        } else {
            Err(format!("table '{}' not found", name))
        }
    }
}
