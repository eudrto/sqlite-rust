use std::fs::File;

use super::{DBInfo, Table, Value};
use crate::engine::Record;
use crate::sql::SelectStmt;
use crate::sqlite_file::SQLiteFile;
use crate::sqlite_storage::SQLiteStorage;

#[derive(Debug)]
pub struct Engine {
    storage: SQLiteStorage,
}

impl Engine {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let sqlite_file = SQLiteFile::new(file);
        let storage = SQLiteStorage::new(sqlite_file);
        Self { storage }
    }

    pub fn exec(&mut self, cmd: &str) {
        match cmd {
            ".dbinfo" => {
                println!("{}", self.exec_dbinfo());
            }
            ".tables" => {
                let tables = self
                    .exec_tables()
                    .into_iter()
                    .fold(String::new(), |acc, e| acc + " " + &e);
                println!("{tables}");
            }
            _ => match self.exec_sql(cmd) {
                Ok(table) => println!("{}", table),
                Err(msg) => println!("{}", msg),
            },
        }
    }

    fn exec_dbinfo(&mut self) -> DBInfo {
        self.storage.get_dbinfo()
    }

    fn exec_tables(&mut self) -> Vec<String> {
        let sqlite_schema = self.storage.get_schema();
        sqlite_schema
            .get_table_names()
            .map(|name| name.to_owned())
            .collect()
    }

    fn exec_sql(&mut self, sql: &str) -> Result<Table, String> {
        let stmt = SelectStmt::parse(sql);
        let table = self.load_table(&stmt.from)?;

        if stmt.select.len() == 1 && stmt.select[0].to_lowercase() == "count(*)" {
            Ok(Table::new(
                &[],
                vec![Record::new(0, vec![Value::Integer(table.size() as i64)])],
            ))
        } else {
            Ok(table.select(&stmt.select))
        }
    }

    fn load_table(&mut self, name: &str) -> Result<Table, String> {
        let records = self.storage.get_table(name)?;

        let sqlite_schema = self.storage.get_schema();
        let sqlite_object = sqlite_schema.get_sqlite_object(name).unwrap();
        let column_names = sqlite_object.get_col_names();
        Ok(Table::new(&column_names, records))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Engine;

    #[test]
    fn exec_select() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = Engine::new(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT name, color FROM apples";

        let table = engine.exec_sql(sql).unwrap();
        assert_eq!(table.size(), 4);

        let want = vec![
            ("Granny Smith", "Light Green"),
            ("Fuji", "Red"),
            ("Honeycrisp", "Blush Red"),
            ("Golden Delicious", "Yellow"),
        ];

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 2);
            assert_eq!(record.values[0].to_string(), want.0);
            assert_eq!(record.values[1].to_string(), want.1);
        }
    }

    #[test]
    fn exec_select_count() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = Engine::new(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT COUNT(*) FROM apples";

        let table = engine.exec_sql(sql).unwrap();
        assert_eq!(table.to_string(), "4");
    }
}
