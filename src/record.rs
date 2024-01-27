use nom::Offset;

use crate::value::Value;
use crate::varint::{parse_varint, parse_varints};

#[derive(Debug)]
pub struct Record {
    pub rowid: u64,
    pub columns: Vec<Value>,
}

impl Record {
    pub fn new(mut bytes: &[u8]) -> Self {
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
                0..=6 => Value::new_integer(*serial_type, window),
                7 => Value::new_real(window),
                8 => Value::Integer(0),
                9 => Value::Integer(1),
                10 | 11 => panic!(),
                _ => Value::new_text_or_blob(*serial_type, window),
            });
        }

        let offset = payload_start.offset(window);
        assert_eq!(offset as u64, payload_size);

        Self {
            rowid,
            columns: values,
        }
    }

    pub fn select(&self, positions: &[usize]) -> Self {
        Self {
            rowid: self.rowid,
            columns: positions
                .iter()
                .map(|position| self.columns[*position].clone())
                .collect(),
        }
    }
}
