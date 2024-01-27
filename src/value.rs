use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl Value {
    pub fn new_integer(serial_type: u64, window: &mut &[u8]) -> Self {
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
        Self::Integer(i64::from_be_bytes(arr))
    }

    pub fn new_real(window: &mut &[u8]) -> Self {
        let float: [u8; 8] = window[..8].try_into().unwrap();
        *window = &window[8..];
        Value::Real(f64::from_be_bytes(float))
    }

    pub fn new_text_or_blob(serial_type: u64, window: &mut &[u8]) -> Self {
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
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Null => String::from("null"),
            Self::Integer(i) => i.to_string(),
            Self::Real(r) => r.to_string(),
            Self::Text(text) => text.clone(),
            Self::Blob(blob) => format!("{:x?}", blob),
        })
    }
}
