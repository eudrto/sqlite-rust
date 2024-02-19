use nom::Offset;

use super::value::{parse_integer, parse_real, parse_text_or_blob};
use crate::bytes::varint::{parse_varint, parse_varints};
use crate::engine::{Record, Value};

pub fn parse_record(mut bytes: &[u8]) -> Record {
    let window = &mut bytes;

    // Table B-Tree Leaf Cell
    let payload_size = parse_varint(window);
    let rowid = parse_varint(window);

    let payload_start = *window;

    // Header
    let header_size = parse_varint(window);
    let bytes_read = payload_start.offset(window);
    let serial_types = parse_varints(&window[..header_size as usize - bytes_read]);

    // Values
    let window = &mut &payload_start[header_size as usize..];
    let mut values = vec![];
    for serial_type in &serial_types {
        values.push(match serial_type {
            0 => Value::Null,
            0..=6 => parse_integer(*serial_type, window),
            7 => parse_real(window),
            8 => Value::Integer(0),
            9 => Value::Integer(1),
            10 | 11 => panic!(),
            _ => parse_text_or_blob(*serial_type, window),
        });
    }

    let offset = payload_start.offset(window);
    debug_assert_eq!(offset as u64, payload_size);

    Record::new(rowid, values)
}
