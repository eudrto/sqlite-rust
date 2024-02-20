use regex::RegexBuilder;

use crate::sql::expr_parser::parse_expr;

use super::ast::Expr;

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

#[derive(Debug)]
pub struct SelectStmt<'a> {
    pub select_clause: Vec<&'a str>,
    pub from_clause: &'a str,
    pub where_clause: Option<Expr>,
}

impl<'a> SelectStmt<'a> {
    pub fn parse(sql: &'a str) -> Self {
        fn select_clause(sql: &str) -> Vec<&str> {
            let pattern = r"SELECT(.*?)FROM";
            let re = RegexBuilder::new(pattern)
                .case_insensitive(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();

            let caps = re.captures(sql).unwrap();
            caps.get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|s| s.trim())
                .collect()
        }

        fn from_clause(sql: &str) -> &str {
            let pattern = r"FROM(.*?)(WHERE|;?$)";
            let re = RegexBuilder::new(pattern)
                .case_insensitive(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();

            let caps = re.captures(sql).unwrap();
            caps.get(1).unwrap().as_str().trim()
        }

        fn where_clause(sql: &str) -> Option<Expr> {
            let pattern = r"WHERE(.*?);?$";
            let re = RegexBuilder::new(pattern)
                .case_insensitive(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();

            re.captures(sql)
                .map(|caps| parse_expr(caps.get(1).unwrap().as_str()))
        }

        Self {
            select_clause: select_clause(sql),
            from_clause: from_clause(sql),
            where_clause: where_clause(sql),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sql::ast::{BinOp, Literal};

    use super::*;

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

    #[test]
    fn select_stmt() {
        let sql = "SELECT COUNT(*)
        FROM apples";

        let stmt = SelectStmt::parse(sql);

        assert_eq!(stmt.select_clause[0], "COUNT(*)");
        assert_eq!(stmt.from_clause, "apples");
    }

    #[test]
    fn select_stmt_semi() {
        let sql = "SELECT COUNT(*)
        FROM apples;";

        let stmt = SelectStmt::parse(sql);

        assert_eq!(stmt.select_clause[0], "COUNT(*)");
        assert_eq!(stmt.from_clause, "apples");
    }

    #[test]
    fn select_stmt_multi() {
        let sql = "SELECT name, color
        FROM apples";

        let stmt = SelectStmt::parse(sql);

        assert_eq!(stmt.select_clause[0], "name");
        assert_eq!(stmt.select_clause[1], "color");
        assert_eq!(stmt.from_clause, "apples");
    }

    #[test]
    fn select_stmt_where() {
        let sql = "SELECT name, color FROM apples WHERE color = 'Yellow'";

        let stmt = SelectStmt::parse(sql);

        assert_eq!(stmt.select_clause[0], "name");
        assert_eq!(stmt.select_clause[1], "color");
        assert_eq!(stmt.from_clause, "apples");

        let where_want = Expr::Binary(
            BinOp::Eq,
            Box::new(Expr::Literal(Literal::Id(String::from("color")))),
            Box::new(Expr::Literal(Literal::Text(String::from("Yellow")))),
        );

        assert_eq!(stmt.where_clause.unwrap(), where_want);
    }
}
