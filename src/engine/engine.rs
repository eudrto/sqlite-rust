use crate::sql::{parse_stmt, Expr};

use super::{DBInfo, Record, Row, Storage, Table, TableHeader, Value};

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
        let stmt = parse_stmt(sql);
        let table = self.load_table(&stmt.from_clause, stmt.where_clause)?;

        if stmt.select_clause.len() == 0 {
            // Workaround:
            // Empty stmt.select_clause represents
            // SELECT COUNT(*) FROM ...

            Ok(Table::new(
                TableHeader::new(&[]),
                vec![Record::new(0, vec![Value::Integer(table.size() as i64)])],
            ))
        } else {
            let mut columns: Vec<_> = stmt
                .select_clause
                .into_iter()
                .map(|expr| expr.eval_select(&table))
                .collect();

            let mut records = vec![];
            'outer: loop {
                let mut values = vec![];

                for column in &mut columns {
                    let value = column.next();
                    if let Some(value) = value {
                        values.push(value);
                    } else {
                        break 'outer;
                    }
                }

                records.push(Record::new(0, values));
            }

            Ok(Table::new(TableHeader::new(&[]), records))
        }
    }

    fn load_table(&mut self, name: &str, where_expr: Option<Expr>) -> Result<Table, String> {
        let mut records = self.storage.get_table(name)?;

        let sqlite_schema = self.storage.get_schema();
        let sqlite_object = sqlite_schema.get_sqlite_object(name).unwrap();
        let column_defs = sqlite_object.get_column_defs();

        // Handle NULL value in INTEGER PRIMARY KEY (rowid) column:
        // "When an SQL table includes an INTEGER PRIMARY KEY column (which aliases the rowid)
        // then that column appears in the record as a NULL value.
        // SQLite will always use the table b-tree key rather than the NULL value
        // when referencing the INTEGER PRIMARY KEY column."
        // https://www.sqlite.org/fileformat.html#representation_of_sql_tables
        let rowid_column = column_defs.iter().position(|column_def| {
            column_def
                .type_name_and_column_constraint
                .to_lowercase()
                .contains("integer primary key")
        });

        if let Some(rowid_colum) = rowid_column {
            for record in &mut records {
                assert_eq!(record.values[rowid_colum], Value::Null);
                record.values[rowid_colum] = Value::Integer(record.rowid);
            }
        }

        let table_header = TableHeader::new(&sqlite_object.get_column_names());

        if let Some(where_expr) = where_expr {
            records = records
                .into_iter()
                .map(|record| Row::new(&table_header, record))
                .filter(|row| bool::from(&where_expr.eval_where(row)))
                .map(|row| row.record)
                .collect()
        }

        Ok(Table::new(table_header, records))
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
    fn exec_select_expr_1() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT 'name: ' + name + ', color: ' + color FROM apples";

        let table = engine.exec_sql(sql).unwrap();

        let want = [
            ("name: Granny Smith, color: Light Green",),
            ("name: Fuji, color: Red",),
            ("name: Honeycrisp, color: Blush Red",),
            ("name: Golden Delicious, color: Yellow",),
        ];
        assert_eq!(table.size(), want.len());

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 1);
            assert_eq!(record.values[0].to_string(), want.0);
        }
    }

    #[test]
    fn exec_select_expr_2() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT name, 1 FROM apples";

        let table = engine.exec_sql(sql).unwrap();

        let want = [
            ("Granny Smith", 1),
            ("Fuji", 1),
            ("Honeycrisp", 1),
            ("Golden Delicious", 1),
        ];
        assert_eq!(table.size(), want.len());

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 2);
            assert_eq!(record.values[0].to_string(), want.0);
            assert_eq!(record.values[1].to_string(), want.1.to_string());
        }
    }

    #[test]
    #[ignore = "endless loop"]
    fn exec_select_expr_3() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("sample.db").to_str().unwrap());
        let sql = "SELECT 1, 2 FROM apples";

        let table = engine.exec_sql(sql).unwrap();

        let want = [(1, 2)];
        assert_eq!(table.size(), want.len());

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 2);
            assert_eq!(record.values[0].to_string(), want.0.to_string());
            assert_eq!(record.values[1].to_string(), want.1.to_string());
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

    #[test]
    fn exec_select_with_where_pass_4() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut engine = new_engine(root.join("superheroes.db").to_str().unwrap());
        let sql = "SELECT id, name FROM superheroes WHERE eye_color = 'Pink Eyes'";

        let table = engine.exec_sql(sql).unwrap();

        let want = [
            (297, "Stealth (New Earth)"),
            (790, "Tobias Whale (New Earth)"),
            (1085, "Felicity (New Earth)"),
            (2729, "Thrust (New Earth)"),
            (3289, "Angora Lapin (New Earth)"),
            (3913, "Matris Ater Clementia (New Earth)"),
        ];
        assert_eq!(table.size(), want.len());

        for (record, want) in table.records.into_iter().zip(want) {
            assert_eq!(record.values.len(), 2);
            assert_eq!(record.values[0].to_string(), want.0.to_string());
            assert_eq!(record.values[1].to_string(), want.1);
        }
    }
}
