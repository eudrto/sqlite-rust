#![allow(unused)]

use super::db_header::DBHeader;
use super::page::{IndexPage, Page, TablePage};
use crate::engine::{DBInfo, Record, SQLiteSchema, Storage, Value};
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
        let bytes = self.sqlite_file.load_page(page_no, page_size);
        Page::parse(bytes, page_no)
    }
}

impl Storage for SQLiteStorage {
    fn get_dbinfo(&mut self) -> DBInfo {
        let page_size = self.get_db_header().page_size;
        let sqlite_schema = self.get_schema();
        let table_cnt = sqlite_schema.sqlite_objects.len();
        DBInfo::new(page_size, table_cnt as u16)
    }

    fn get_schema(&mut self) -> SQLiteSchema {
        let records = self.search_table(1, None);
        let sqlite_objects = records.into_iter().map(|r| r.into()).collect();
        SQLiteSchema::new(sqlite_objects)
    }

    fn search_table(&mut self, page_no: u32, rowids: Option<&[i64]>) -> Vec<Record> {
        let Page::Table(page) = self.get_page(page_no) else {
            panic!("internal error");
        };

        match page {
            TablePage::Leaf(page) => page.get_records(rowids),
            TablePage::Interior(page) => page
                .get_buckets(rowids)
                .into_iter()
                .map(|(ptr, rowids)| self.search_table(ptr, rowids))
                .flatten()
                .collect(),
        }
    }

    fn search_index(&mut self, page_no: u32, value: &Value) -> Vec<i64> {
        let Page::Index(page) = self.get_page(page_no) else {
            panic!("internal error");
        };

        match page {
            IndexPage::Leaf(page) => page.get_rowids(value),
            IndexPage::Interior(page) => {
                let (ptrs, rowids) = page.get_children(value);

                let mut results: Vec<i64> = vec![];
                for i in 0..rowids.len() {
                    results.extend(self.search_index(ptrs[i], value));
                    results.push(rowids[i])
                }
                results.extend(self.search_index(*ptrs.last().unwrap(), value));
                results
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf};

    use itertools::Itertools;

    use crate::{
        engine::{Storage, Value},
        sqlite_file::SQLiteFile,
    };

    use super::SQLiteStorage;

    fn construct_sqlite_storage(db_file_rel_path: &str) -> SQLiteStorage {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = File::open(root.join(db_file_rel_path)).unwrap();
        let sqlite_file = SQLiteFile::new(file);
        SQLiteStorage::new(sqlite_file)
    }

    fn get_rootpage(sqlite_storage: &mut SQLiteStorage, table_name: &str) -> u32 {
        let sqlite_schema = sqlite_storage.get_schema();
        let sqlite_object = sqlite_schema.get_sqlite_object(table_name).unwrap();
        sqlite_object.rootpage
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
        assert_eq!(apples.get_column_names(), vec!["id", "name", "color"]);

        let oranges = sqlite_schema.get_sqlite_object("oranges").unwrap();
        assert_eq!(
            oranges.get_column_names(),
            vec!["id", "name", "description"]
        );
    }

    #[test]
    fn get_tables_sample_ok() {
        let mut sqlite_storage = construct_sqlite_storage("sample.db");

        let apples_rootpage = get_rootpage(&mut sqlite_storage, "apples");
        let apples = sqlite_storage.search_table(apples_rootpage, None);
        assert_eq!(apples.len(), 4);
        for record in &apples {
            assert_eq!(record.values.len(), 3);
        }

        let oranges_rootpage = get_rootpage(&mut sqlite_storage, "oranges");
        let oranges = sqlite_storage.search_table(oranges_rootpage, None);
        assert_eq!(oranges.len(), 6);
        for record in &oranges {
            assert_eq!(record.values.len(), 3);
        }
    }

    #[test]
    fn get_tables_superheroes_ok() {
        let mut sqlite_storage = construct_sqlite_storage("superheroes.db");

        let rootpage = get_rootpage(&mut sqlite_storage, "superheroes");
        let superheroes = sqlite_storage.search_table(rootpage, None);
        assert_eq!(superheroes.len(), 6895);
    }

    #[test]
    fn search_index_mountains() {
        let mut sqlite_storage = construct_sqlite_storage("dbs/mountains.db");

        let rootpage = get_rootpage(&mut sqlite_storage, "idx_mountains_country");
        let value = Value::Text("France".to_string());
        let rowids = sqlite_storage.search_index(rootpage, &value);
        assert_eq!(rowids.len(), 2);
    }

    #[test]
    fn search_index_companies() {
        let mut sqlite_storage = construct_sqlite_storage("companies.db");

        let rootpage = get_rootpage(&mut sqlite_storage, "idx_companies_country");
        let value = Value::Text("myanmar".to_string());
        let rowids = sqlite_storage.search_index(rootpage, &value);

        let want_len = 799;
        assert_eq!(rowids.len(), want_len);

        let sorted_rowids: Vec<_> = rowids.iter().sorted().cloned().collect();
        assert_eq!(rowids, sorted_rowids);

        let unique_rowids: Vec<_> = rowids.iter().unique().collect();
        assert_eq!(unique_rowids.len(), want_len);
    }

    #[test]
    fn search_table_mountains() {
        let mut sqlite_storage = construct_sqlite_storage("dbs/mountains.db");

        let index_rootpage = get_rootpage(&mut sqlite_storage, "idx_mountains_country");
        let value = Value::Text("France".to_string());
        let mut rowids = sqlite_storage.search_index(index_rootpage, &value);

        let table_rootpage = get_rootpage(&mut sqlite_storage, "mountains");
        let records = sqlite_storage.search_table(2, Some(&rowids));

        assert_eq!(records.len(), 2);
        for record in &records {
            assert_eq!(record.values[3].to_string(), "France");
        }
    }

    #[test]
    fn search_table_companies_myanmar() {
        let mut sqlite_storage = construct_sqlite_storage("companies.db");

        let index_rootpage = get_rootpage(&mut sqlite_storage, "idx_companies_country");
        let value = Value::Text("myanmar".to_string());
        let mut rowids = sqlite_storage.search_index(index_rootpage, &value);

        let table_rootpage = get_rootpage(&mut sqlite_storage, "companies");
        let records = sqlite_storage.search_table(table_rootpage, Some(&rowids));

        assert_eq!(records.len(), 799);
        for record in &records {
            assert_eq!(record.values[7].to_string(), "myanmar");
        }
    }

    #[test]
    fn search_table_companies_eritrea() {
        let mut sqlite_storage = construct_sqlite_storage("companies.db");

        let index_rootpage = get_rootpage(&mut sqlite_storage, "idx_companies_country");
        let value = Value::Text("eritrea".to_string());
        let mut rowids = sqlite_storage.search_index(index_rootpage, &value);

        let table_rootpage = get_rootpage(&mut sqlite_storage, "companies");
        let records = sqlite_storage.search_table(table_rootpage, Some(&rowids));

        assert_eq!(records.len(), 4);
        for record in &records {
            assert_eq!(record.values[7].to_string(), "eritrea");
        }
    }
}
