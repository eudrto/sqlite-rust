use sqlite_starter_rust::engine::{
    DBInfo, Engine, Record, SQLiteObject, SQLiteObjectType, SQLiteSchema, Storage, Value,
};

struct MockStorage;

impl Storage for MockStorage {
    fn get_dbinfo(&mut self) -> DBInfo {
        unimplemented!()
    }

    fn get_schema(&mut self) -> SQLiteSchema {
        let object_type = SQLiteObjectType::Table;
        let name = "books";
        let tbl_name = "books";
        let rootpage = 0;
        let sql = "CREATE TABLE unknown
        (
                id integer primary key autoincrement,
                title text,
                author text,
                genre text,
                year_published integer
        )";

        SQLiteSchema::new(vec![SQLiteObject::new(
            object_type,
            name,
            tbl_name,
            rootpage,
            sql,
        )])
    }

    fn search_table(&mut self, _rootpage: u32, _rowids: Option<&[i64]>) -> Vec<Record> {
        let table = [
            ("To Kill a Mockingbird", "Harper Lee", "Fiction", 1960),
            ("1984", "George Orwell", "Dystopian", 1949),
            ("The Great Gatsby", "F. Scott Fitzgerald", "Classic", 1925),
            ("Pride and Prejudice", "Jane Austen", "Romance", 1813),
        ];

        table
            .into_iter()
            .map(|row| {
                Record::new(
                    0,
                    vec![
                        Value::Null,
                        Value::Text(row.0.into()),
                        Value::Text(row.1.into()),
                        Value::Text(row.2.into()),
                        Value::Integer(row.3),
                    ],
                )
            })
            .collect()
    }
}

#[test]
fn exec_select() {
    let mut engine = Engine::new(MockStorage);
    let sql = "SELECT title, author, year_published FROM books";

    let table = engine.exec_sql(sql).unwrap();
    assert_eq!(table.size(), 4);

    let want = [
        ("To Kill a Mockingbird", "Harper Lee", "1960"),
        ("1984", "George Orwell", "1949"),
        ("The Great Gatsby", "F. Scott Fitzgerald", "1925"),
        ("Pride and Prejudice", "Jane Austen", "1813"),
    ];

    for (record, want) in table.records.into_iter().zip(want) {
        assert_eq!(record.values.len(), 3);
        assert_eq!(record.values[0].to_string(), want.0);
        assert_eq!(record.values[1].to_string(), want.1);
        assert_eq!(record.values[2].to_string(), want.2);
    }
}

#[test]
fn exec_select_count() {
    let mut engine = Engine::new(MockStorage);
    let sql = "SELECT COUNT(*) FROM books";

    let table = engine.exec_sql(sql).unwrap();
    assert_eq!(table.to_string(), "4");
}

#[test]
fn exec_select_with_where_1() {
    let mut engine = Engine::new(MockStorage);
    let sql = "SELECT title, author, year_published FROM books WHERE genre = 'Dystopian'";

    let table = engine.exec_sql(sql).unwrap();

    let want = [("1984", "George Orwell", "1949")];
    assert_eq!(table.size(), want.len());

    for (record, want) in table.records.into_iter().zip(want) {
        assert_eq!(record.values.len(), 3);
        assert_eq!(record.values[0].to_string(), want.0);
        assert_eq!(record.values[1].to_string(), want.1);
    }
}

#[test]
fn exec_select_with_where_2() {
    let mut engine = Engine::new(MockStorage);
    let sql = "SELECT title, author, year_published FROM books WHERE author = 'Harper Lee' OR author = 'Jane Austen'";

    let table = engine.exec_sql(sql).unwrap();

    let want = [
        ("To Kill a Mockingbird", "Harper Lee", "1960"),
        ("Pride and Prejudice", "Jane Austen", "1813"),
    ];
    assert_eq!(table.size(), want.len());

    for (record, want) in table.records.into_iter().zip(want) {
        assert_eq!(record.values.len(), 3);
        assert_eq!(record.values[0].to_string(), want.0);
        assert_eq!(record.values[1].to_string(), want.1);
    }
}
