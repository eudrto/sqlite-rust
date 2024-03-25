use self::{
    index_page::IndexLeafPage,
    page_header::PageType,
    raw_page::RawPage,
    table_page::{TableInteriorPage, TableLeafPage},
};

mod index_page;
mod page_header;
mod raw_page;
mod table_page;

#[derive(Debug)]
pub enum Page {
    Table(TablePage),
    Index(IndexPage),
}

impl Page {
    pub fn parse(bytes: Vec<u8>, page_no: u32) -> Self {
        let page = RawPage::parse(bytes, page_no);

        match page.page_header.page_type {
            PageType::TableLeaf => Page::Table(TablePage::Leaf(TableLeafPage::new(page))),
            PageType::TableInterior => {
                Page::Table(TablePage::Interior(TableInteriorPage::new(page)))
            }
            PageType::IndexLeaf => Page::Index(IndexPage::Leaf(IndexLeafPage::new(page))),
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
pub enum IndexPage {
    Leaf(IndexLeafPage),
}
