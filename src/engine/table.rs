use std::fmt::Display;

use super::{Record, TableHeader, Value};

pub struct Table {
    table_header: TableHeader,
    pub records: Vec<Record>,
}

impl Table {
    pub fn new(table_header: TableHeader, records: Vec<Record>) -> Self {
        Self {
            table_header,
            records,
        }
    }

    pub fn get(&self, column: &str) -> impl Iterator<Item = &Value> {
        let idx = self.table_header[column];
        self.records.iter().map(move |record| &record.values[idx])
    }

    pub fn size(&self) -> usize {
        self.records.len()
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .records
                .iter()
                .map(|record| format!("{}", record))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}
