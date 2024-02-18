use std::fmt::Debug;

use super::page_header::PageHeader;
use crate::record::Record;

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

    pub fn read_records(&self) -> Vec<Record> {
        self.cell_ptr_arr
            .iter()
            .map(|cell_ptr| {
                let bytes = &self.bytes[*cell_ptr as usize..];
                Record::new(bytes)
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
