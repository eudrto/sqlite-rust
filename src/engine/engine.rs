use super::{DBInfo, Row, Storage, Table, TableHeader, Value};
use crate::engine::Record;
use crate::sql::{Expr, SelectStmt};

#[derive(Debug)]
pub struct Engine<S: Storage> {
    storage: S,
}

impl<S: Storage> Engine<S> {
    pub fn new(storage: S) -> Self {
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

    pub fn exec_dbinfo(&mut self) -> DBInfo {
        self.storage.get_dbinfo()
    }

    pub fn exec_tables(&mut self) -> Vec<String> {
        let sqlite_schema = self.storage.get_schema();
        sqlite_schema
            .get_table_names()
            .map(|name| name.to_owned())
            .collect()
    }

    pub fn exec_sql(&mut self, sql: &str) -> Result<Table, String> {
        let stmt = SelectStmt::parse(sql);
        let table = self.load_table(&stmt.from_clause, stmt.where_clause)?;

        if stmt.select_clause.len() == 1 && stmt.select_clause[0].to_lowercase() == "count(*)" {
            Ok(Table::new(
                &[],
                vec![Record::new(0, vec![Value::Integer(table.size() as i64)])],
            ))
        } else {
            Ok(table.select(&stmt.select_clause))
        }
    }

    fn load_table(&mut self, name: &str, where_expr: Option<Expr>) -> Result<Table, String> {
        let mut records = self.storage.get_table(name)?;

        let sqlite_schema = self.storage.get_schema();
        let sqlite_object = sqlite_schema.get_sqlite_object(name).unwrap();
        let column_names = sqlite_object.get_col_names();

        let table_header = TableHeader::new(&column_names);

        if let Some(where_expr) = where_expr {
            records = records
                .into_iter()
                .map(|record| Row::new(&table_header, record))
                .filter(|row| bool::from(&where_expr.eval(row)))
                .map(|row| row.record)
                .collect()
        }

        Ok(Table::new(&column_names, records))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::engine::new_engine;

    #[test]
    fn exec_select() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT name, color FROM apples";

        let table = engine.exec_sql(sql).unwrap();

        let want = [
            ("Granny Smith", "Light Green"),
            ("Fuji", "Red"),
            ("Honeycrisp", "Blush Red"),
            ("Golden Delicious", "Yellow"),
        ];
        assert_eq!(table.size(), want.len());

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 2);
            assert_eq!(record.values[0].to_string(), want.0);
            assert_eq!(record.values[1].to_string(), want.1);
        }
    }

    #[test]
    fn exec_select_count() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT COUNT(*) FROM apples";

        let table = engine.exec_sql(sql).unwrap();
        assert_eq!(table.to_string(), "4");
    }

    #[test]
    fn exec_select_with_where_pass_1() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT name, color FROM apples WHERE color = 'Yellow'";

        let table = engine.exec_sql(sql).unwrap();

        let want = [("Golden Delicious", "Yellow")];
        assert_eq!(table.size(), want.len());

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 2);
            assert_eq!(record.values[0].to_string(), want.0);
            assert_eq!(record.values[1].to_string(), want.1);
        }
    }

    #[test]
    fn exec_select_with_where_pass_2() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT name, color FROM apples WHERE color == 'Yellow' OR color = 'Red'";

        let table = engine.exec_sql(sql).unwrap();

        let want = [("Fuji", "Red"), ("Golden Delicious", "Yellow")];
        assert_eq!(table.size(), want.len());

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 2);
            assert_eq!(record.values[0].to_string(), want.0);
            assert_eq!(record.values[1].to_string(), want.1);
        }
    }

    #[test]
    fn exec_select_with_where_pass_3() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT name, color FROM apples WHERE color = 'Ye' + 'll' + 'ow'";

        let table = engine.exec_sql(sql).unwrap();

        let want = [("Golden Delicious", "Yellow")];
        assert_eq!(table.size(), want.len());

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 2);
            assert_eq!(record.values[0].to_string(), want.0);
            assert_eq!(record.values[1].to_string(), want.1);
        }
    }

    #[test]
    #[should_panic]
    fn exec_select_with_where_fail_1() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT name FROM apples WHERE col == 'val'";

        engine.exec_sql(sql).unwrap();
    }
}
