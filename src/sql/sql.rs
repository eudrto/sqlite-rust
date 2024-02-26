use regex::RegexBuilder;

#[derive(Debug)]
pub struct CreateTableStmt<'a> {
    pub columns: Vec<&'a str>,
}

impl<'a> CreateTableStmt<'a> {
    pub fn parse(sql: &'a str) -> Self {
        let pattern = r"\((.*?)\)";
        let re = RegexBuilder::new(pattern)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        let caps = re.captures(sql).unwrap();
        let columns = caps.get(1).unwrap().as_str();

        let column_names = columns
            .split(",")
            .map(|part| part.trim().split_once(" ").unwrap().0)
            .collect();

        Self {
            columns: column_names,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CreateTableStmt;

    #[test]
    fn create_table_stmt() {
        let sql = "CREATE TABLE oranges
        (
                id integer primary key autoincrement,
                name text,
                description text
        )";

        let stmt = CreateTableStmt::parse(sql);

        let want_arr = ["id", "name", "description"];
        for (got, want) in stmt.columns.into_iter().zip(want_arr) {
            assert_eq!(got, want);
        }
    }
}
