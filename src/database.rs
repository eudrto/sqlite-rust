use std::fs::File;

use crate::db_header::DBHeader;
use crate::page::Page;
use crate::record::Record;
use crate::sqlite_schema::SQLiteSchema;

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

    pub fn load_sqlite_schema_table(&mut self) -> SQLiteSchema {
        let page = self.read_page(1);

        let mut records = vec![];
        for cell_ptr in page.cell_ptr_arr {
            let bytes = &page.bytes[cell_ptr as usize..];
            records.push(Record::new(bytes));
        }

        SQLiteSchema::new(records)
    }

    pub fn load_root_page(&mut self, name: &str) -> Result<Page, &str> {
        let sqlite_schema = self.load_sqlite_schema_table();
        let sqlite_object = sqlite_schema.get_sqlite_object(name);
        let Some(sqlite_object) = sqlite_object else {
            return Err("table not found");
        };

        Ok(self.read_page(sqlite_object.rootpage as u32))
    }
}
