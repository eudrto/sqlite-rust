use crate::{engine::Record, sqlite_storage::cell::{TableInteriorCell, TableLeafCell}};

use super::raw_page::RawPage;

#[derive(Debug)]
pub struct TableLeafPage {
    raw_page: RawPage,
}

impl TableLeafPage {
    pub fn new(raw_page: RawPage) -> Self {
        Self { raw_page }
    }

    pub fn get_records(&self) -> Vec<Record> {
        self.raw_page
            .get_cells::<TableLeafCell>()
            .map(|cell| cell.parse_record())
            .collect()
    }
}

#[derive(Debug)]
pub struct TableInteriorPage {
    raw_page: RawPage,
}

impl TableInteriorPage {
    pub fn new(raw_page: RawPage) -> Self {
        Self { raw_page }
    }

    pub fn get_children(&self) -> Vec<u32> {
        self.raw_page
            .get_cells::<TableInteriorCell>()
            .map(|cell| cell.left_child_ptr)
            .chain([self.raw_page.page_header.right_most_ptr.unwrap()])
            .collect()
    }
}
