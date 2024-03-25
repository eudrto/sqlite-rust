use crate::sql::sql::{ColumnDef, CreateTableStmt};

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
    pub fn new(
        object_type: SQLiteObjectType,
        name: &str,
        tbl_name: &str,
        rootpage: u32,
        sql: &str,
    ) -> Self {
        Self {
            object_type,
            name: name.into(),
            tbl_name: tbl_name.into(),
            rootpage,
            sql: sql.into(),
        }
    }

    pub fn is_table(&self) -> bool {
        return matches!(self.object_type, SQLiteObjectType::Table);
    }

    pub fn is_index(&self) -> bool {
        return matches!(self.object_type, SQLiteObjectType::Index);
    }

    pub fn get_column_defs(&self) -> Vec<ColumnDef> {
        let stmt = CreateTableStmt::parse(&self.sql);
        stmt.column_defs
    }

    pub fn get_column_names(&self) -> Vec<&str> {
        self.get_column_defs()
            .into_iter()
            .map(|column_def| column_def.column_name)
            .collect()
    }
}
