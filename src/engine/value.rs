use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Integer(b as i64)
    }
}

impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
        match value {
            Value::Integer(i) if *i == 0 || *i == 1 => *i == 1,
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
