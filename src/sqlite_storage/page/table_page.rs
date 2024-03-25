use crate::{
    engine::Record,
    sqlite_storage::cell::{TableInteriorCell, TableLeafCell},
};

use super::raw_page::RawPage;

#[derive(Debug)]
pub struct TableLeafPage {
    raw_page: RawPage,
}

impl TableLeafPage {
    pub fn new(raw_page: RawPage) -> Self {
        Self { raw_page }
    }

    pub fn get_records(&self, rowids: Option<&[i64]>) -> Vec<Record> {
        let cells = self.raw_page.get_cells::<TableLeafCell>();

        match rowids {
            Some(rowids) => {
                let cells: Vec<_> = cells.collect();
                let mut cells: &[_] = &cells;

                rowids
                    .iter()
                    .map(|rowid| {
                        let idx = cells
                            .binary_search_by_key(rowid, |cell| cell.rowid)
                            .unwrap();

                        let record = cells[idx].parse_record();
                        cells = &cells[idx..];
                        record
                    })
                    .collect()
            }
            None => cells.map(|cell| cell.parse_record()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct TableInteriorPage {
    raw_page: RawPage,
}

impl TableInteriorPage {
    pub fn new(raw_page: RawPage) -> Self {
        Self { raw_page }
    }

    pub fn get_buckets<'a>(&self, rowids: Option<&'a [i64]>) -> Vec<(u32, Option<&'a [i64]>)> {
        let cells = self.raw_page.get_cells::<TableInteriorCell>();
        let right_most_ptr = self.raw_page.page_header.right_most_ptr.unwrap();

        match rowids {
            Some(rowids) => {
                let cells: Vec<_> = cells.collect();
                let buckets = create_buckets(&cells, right_most_ptr, &rowids);
                buckets
                    .into_iter()
                    .map(|(page_no, rowids)| (page_no, Some(rowids)))
                    .collect()
            }
            None => {
                let none: Option<&[i64]> = None;
                cells
                    .map(|cell| cell.left_child_ptr)
                    .chain([right_most_ptr])
                    .zip([none].into_iter().cycle())
                    .collect()
            }
        }
    }
}

fn create_buckets<'a>(
    cells: &[TableInteriorCell],
    right_most_ptr: u32,
    rowids: &'a [i64],
) -> Vec<(u32, &'a [i64])> {
    let keys: Vec<_> = cells.iter().map(|cell| cell.key).collect();
    let groups = create_bins(&keys, &rowids);

    cells
        .iter()
        .map(|cell| cell.left_child_ptr)
        .chain([right_most_ptr])
        .zip(groups)
        .filter(|(_, bin)| !bin.is_empty())
        .collect()
}

fn create_bins<'a, T>(ranges: &[T], points: &'a [T]) -> Vec<&'a [T]>
where
    T: PartialOrd,
{
    let mut bins = vec![];

    let mut idx = 0;
    for end in ranges {
        let start_idx = idx;
        while idx < points.len() && points[idx] <= *end {
            idx += 1;
        }

        bins.push(&points[start_idx..idx]);
    }

    bins.push(&points[idx..]);

    bins
}

#[cfg(test)]
mod tests {
    use super::create_bins;

    #[test]
    fn test_create_buckets() {
        // ]  ,  2]
        // ] 2,  5]
        // ] 5,  6]
        // ] 6,  8]
        // ] 8, 10]
        // ]10,   ]
        let ranges = [2, 5, 6, 8, 10];
        let points = [1, 2, 4, 5, 6, 9, 10, 12];

        let got = create_bins(&ranges, &points);
        assert_eq!(got.len(), ranges.len() + 1);

        let want: Vec<&[_]> = vec![&[1, 2], &[4, 5], &[6], &[], &[9, 10], &[12]];
        assert_eq!(got, want);
    }
}
