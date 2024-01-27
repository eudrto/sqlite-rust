use crate::record::Record;

pub struct Table {
    col_names: Vec<String>,
    records: Vec<Record>,
}

impl Table {
    pub fn new(columns: &[&str], records: Vec<Record>) -> Self {
        Self {
            col_names: columns.iter().map(|column| String::from(*column)).collect(),
            records,
        }
    }

    pub fn select(self, columns: &[&str]) -> Self {
        let positions = self.col_names_to_col_positions(columns);

        Self {
            col_names: columns.iter().map(|column| String::from(*column)).collect(),
            records: self
                .records
                .into_iter()
                .map(|record| record.select(&positions))
                .collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.records.len()
    }

    fn col_names_to_col_positions(&self, columns: &[&str]) -> Vec<usize> {
        columns
            .iter()
            .map(|column| self.col_name_to_col_position(column))
            // TODO Missing columns: hard error vs best effott
            .filter_map(|name| name)
            .collect()
    }

    fn col_name_to_col_position(&self, column: &str) -> Option<usize> {
        self.col_names
            .iter()
            .position(|col_name| col_name == column)
    }
}
