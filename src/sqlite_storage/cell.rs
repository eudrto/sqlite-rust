use crate::{bytes::varint::parse_varint, engine::Record};

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
