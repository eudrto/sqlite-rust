use super::{
    ast::SelectStmt,
    sql::CreateIndexStmt,
    {BinOp, Expr, Literal},
};

use peg::parser;

parser! {
    pub grammar parser() for str {
        // --------------------
        // lexical grammar
        // --------------------

        // case insensitive
        rule i(literal: &'static str)
            = input:$([_]*<{literal.len()}>) {? if input.eq_ignore_ascii_case(literal) { Ok(()) } else { Err(literal) } }

        // whitespace
        rule _ = [' ' | '\n' | '\t']*

        // character
        rule alpha() -> &'input str = a:$(['a'..='z' | 'A'..='Z']) { a }
        rule alpha_() -> &'input str = a:$(alpha() / "_") { a }
        rule num() -> &'input str = n:$(['0'..='9']) { n }
        rule alphanum() -> &'input str = a:$(alpha() / num()) { a }
        rule alphanum_() -> &'input str = a:$(alphanum() / "_") { a }

        // token
        rule tok_left_paren() -> &'input str = _ t:$"(" {t}
        rule tok_right_paren() -> &'input str = _ t:$")" {t}
        rule tok_comma() -> &'input str = _ t:$"," {t}
        rule tok_semi() -> &'input str = _ t:$";" {t}

        rule tok_or() -> &'input str = _ t:$"OR" {t}
        rule tok_and() -> &'input str = _ t:$"AND" {t}
        rule tok_eq() -> &'input str = _ t:$("==" / "=") {t}
        rule tok_neq() -> &'input str = _ t:$("<>" / "!=") {t}
        rule tok_lt() -> &'input str = _ t:$"<" {t}
        rule tok_lte() -> &'input str = _ t:$"<=" {t}
        rule tok_gt() -> &'input str = _ t:$">" {t}
        rule tok_gte() -> &'input str = _ t:$">=" {t}
        rule tok_add() -> &'input str = _ t:$"+" {t}
        rule tok_sub() -> &'input str = _ t:$"-" {t}
        rule tok_star() -> &'input str = _ t:$"*" {t}
        rule tok_div() -> &'input str = _ t:$"/" {t}

        rule tok_integer() -> Literal
            = _ i:$(num()+) { Literal::Integer(i.parse().unwrap()) }

        rule tok_string() -> Literal
            = _ "'" s:$([^ '\'']*) "'" { Literal::Text(s.into()) }

        rule tok_id() -> Literal
            = _ i:$(alpha_() alphanum_()*) { Literal::Id(i.into()) }

        // keyword
        rule kw_create() = _ i("create")
        rule kw_from() = _ i("from")
        rule kw_index() = _ i("index")
        rule kw_on() = _ i("on")
        rule kw_select() = _ i("select")
        rule kw_table() = _ i("table")
        rule kw_unique() = _ i("unique")
        rule kw_where() = _ i("where")

        // --------------------
        // syntacitc grammar
        // --------------------

        pub rule expr() -> Expr = precedence!{
            l:(@) tok_or()  r:@ { Expr::Binary(BinOp::Or, Box::new(l), Box::new(r))}
            --
            l:(@) tok_and() r:@ { Expr::Binary(BinOp::And, Box::new(l), Box::new(r))}
            --
            l:(@) tok_eq()  r:@ { Expr::Binary(BinOp::Eq, Box::new(l), Box::new(r))}
            l:(@) tok_neq() r:@ { Expr::Binary(BinOp::Neq, Box::new(l), Box::new(r))}
            --
            l:(@) tok_lt()  r:@ { Expr::Binary(BinOp::Lt, Box::new(l), Box::new(r))}
            l:(@) tok_lte()  r:@ { Expr::Binary(BinOp::Lte, Box::new(l), Box::new(r))}
            l:(@) tok_gt()  r:@ { Expr::Binary(BinOp::Gt, Box::new(l), Box::new(r))}
            l:(@) tok_gte()  r:@ { Expr::Binary(BinOp::Gte, Box::new(l), Box::new(r))}
            --
            l:(@) tok_add() r:@ { Expr::Binary(BinOp::Add, Box::new(l), Box::new(r))}
            l:(@) tok_sub() r:@ { Expr::Binary(BinOp::Sub, Box::new(l), Box::new(r))}
            --
            l:(@) tok_star() r:@ { Expr::Binary(BinOp::Mul, Box::new(l), Box::new(r))}
            l:(@) tok_div() r:@ { Expr::Binary(BinOp::Div, Box::new(l), Box::new(r))}
            --
            i:tok_integer() { Expr::Literal(i)  }
            s:tok_string() { Expr::Literal(s) }
            i:tok_id() { Expr::Literal(i) }
            tok_left_paren() e:expr() tok_right_paren() { e }
        }

        rule exprs() -> Vec<Expr>
            = exprs:(expr() ** tok_comma()) { exprs }

        rule select_clause() -> Vec<Expr>
            = kw_select() _ ("count(*)" / "COUNT(*)") {vec![]} / kw_select() e:exprs() { e }

        rule from_clause() -> &'input str
            = kw_from() f:$tok_id() { f.trim() }

        rule where_clause() -> Expr
            = kw_where() w:expr() { w }

        pub rule select_stmt() -> SelectStmt
            = s:select_clause() f:from_clause() w:where_clause()? tok_semi()? _ { SelectStmt::new_select(s, f, w) }

        pub rule create_index_stmt() -> CreateIndexStmt<'input>
            = kw_create() kw_unique()? kw_index()
            tok_id() kw_on() table_name:$tok_id()
            tok_left_paren() indexed_columns:($tok_id() ++ tok_comma()) tok_right_paren()
            tok_semi()? _
            {
                CreateIndexStmt {
                    table_name: table_name.trim(),
                    indexed_columns: indexed_columns
                        .into_iter()
                        .map(|indexed_column| indexed_column.trim())
                        .collect(),
                }
            }
    }
}

#[cfg(test)]
pub fn parse_expr(sql: &str) -> Expr {
    parser::expr(sql).expect("syntax error")
}

pub fn parse_select_stmt(sql: &str) -> SelectStmt {
    parser::select_stmt(sql).expect("syntax error")
}

pub fn parse_create_index_stmt(sql: &str) -> CreateIndexStmt {
    parser::create_index_stmt(sql).expect("syntax error")
}

#[cfg(test)]
mod tests {
    use crate::sql::parser::parse_select_stmt;

    use super::super::ast::{BinOp, Expr, Literal};
    use super::{parse_create_index_stmt, parse_expr};

    #[test]
    fn parser_pass_1() {
        let input = "x = 1";
        let got = parse_expr(input);

        let want = Expr::new_binary(
            BinOp::Eq,
            Expr::new_literal(Literal::new_id("x")),
            Expr::new_literal(Literal::new_integer(1)),
        );

        assert_eq!(got, want);
    }

    #[test]
    fn parser_pass_2() {
        let input = "color = 'Yellow'";
        let got = parse_expr(input);

        let want = Expr::new_binary(
            BinOp::Eq,
            Expr::new_literal(Literal::new_id("color")),
            Expr::new_literal(Literal::new_text("Yellow")),
        );

        assert_eq!(got, want);
    }

    #[test]
    fn parser_pass_3() {
        let input = "low <= 0 OR high >= 1";
        let got = parse_expr(input);

        let want = Expr::new_binary(
            BinOp::Or,
            Expr::new_binary(
                BinOp::Lte,
                Expr::new_literal(Literal::new_id("low")),
                Expr::new_literal(Literal::new_integer(0)),
            ),
            Expr::new_binary(
                BinOp::Gte,
                Expr::new_literal(Literal::new_id("high")),
                Expr::new_literal(Literal::new_integer(1)),
            ),
        );

        assert_eq!(got, want);
    }

    #[test]
    fn parser_pass_4() {
        let input = "1 >= val AND 2 <= val OR val == 0";
        let got = parse_expr(input);

        let want = Expr::new_binary(
            BinOp::Or,
            Expr::new_binary(
                BinOp::And,
                Expr::new_binary(
                    BinOp::Gte,
                    Expr::new_literal(Literal::new_integer(1)),
                    Expr::new_literal(Literal::new_id("val")),
                ),
                Expr::new_binary(
                    BinOp::Lte,
                    Expr::new_literal(Literal::new_integer(2)),
                    Expr::new_literal(Literal::new_id("val")),
                ),
            ),
            Expr::new_binary(
                BinOp::Eq,
                Expr::new_literal(Literal::new_id("val")),
                Expr::new_literal(Literal::new_integer(0)),
            ),
        );

        assert_eq!(got, want);
    }

    #[test]
    fn parser_pass_5() {
        let input = "1 + 2";
        parse_expr(input);
    }

    #[test]
    fn parser_pass_6() {
        let input = "val != 0 AND val <= 1 OR val >= 2";
        let got = parse_expr(input);

        let want = Expr::new_binary(
            BinOp::Or,
            Expr::new_binary(
                BinOp::And,
                Expr::new_binary(
                    BinOp::Neq,
                    Expr::new_literal(Literal::new_id("val")),
                    Expr::new_literal(Literal::new_integer(0)),
                ),
                Expr::new_binary(
                    BinOp::Lte,
                    Expr::new_literal(Literal::new_id("val")),
                    Expr::new_literal(Literal::new_integer(1)),
                ),
            ),
            Expr::new_binary(
                BinOp::Gte,
                Expr::new_literal(Literal::new_id("val")),
                Expr::new_literal(Literal::new_integer(2)),
            ),
        );

        assert_eq!(got, want);
    }

    #[test]
    fn parser_pass_7() {
        let input = "val != 0 AND ( val <= 1 OR val >= 2 )";
        let got = parse_expr(input);

        let want = Expr::new_binary(
            BinOp::And,
            Expr::new_binary(
                BinOp::Neq,
                Expr::new_literal(Literal::new_id("val")),
                Expr::new_literal(Literal::new_integer(0)),
            ),
            Expr::new_binary(
                BinOp::Or,
                Expr::new_binary(
                    BinOp::Lte,
                    Expr::new_literal(Literal::new_id("val")),
                    Expr::new_literal(Literal::new_integer(1)),
                ),
                Expr::new_binary(
                    BinOp::Gte,
                    Expr::new_literal(Literal::new_id("val")),
                    Expr::new_literal(Literal::new_integer(2)),
                ),
            ),
        );

        assert_eq!(got, want);
    }

    #[test]
    fn select_stmt() {
        let sql = "SELECT COUNT(*)
        FROM apples";

        let stmt = parse_select_stmt(sql);

        assert!(stmt.select_clause.len() == 0);
        assert_eq!(stmt.from_clause, "apples");
    }

    #[test]
    fn select_stmt_semi() {
        let sql = "SELECT COUNT(*)
        FROM apples;";

        let stmt = parse_select_stmt(sql);

        assert!(stmt.select_clause.len() == 0);
        assert_eq!(stmt.from_clause, "apples");
    }

    #[test]
    fn select_stmt_multi() {
        let sql = "SELECT name, color
        FROM apples";

        let stmt = parse_select_stmt(sql);

        assert_eq!(
            &stmt.select_clause,
            &[
                Expr::new_literal(Literal::new_id("name")),
                Expr::new_literal(Literal::new_id("color"))
            ]
        );

        assert_eq!(stmt.from_clause, "apples");
    }

    #[test]
    fn select_stmt_where() {
        let sql = "SELECT name, color FROM apples WHERE color = 'Yellow'";

        let stmt = parse_select_stmt(sql);

        assert_eq!(
            &stmt.select_clause,
            &[
                Expr::new_literal(Literal::new_id("name")),
                Expr::new_literal(Literal::new_id("color"))
            ]
        );
        assert_eq!(stmt.from_clause, "apples");

        let where_want = Expr::Binary(
            BinOp::Eq,
            Box::new(Expr::Literal(Literal::Id(String::from("color")))),
            Box::new(Expr::Literal(Literal::Text(String::from("Yellow")))),
        );

        assert_eq!(stmt.where_clause.unwrap(), where_want);
    }

    #[test]
    fn create_index_stmt() {
        let sql = "CREATE INDEX idx_companies_country on companies (country)";

        let create_index_stmt = parse_create_index_stmt(sql);

        assert_eq!(create_index_stmt.table_name, "companies");
        assert_eq!(create_index_stmt.indexed_columns, ["country"]);
    }
}
