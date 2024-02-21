use std::ops::Index;

use super::{Record, TableHeader, Value};

#[derive(Debug)]
pub struct Row<'a> {
    pub header: &'a TableHeader,
    pub record: Record,
}

impl<'a> Row<'a> {
    pub fn new(header: &'a TableHeader, record: Record) -> Self {
        Self { header, record }
    }
}

impl<'a> Index<&str> for Row<'a> {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.record[self.header[index]]
    }
}
