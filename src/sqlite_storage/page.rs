use std::fmt::Debug;

use super::{
    page_header::{PageHeader, PageType},
    record::parse_record,
};
use crate::{bytes::from_be_bytes::from_be_bytes, engine::Record};

pub struct Page {
    pub page_header: PageHeader,
    pub cell_ptr_arr: Vec<u16>,
    pub bytes: Vec<u8>,
}

impl Page {
    pub fn parse(bytes: Vec<u8>, page_no: u32) -> Self {
        let start_offset = if page_no == 1 { 100 } else { 0 };
        let window = &mut &bytes[start_offset..];
        let page_header = PageHeader::parse(window);

        let cell_ptr_arr = window[..2 * page_header.cell_cnt as usize]
            .chunks(2)
            .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
            .collect();

        Self {
            page_header,
            cell_ptr_arr,
            bytes,
        }
    }

    pub fn get_children(&self) -> Vec<u32> {
        if !matches!(self.page_header.page_type, PageType::Interior) {
            panic!("internal error")
        }

        self.cell_ptr_arr
            .iter()
            .map(|cell_ptr| {
                let cell_ptr = *cell_ptr as usize;
                from_be_bytes(&mut &self.bytes[cell_ptr..cell_ptr + 4])
            })
            .collect()
    }

    pub fn get_records(&self) -> Vec<Record> {
        if !matches!(self.page_header.page_type, PageType::Leaf) {
            panic!("internal error")
        }

        self.cell_ptr_arr
            .iter()
            .map(|cell_ptr| {
                let bytes = &self.bytes[*cell_ptr as usize..];
                parse_record(bytes)
            })
            .collect()
    }
}

impl Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("header", &self.page_header)
            .field("cell_ptr_arr", &self.cell_ptr_arr)
            .field("bytes_len", &self.bytes.len())
            .finish()
    }
}
