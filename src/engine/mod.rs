mod dbinfo;
mod engine;
mod record;
mod sqlite_object;
mod sqlite_schema;
mod table;
mod value;

use std::fs::File;

pub use dbinfo::DBInfo;
pub use engine::Engine;
pub use record::Record;
pub use sqlite_object::{SQLiteObject, SQLiteObjectType};
pub use sqlite_schema::SQLiteSchema;
pub use table::Table;
pub use value::Value;

use crate::{sqlite_file::SQLiteFile, sqlite_storage::SQLiteStorage};

pub trait Storage {
    fn get_dbinfo(&mut self) -> DBInfo;
    fn get_schema(&mut self) -> SQLiteSchema;
    fn get_table(&mut self, name: &str) -> Result<Vec<Record>, String>;
}

pub fn new_engine(file_path: &str) -> Engine<SQLiteStorage> {
    let file = File::open(file_path).unwrap();
    let sqlite_file = SQLiteFile::new(file);
    let storage = SQLiteStorage::new(sqlite_file);
    Engine::new(storage)
}
