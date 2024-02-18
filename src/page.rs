use std::fmt::Debug;

use crate::{page_header::PageHeader, record::Record};

fn read_cell_ptr_arr(bytes: &[u8], cell_cnt: usize) -> Vec<u16> {
    bytes[..2 * cell_cnt]
        .chunks(2)
        .map(|chunk| u16::from_be_bytes(chunk.try_into().unwrap()))
        .collect()
}

pub struct Page {
    pub header: PageHeader,
    pub cell_ptr_arr: Vec<u16>,
    pub bytes: Vec<u8>,
}

impl Page {
    pub fn new(bytes: Vec<u8>, page_no: u32) -> Self {
        let start_offset = if page_no == 1 { 100 } else { 0 };
        let window = &mut &bytes[start_offset..];
        let header = PageHeader::new(window);

        let cell_ptr_arr = read_cell_ptr_arr(window, header.cell_cnt as usize);

        Self {
            header,
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
            .field("header", &self.header)
            .field("cell_ptr_arr", &self.cell_ptr_arr)
            .field("bytes_len", &self.bytes.len())
            .finish()
    }
}
