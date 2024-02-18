use std::{fs::File, io::Read};

use crate::bytes::from_be_bytes::from_be_bytes;

#[derive(Debug)]
pub struct DBHeader {
    pub page_size: u16,
    pub reserved_size: u8,
    pub page_cnt: u32,
    pub text_encoding: u32,
}

impl DBHeader {
    pub fn new(file: &mut File) -> Self {
        let mut header = [0; 100];
        file.read_exact(&mut header).unwrap();

        let db_header = Self {
            page_size: from_be_bytes(&mut &header[16..]),
            reserved_size: from_be_bytes(&mut &header[20..]),
            page_cnt: from_be_bytes(&mut &header[28..]),
            text_encoding: from_be_bytes(&mut &header[56..]),
        };

        db_header
    }
}
