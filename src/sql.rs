use regex::RegexBuilder;

pub struct SelectStmt<'a> {
    pub select: &'a str,
    pub from: &'a str,
}

impl<'a> SelectStmt<'a> {
    pub fn parse(sql: &'a str) -> Self {
        fn select(sql: &str) -> &str {
            let pattern = r"SELECT(.*?)FROM";
            let re = RegexBuilder::new(pattern)
                .case_insensitive(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();

            let caps = re.captures(sql).unwrap();
            caps.get(1).unwrap().as_str().trim()
        }

        fn from(sql: &str) -> &str {
            let pattern = r"FROM(.*?);?$";
            let re = RegexBuilder::new(pattern)
                .case_insensitive(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();

            let caps = re.captures(sql).unwrap();
            caps.get(1).unwrap().as_str().trim()
        }

        Self {
            select: select(sql),
            from: from(sql),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_stmt() {
        let sql = "SELECT COUNT(*)
        FROM apples";

        let stmt = SelectStmt::parse(sql);

        let select = "COUNT(*)";
        let from = "apples";

        assert_eq!(stmt.select, select);
        assert_eq!(stmt.from, from);
    }

    #[test]
    fn select_stmt_semi() {
        let sql = "SELECT COUNT(*)
        FROM apples;";

        let stmt = SelectStmt::parse(sql);

        let select = "COUNT(*)";
        let from = "apples";

        assert_eq!(stmt.select, select);
        assert_eq!(stmt.from, from);
    }
}
