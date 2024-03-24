use std::fmt::Debug;

use super::{
    cell::{TableInteriorCell, TableLeafCell},
    page_header::{PageHeader, PageType},
};
use crate::engine::Record;

#[derive(Debug)]
pub enum Page {
    Table(TablePage),
}

impl Page {
    pub fn parse(bytes: Vec<u8>, page_no: u32) -> Self {
        let page = RawPage::parse(bytes, page_no);

        match page.page_header.page_type {
            PageType::TableLeaf => Page::Table(TablePage::Leaf(TableLeafPage::new(page))),
            PageType::TableInterior => {
                Page::Table(TablePage::Interior(TableInteriorPage::new(page)))
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum TablePage {
    Leaf(TableLeafPage),
    Interior(TableInteriorPage),
}

#[derive(Debug)]
pub struct TableLeafPage {
    raw_page: RawPage,
}

impl TableLeafPage {
    fn new(raw_page: RawPage) -> Self {
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
    fn new(raw_page: RawPage) -> Self {
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

struct RawPage {
    pub page_header: PageHeader,
    pub cell_ptr_arr: Vec<u16>,
    pub bytes: Vec<u8>,
}

impl RawPage {
    fn parse(bytes: Vec<u8>, page_no: u32) -> Self {
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

    fn get_cells<'a, T: Cell<'a>>(&'a self) -> impl Iterator<Item = T> + 'a {
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

trait Cell<'a> {
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
