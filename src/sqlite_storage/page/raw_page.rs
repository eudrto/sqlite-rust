use std::fmt::Debug;

use crate::sqlite_storage::cell::{
    IndexInteriorCell, IndexLeafCell, TableInteriorCell, TableLeafCell,
};

use super::page_header::PageHeader;

pub struct RawPage {
    pub page_header: PageHeader,
    pub cell_ptr_arr: Vec<u16>,
    pub bytes: Vec<u8>,
}

impl RawPage {
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

    pub fn get_cells<'a, T: Cell<'a>>(&'a self) -> impl Iterator<Item = T> + 'a {
        self.cell_ptr_arr.iter().map(|cell_ptr| {
            let bytes = &self.bytes[*cell_ptr as usize..];
            T::parse(bytes)
        })
    }
}

impl Debug for RawPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("header", &self.page_header)
            .field("cell_ptr_arr", &self.cell_ptr_arr)
            .field("bytes_len", &self.bytes.len())
            .finish()
    }
}

pub trait Cell<'a> {
    fn parse(bytes: &'a [u8]) -> Self;
}

impl<'a> Cell<'a> for TableLeafCell<'a> {
    fn parse(bytes: &'a [u8]) -> Self {
        Self::parse(bytes)
    }
}

impl Cell<'_> for TableInteriorCell {
    fn parse(bytes: &[u8]) -> Self {
        Self::parse(bytes)
    }
}

impl<'a> Cell<'a> for IndexLeafCell<'a> {
    fn parse(bytes: &'a [u8]) -> Self {
        Self::parse(bytes)
    }
}

impl<'a> Cell<'a> for IndexInteriorCell<'a> {
    fn parse(bytes: &'a [u8]) -> Self {
        Self::parse(bytes)
    }
}
