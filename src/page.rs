use std::{
    fmt::Debug,
    fs::File,
    io::{Read, Seek},
};

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
    pub fn new(file: &mut File, number: usize, page_size: usize) -> Self {
        let page_start = (number - 1) * page_size;
        file.seek(std::io::SeekFrom::Start(page_start as u64))
            .unwrap();

        let mut bytes = vec![0; page_size];
        file.read_exact(&mut bytes).unwrap();

        let window = &mut if number == 1 { &bytes[100..] } else { &bytes };
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
