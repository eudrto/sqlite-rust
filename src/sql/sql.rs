use regex::RegexBuilder;

#[derive(Debug)]
pub struct ColumnDef<'a> {
    pub column_name: &'a str,
    pub type_name_and_column_constraint: &'a str,
}

#[derive(Debug)]
pub struct CreateTableStmt<'a> {
    pub column_defs: Vec<ColumnDef<'a>>,
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

        let column_defs = columns
            .split(",")
            .map(|part| {
                let (prefix, suffix) = part.trim().split_once(" ").unwrap();
                ColumnDef {
                    column_name: prefix,
                    type_name_and_column_constraint: suffix,
                }
            })
            .collect();

        Self { column_defs }
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
        let column_names = stmt
            .column_defs
            .into_iter()
            .map(|column_def| column_def.column_name);

        let want_arr = ["id", "name", "description"];

        for (got, want) in column_names.zip(want_arr) {
            assert_eq!(got, want);
        }
    }
}
