use std::fmt::Display;

#[derive(Debug)]
pub struct DBInfo {
    page_size: u16,
    table_cnt: u16,
}

impl DBInfo {
    pub fn new(page_size: u16, table_cnt: u16) -> Self {
        Self {
            page_size,
            table_cnt,
        }
    }
}

impl Display for DBInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "database page size: {}\nnumber of tables: {}",
            self.page_size, self.table_cnt
        ))
    }
}
