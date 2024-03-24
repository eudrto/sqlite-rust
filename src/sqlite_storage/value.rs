use crate::engine::Value;

pub fn parse_integer(serial_type: i64, window: &mut &[u8]) -> Value {
    let size = match serial_type {
        5 => 6,
        6 => 8,
        _ => serial_type as usize,
    };

    let mut arr = [0; 8];
    for (i, byte) in window[..size].iter().enumerate() {
        arr[arr.len() - 1 - i] = *byte;
    }

    *window = &window[size..];
    Value::Integer(i64::from_be_bytes(arr))
}

pub fn parse_real(window: &mut &[u8]) -> Value {
    let float: [u8; 8] = window[..8].try_into().unwrap();
    *window = &window[8..];
    Value::Real(f64::from_be_bytes(float))
}

pub fn parse_text_or_blob(serial_type: i64, window: &mut &[u8]) -> Value {
    let subtrahend = if serial_type % 2 == 0 { 12 } else { 13 };
    let length = (serial_type - subtrahend) / 2;
    let value = &window[..length as usize];
    *window = &window[length as usize..];

    match subtrahend {
        12 => Value::Blob(Vec::from(value)),
        // TODO Use the `text_encoding` field of `DBHeader`
        13 => Value::Text(String::from_utf8(Vec::from(value)).unwrap()),
        _ => panic!(),
    }
}
