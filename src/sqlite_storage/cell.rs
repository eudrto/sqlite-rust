use crate::{
    bytes::{from_be_bytes::from_be_bytes, varint::parse_varint},
    engine::{Record, Value},
};

use super::record::parse_record;

#[derive(Debug)]
pub struct TableLeafCell<'a> {
    pub rowid: i64,
    payload: &'a [u8],
}

impl<'a> TableLeafCell<'a> {
    pub fn parse(mut bytes: &'a [u8]) -> Self {
        let window = &mut bytes;

        let _payload_size = parse_varint(window);
        let rowid = parse_varint(window) as i64;

        Self {
            rowid,
            payload: window,
        }
    }

    pub fn parse_record(&self) -> Record {
        let values = parse_record(self.payload);
        Record::new(self.rowid, values)
    }
}

#[derive(Debug)]
pub struct TableInteriorCell {
    pub left_child_ptr: u32,
    pub key: i64,
}

impl TableInteriorCell {
    pub fn parse(bytes: &[u8]) -> Self {
        let left_child_ptr = from_be_bytes(&mut &bytes[..4]);
        let key = parse_varint(&mut &bytes[4..]);

        Self {
            left_child_ptr,
            key,
        }
    }
}

#[derive(Debug)]
pub struct IndexLeafCell<'a> {
    payload: &'a [u8],
}

impl<'a> IndexLeafCell<'a> {
    pub fn parse(mut bytes: &'a [u8]) -> Self {
        let window = &mut bytes;

        let _payload_size = parse_varint(window);

        Self { payload: window }
    }

    pub fn parse_record(&self) -> Vec<Value> {
        parse_record(self.payload)
    }
}
