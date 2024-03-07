use std::fmt::Debug;

use crate::bytes::from_be_bytes::from_be_bytes;

#[derive(Debug)]
pub enum PageType {
    Interior,
    Leaf,
}

impl PageType {
    fn new(flag: u8) -> PageType {
        match flag {
            5 => PageType::Interior,
            13 => PageType::Leaf,
            _ => panic!("internal error: page type: {}", flag),
        }
    }
}

#[derive(Debug)]
pub struct PageHeader {
    pub page_type: PageType,
    pub freeblock_start: u16,
    pub cell_cnt: u16,
    pub cell_content_area_start: u16,
    pub fragmented_free_bytes_cnt: u8,
    pub right_most_ptr: Option<u32>,
}

impl PageHeader {
    pub fn parse(window: &mut &[u8]) -> Self {
        let mut page_header = Self {
            page_type: PageType::new(from_be_bytes(window)),
            freeblock_start: from_be_bytes(window),
            cell_cnt: from_be_bytes(window),
            cell_content_area_start: from_be_bytes(window),
            fragmented_free_bytes_cnt: from_be_bytes(window),
            right_most_ptr: None,
        };

        if page_header.is_interior() {
            page_header.right_most_ptr = Some(from_be_bytes(window));
        }

        page_header
    }

    fn is_interior(&self) -> bool {
        matches!(self.page_type, PageType::Interior)
    }
}
