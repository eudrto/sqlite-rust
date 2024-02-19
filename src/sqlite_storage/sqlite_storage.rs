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

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use crate::sqlite_file::SQLiteFile;

    use super::SQLiteStorage;

    fn construct_sqlite_storage(db_file_rel_path: &str) -> SQLiteStorage {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = File::open(root.join(db_file_rel_path)).unwrap();
        let sqlite_file = SQLiteFile::new(file);
        SQLiteStorage::new(sqlite_file)
    }

    #[test]
    fn get_dbinfo() {
        let mut sqlite_storage = construct_sqlite_storage("sample.db");
        let dbinfo = sqlite_storage.get_dbinfo();

        assert_eq!(dbinfo.page_size, 4096);
        assert_eq!(dbinfo.table_cnt, 3);
    }

    #[test]
    fn get_schema() {
        let mut sqlite_storage = construct_sqlite_storage("sample.db");
        let sqlite_schema = sqlite_storage.get_schema();

        let apples = sqlite_schema.get_sqlite_object("apples").unwrap();
        assert_eq!(apples.get_col_names(), vec!["id", "name", "color"]);

        let oranges = sqlite_schema.get_sqlite_object("oranges").unwrap();
        assert_eq!(oranges.get_col_names(), vec!["id", "name", "description"]);
    }

    #[test]
    fn get_tables_ok() {
        let mut sqlite_storage = construct_sqlite_storage("sample.db");

        let apples = sqlite_storage.get_table("apples").unwrap();
        assert_eq!(apples.len(), 4);
        for record in &apples {
            assert_eq!(record.values.len(), 3);
        }

        let oranges = sqlite_storage.get_table("oranges").unwrap();
        assert_eq!(oranges.len(), 6);
        for record in &oranges {
            assert_eq!(record.values.len(), 3);
        }
    }

    #[test]
    fn get_tables_err() {
        let mut sqlite_storage = construct_sqlite_storage("sample.db");
        assert!(matches!(sqlite_storage.get_table("grapes"), Err(_)));
    }
}
