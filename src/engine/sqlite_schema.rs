use super::{SQLiteObject, SQLiteObjectType};

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

    pub fn get_table_names(&self) -> impl Iterator<Item = &str> {
        self.sqlite_objects
            .iter()
            .filter(|sqlite_object| matches!(sqlite_object.object_type, SQLiteObjectType::Table))
            .map(|sqlite_object| &sqlite_object.name[..])
    }
}
