use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

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

impl Value {
    pub fn or(&self, rhs: &Value) -> Value {
        Value::from(self.into() || rhs.into())
    }
    pub fn and(&self, rhs: &Value) -> Value {
        Value::from(self.into() && rhs.into())
    }
}

impl<'a, 'b> Add<&'b Value> for &'a Value {
    type Output = Value;
    fn add(self, rhs: &'b Value) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(x), Value::Integer(y)) => Value::Integer(x + y),
            (Value::Real(x), Value::Real(y)) => Value::Real(x + y),
            (Value::Text(x), Value::Text(y)) => Value::Text(x.to_owned() + y),
            _ => panic!(),
        }
    }
}

impl<'a, 'b> Sub<&'b Value> for &'a Value {
    type Output = Value;
    fn sub(self, rhs: &'b Value) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(x), Value::Integer(y)) => Value::Integer(x - y),
            (Value::Real(x), Value::Real(y)) => Value::Real(x - y),
            _ => panic!(),
        }
    }
}

impl<'a, 'b> Mul<&'b Value> for &'a Value {
    type Output = Value;
    fn mul(self, rhs: &'b Value) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(x), Value::Integer(y)) => Value::Integer(x * y),
            (Value::Real(x), Value::Real(y)) => Value::Real(x * y),
            _ => panic!(),
        }
    }
}

impl<'a, 'b> Div<&'b Value> for &'a Value {
    type Output = Value;
    fn div(self, rhs: &'b Value) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(x), Value::Integer(y)) => Value::Integer(x * y),
            (Value::Real(x), Value::Real(y)) => Value::Real(x * y),
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
