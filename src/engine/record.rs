use std::{fmt::Display, ops::Index};

use itertools::Itertools;

use super::Value;

#[derive(Debug)]
pub struct Record {
    pub rowid: i64,
    pub values: Vec<Value>,
}

impl Record {
    pub fn new(rowid: i64, values: Vec<Value>) -> Self {
        Self { rowid, values }
    }

    pub fn select(&self, positions: &[usize]) -> Self {
        Self {
            rowid: self.rowid,
            values: positions
                .iter()
                .map(|position| self.values[*position].clone())
                .collect(),
        }
    }
}

impl Index<usize> for Record {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .values
                .iter()
                .map(|value| format!("{}", value))
                .join("|"),
        )
    }
}
