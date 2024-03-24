use nom::Offset;

use super::value::{parse_integer, parse_real, parse_text_or_blob};
use crate::bytes::varint::{parse_varint, parse_varints};
use crate::engine::Value;

#[derive(Debug)]
struct RecordHeader {
    header_size: i64,
    serial_types: Vec<i64>,
}

impl RecordHeader {
    fn parse(mut bytes: &[u8]) -> Self {
        let header_start = bytes;
        let window = &mut bytes;

        let header_size = parse_varint(window);
        let bytes_read = header_start.offset(window);
        let serial_types = parse_varints(&window[..header_size as usize - bytes_read]);

        Self {
            header_size,
            serial_types,
        }
    }
}

pub fn parse_record(bytes: &[u8]) -> Vec<Value> {
    let record_header = RecordHeader::parse(bytes);

    let mut body = &bytes[record_header.header_size as usize..];
    let window = &mut body;

    record_header
        .serial_types
        .iter()
        .map(|serial_type| match serial_type {
            0 => Value::Null,
            0..=6 => parse_integer(*serial_type, window),
            7 => parse_real(window),
            8 => Value::Integer(0),
            9 => Value::Integer(1),
            10 | 11 => panic!("internal error"),
            _ => parse_text_or_blob(*serial_type, window),
        })
        .collect()
}
