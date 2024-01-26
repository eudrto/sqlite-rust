use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct DBHeader {
    pub page_size: u16,
    pub page_cnt: u32,
}

impl DBHeader {
    pub fn new(file: &mut File) -> Self {
        let mut header = [0; 100];
        file.read_exact(&mut header).expect("read header");

        let db_header = Self {
            page_size: Self::parse_page_size(&header),
            page_cnt: Self::parse_page_cnt(&header),
        };

        db_header
    }

    fn parse_page_size(header: &[u8; 100]) -> u16 {
        const OFFSET: usize = 16;
        const SIZE: usize = 2;

        let s = &header[OFFSET..OFFSET + SIZE];
        let arr: [u8; SIZE] = s.try_into().expect("convert slice to array");
        u16::from_be_bytes(arr)
    }

    fn parse_page_cnt(header: &[u8; 100]) -> u32 {
        const OFFSET: usize = 28;
        const SIZE: usize = 4;

        let s = &header[OFFSET..OFFSET + SIZE];
        let arr: [u8; SIZE] = s.try_into().expect("convert slice to array");
        u32::from_be_bytes(arr)
    }
}
