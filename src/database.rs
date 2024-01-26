use std::fs::File;

use crate::db_header::DBHeader;
use crate::page::Page;

#[derive(Debug)]
pub struct Database {
    pub file: File,
    pub header: DBHeader,
}

impl Database {
    pub fn new(file_path: &str) -> Self {
        let mut file = File::open(file_path).unwrap();
        let header = DBHeader::new(&mut file);
        Self { file, header }
    }

    pub fn read_page(&mut self, number: u32) -> Page {
        Page::new(
            &mut self.file,
            number as usize,
            self.header.page_size as usize,
        )
    }
}
