use crate::sql::parse_create_index_stmt;

use super::SQLiteObject;

pub struct SQLiteSchema {
    pub sqlite_objects: Vec<SQLiteObject>,
}

impl SQLiteSchema {
    pub fn new(sqlite_objects: Vec<SQLiteObject>) -> Self {
        Self { sqlite_objects }
    }

    pub fn get_sqlite_object(&self, name: &str) -> Option<&SQLiteObject> {
        self.sqlite_objects
            .iter()
            .find(|sqlite_object| sqlite_object.name == name)
    }

    pub fn find_index(&self, table_name: &str, indexed_column: &str) -> Option<&SQLiteObject> {
        self.sqlite_objects
            .iter()
            .filter(|sqlite_object| sqlite_object.is_index())
            .find(|sqlite_object| {
                let stmt = parse_create_index_stmt(&sqlite_object.sql);

                if stmt.table_name == table_name
                    && stmt.indexed_columns.len() == 1
                    && stmt.indexed_columns[0] == indexed_column
                {
                    true
                } else {
                    false
                }
            })
    }

    pub fn get_table_names(&self) -> impl Iterator<Item = &str> {
        self.sqlite_objects
            .iter()
            .filter(|sqlite_object| sqlite_object.is_table())
            .map(|sqlite_object| &sqlite_object.name[..])
    }
}
