use std::{collections::HashMap, ops::Index};

#[derive(Debug)]
pub struct TableHeader(HashMap<String, usize>);

impl TableHeader {
    pub fn new(column_names: &[&str]) -> Self {
        Self(
            column_names
                .into_iter()
                .enumerate()
                .map(|(idx, name)| (String::from(*name), idx))
                .collect(),
        )
    }
}

impl Index<&str> for TableHeader {
    type Output = usize;
    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}
