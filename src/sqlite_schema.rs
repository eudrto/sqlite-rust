use crate::record::Record;
use crate::value::Value;

#[derive(Debug)]
pub enum SQLiteObjectType {
    Table,
    Index,
}

#[derive(Debug)]
pub struct SQLiteObject {
    pub object_type: SQLiteObjectType,
    pub name: String,
    pub tbl_name: String,
    pub rootpage: u64,
    pub sql: String,
}

pub struct SQLiteSchema {
    pub sqlite_objects: Vec<SQLiteObject>,
}

impl SQLiteSchema {
    pub fn new(records: Vec<Record>) -> Self {
        let sqlite_objects = records
            .into_iter()
            .map(|record| {
                let mut records_it = record.columns.into_iter();

                let object_type = match records_it.next().unwrap() {
                    Value::Text(object_type) => match object_type.as_ref() {
                        "table" => SQLiteObjectType::Table,
                        "index" => SQLiteObjectType::Index,
                        _ => unimplemented!(),
                    },
                    _ => panic!(),
                };

                let name = match records_it.next().unwrap() {
                    Value::Text(name) => name,
                    _ => panic!(),
                };

                let tbl_name = match records_it.next().unwrap() {
                    Value::Text(name) => name,
                    _ => panic!(),
                };

                let rootpage = match records_it.next().unwrap() {
                    Value::Integer(rootpage) => rootpage as u64,
                    _ => panic!(),
                };

                let sql = match records_it.next().unwrap() {
                    Value::Text(sql) => sql,
                    _ => panic!(),
                };

                SQLiteObject {
                    object_type,
                    name,
                    tbl_name,
                    rootpage,
                    sql,
                }
            })
            .collect();

        Self { sqlite_objects }
    }

    pub fn get_sqlite_object(&self, name: &str) -> Option<&SQLiteObject> {
        self.sqlite_objects
            .iter()
            .find(|sqlite_object| sqlite_object.name == name)
    }

    pub fn dot_tables(&self) -> String {
        self.sqlite_objects
            .iter()
            .filter(|sqlite_object| matches!(sqlite_object.object_type, SQLiteObjectType::Table))
            .map(|sqlite_object| &sqlite_object.name)
            .fold(String::new(), |acc, e| acc + " " + &e)
    }
}
