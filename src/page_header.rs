use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct PageHeader {
    pub page_type: u8,
    pub freeblock_start: u16,
    pub cell_cnt: u16,
    pub cell_content_area_start: u16,
    pub fragmented_free_bytes_cnt: u8,
    pub right_most_ptr: Option<u32>,
}

impl PageHeader {
    pub fn new(file: &mut File) -> Self {
        let mut header = [0; 8];
        file.read_exact(&mut header).expect("read header");

        let mut page_header = Self {
            page_type: Self::parse_page_type(&header),
            freeblock_start: Self::parse_freeblock_start(&header),
            cell_cnt: Self::parse_cell_cnt(&header),
            cell_content_area_start: Self::parse_cell_content_area_start(&header),
            fragmented_free_bytes_cnt: Self::parse_fragmented_free_bytes_cnt(&header),
            right_most_ptr: None,
        };

        if page_header.is_interior() {
            let mut ptr = [0; 4];
            file.read_exact(&mut ptr).expect("read header");
            page_header.right_most_ptr = Some(u32::from_be_bytes(ptr));
        }

        page_header
    }

    fn parse_page_type(header: &[u8; 8]) -> u8 {
        const OFFSET: usize = 0;
        const SIZE: usize = 1;

        let s = &header[OFFSET..OFFSET + SIZE];
        let arr: [u8; SIZE] = s.try_into().expect("convert slice to array");
        u8::from_be_bytes(arr)
    }

    fn parse_freeblock_start(header: &[u8; 8]) -> u16 {
        const OFFSET: usize = 1;
        const SIZE: usize = 2;

        let s = &header[OFFSET..OFFSET + SIZE];
        let arr: [u8; SIZE] = s.try_into().expect("convert slice to array");
        u16::from_be_bytes(arr)
    }

    fn parse_cell_cnt(header: &[u8; 8]) -> u16 {
        const OFFSET: usize = 3;
        const SIZE: usize = 2;

        let s = &header[OFFSET..OFFSET + SIZE];
        let arr: [u8; SIZE] = s.try_into().expect("convert slice to array");
        u16::from_be_bytes(arr)
    }

    fn parse_cell_content_area_start(header: &[u8; 8]) -> u16 {
        const OFFSET: usize = 5;
        const SIZE: usize = 2;

        let s = &header[OFFSET..OFFSET + SIZE];
        let arr: [u8; SIZE] = s.try_into().expect("convert slice to array");
        u16::from_be_bytes(arr)
    }

    fn parse_fragmented_free_bytes_cnt(header: &[u8; 8]) -> u8 {
        const OFFSET: usize = 7;
        const SIZE: usize = 1;

        let s = &header[OFFSET..OFFSET + SIZE];
        let arr: [u8; SIZE] = s.try_into().expect("convert slice to array");
        u8::from_be_bytes(arr)
    }

    fn is_interior(&self) -> bool {
        self.page_type == 2 || self.page_type == 5
    }
}
