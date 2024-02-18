use crate::sqlite_file::SQLiteFile;

use super::{db_header::DBHeader, page::Page};

#[derive(Debug)]
pub struct SQLiteStorage {
    sqlite_file: SQLiteFile,
}

impl SQLiteStorage {
    pub fn new(sqlite_file: SQLiteFile) -> Self {
        Self { sqlite_file }
    }

    pub fn get_db_header(&mut self) -> DBHeader {
        DBHeader::parse(self.sqlite_file.load_db_header())
    }

    pub fn get_page(&mut self, page_no: u32) -> Page {
        let page_size = self.get_db_header().page_size as usize;
        let page = self.sqlite_file.load_page(page_no, page_size);
        Page::parse(page, page_no)
    }
}
