use crate::sql::CreateTableStmt;

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
    pub rootpage: u32,
    pub sql: String,
}

impl SQLiteObject {
    pub fn get_col_names(&self) -> Vec<&str> {
        let stmt = CreateTableStmt::parse(&self.sql);
        stmt.columns
    }
}
