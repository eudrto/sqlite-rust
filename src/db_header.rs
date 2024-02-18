use crate::bytes::from_be_bytes::from_be_bytes;

#[derive(Debug)]
pub struct DBHeader {
    pub page_size: u16,
    pub reserved_size: u8,
    pub page_cnt: u32,
    pub text_encoding: u32,
}

impl DBHeader {
    pub fn new(bytes: [u8; 100]) -> Self {
        Self {
            page_size: from_be_bytes(&mut &bytes[16..]),
            reserved_size: from_be_bytes(&mut &bytes[20..]),
            page_cnt: from_be_bytes(&mut &bytes[28..]),
            text_encoding: from_be_bytes(&mut &bytes[56..]),
        }
    }
}
