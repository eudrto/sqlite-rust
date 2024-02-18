use std::convert::From;

use crate::{
    record::Record,
    sqlite_object::{SQLiteObject, SQLiteObjectType},
    value::Value,
};

impl From<Record> for SQLiteObject {
    fn from(record: Record) -> Self {
        let mut records_it = record.values.into_iter();

        let object_type = match records_it.next().unwrap() {
            Value::Text(object_type) => match object_type.as_ref() {
                "table" => SQLiteObjectType::Table,
                "index" => SQLiteObjectType::Index,
                _ => unimplemented!(),
            },
            _ => panic!(),
        };

        let Value::Text(name) = records_it.next().unwrap() else {
            panic!()
        };
        let Value::Text(tbl_name) = records_it.next().unwrap() else {
            panic!()
        };
        let Value::Integer(rootpage) = records_it.next().unwrap() else {
            panic!()
        };
        let Value::Text(sql) = records_it.next().unwrap() else {
            panic!()
        };

        SQLiteObject {
            object_type,
            name,
            tbl_name,
            rootpage: rootpage as u32,
            sql,
        }
    }
}
