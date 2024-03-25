use std::cmp::Ordering;

use crate::{
    engine::Value,
    sqlite_storage::cell::{IndexInteriorCell, IndexLeafCell},
};

use super::raw_page::RawPage;

#[derive(Debug)]
pub struct IndexLeafPage {
    raw_page: RawPage,
}

impl IndexLeafPage {
    pub fn new(raw_page: RawPage) -> Self {
        Self { raw_page }
    }

    pub fn get_rowids(&self, value: &Value) -> Vec<i64> {
        let cells = self.raw_page.get_cells::<IndexLeafCell>();
        let values: Vec<_> = cells.map(|cell| cell.parse_record()).collect();

        let keys: Vec<_> = values.iter().map(|value| &value[0]).collect();
        let indices = binary_search_range(&keys, &value);

        let mut records = vec![];
        if let Ok((start, end)) = indices {
            records = values
                .into_iter()
                .skip(start)
                .take(end - start)
                .map(|values| i64::from(&values[1]))
                .collect()
        };
        records
    }
}

#[derive(Debug)]
pub struct IndexInteriorPage {
    raw_page: RawPage,
}

impl IndexInteriorPage {
    pub fn new(raw_page: RawPage) -> Self {
        Self { raw_page }
    }

    pub fn get_children(&self, value: &Value) -> (Vec<u32>, Vec<i64>) {
        let cells: Vec<_> = self.raw_page.get_cells::<IndexInteriorCell>().collect();
        let values: Vec<_> = cells.iter().map(|cell| cell.parse_record()).collect();

        let keys: Vec<_> = values.iter().map(|value| &value[0]).collect();
        let indices = binary_search_range(&keys, &value);

        let mut ptrs = cells.iter().map(|cell| cell.left_child_ptr).chain([self
            .raw_page
            .page_header
            .right_most_ptr
            .unwrap()]);

        match indices {
            Ok((start, end)) => {
                let ptrs = ptrs.skip(start).take(end - start + 1).collect();

                let rowids = values
                    .into_iter()
                    .skip(start)
                    .take(end - start)
                    .map(|values| i64::from(&values[1]))
                    .collect();

                (ptrs, rowids)
            }
            Err(idx) => (vec![ptrs.nth(idx).unwrap()], vec![]),
        }
    }
}

/// Makes a `PartialOrd` type act as if it implemented `Ord`
/// by treating values that are not comparable as equal.
fn cmp<T: PartialOrd>(elem: &T, needle: &T) -> Ordering {
    if let Some(ordering) = elem.partial_cmp(needle) {
        ordering
    } else {
        Ordering::Equal
    }
}

fn binary_search_range<T: PartialOrd>(haystack: &[T], needle: &T) -> Result<(usize, usize), usize> {
    let result = haystack.binary_search_by(|elem| cmp(elem, needle));
    match result {
        Ok(idx) => {
            let mut left = idx;
            loop {
                if left == 0 || haystack[left - 1] != *needle {
                    break;
                }
                left -= 1;
            }

            let mut right = idx;
            while right < haystack.len() && haystack[right] == *needle {
                right += 1;
            }

            Ok((left, right))
        }
        Err(idx) => Err(idx),
    }
}

#[cfg(test)]
mod tests {
    use super::binary_search_range;

    #[test]
    fn binary_search_range_1() {
        let haystack = ["b", "d", "d", "e"];
        let needle = "a";

        let got = binary_search_range(&haystack, &needle);
        assert_eq!(got, Err(0));
    }

    #[test]
    fn binary_search_range_2() {
        let haystack = ["b", "d", "d", "e"];
        let needle = "b";

        let got = binary_search_range(&haystack, &needle);
        assert_eq!(got, Ok((0, 1)));
    }

    #[test]
    fn binary_search_range_3() {
        let haystack = ["b", "d", "d", "e"];
        let needle = "c";

        let got = binary_search_range(&haystack, &&needle);
        assert_eq!(got, Err(1));
    }

    #[test]
    fn binary_search_range_4() {
        let haystack = ["b", "d", "d", "e"];
        let needle = "d";

        let got = binary_search_range(&haystack, &needle);
        assert_eq!(got, Ok((1, 3)));
    }

    #[test]
    fn binary_search_range_5() {
        let haystack = ["b", "d", "d", "e"];
        let needle = "e";

        let got = binary_search_range(&haystack, &needle);
        assert_eq!(got, Ok((3, 4)))
    }

    #[test]
    fn binary_search_range_6() {
        let haystack = ["b", "d", "d", "e"];
        let needle = "f";

        let got = binary_search_range(&haystack, &needle);
        assert_eq!(got, Err(4));
    }
}
