use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub struct SQLiteFile {
    file: File,
}

impl SQLiteFile {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    pub fn load_db_header(&mut self) -> [u8; 100] {
        self.file.seek(SeekFrom::Start(0)).unwrap();
        let mut header = [0; 100];
        self.file.read_exact(&mut header).unwrap();
        header
    }

    pub fn load_page(&mut self, page_no: u32, page_size: usize) -> Vec<u8> {
        let page_no = page_no as usize;
        let start = (page_no - 1) * page_size;

        self.file.seek(SeekFrom::Start(start as u64)).unwrap();
        let mut page = vec![0; page_size];
        self.file.read_exact(&mut page).unwrap();

        page
    }
}
