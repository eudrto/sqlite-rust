use super::ast::{BinOp, Expr, Literal};

use peg::parser;

parser! {
    pub grammar parser() for str {
        // whitespace
        rule _ = [' ' | '\n' | '\t']*

        // character
        rule alpha() -> &'input str = a:$(['a'..='z' | 'A'..='Z']) { a }
        rule alpha_() -> &'input str = a:$(alpha() / "_") { a }
        rule num() -> &'input str = n:$(['0'..='9']) { n }
        rule alphanum() -> &'input str = a:$(alpha() / num()) { a }
        rule alphanum_() -> &'input str = a:$(alphanum() / "_") { a }

        // token
        rule tok_or() -> &'input str = _ t:$"OR" {t}
        rule tok_and() -> &'input str = _ t:$"AND" {t}
        rule tok_eq() -> &'input str = _ t:$("==" / "=") {t}
        rule tok_neq() -> &'input str = _ t:$("<>" / "!=") {t}
        rule tok_lt() -> &'input str = _ t:$"<" {t}
        rule tok_lte() -> &'input str = _ t:$"<=" {t}
        rule tok_gt() -> &'input str = _ t:$">" {t}
        rule tok_gte() -> &'input str = _ t:$">=" {t}

        rule tok_integer() -> Literal
            = _ i:$(num()+) { Literal::Integer(i.parse().unwrap()) }

        rule tok_string() -> Literal
            = _ "'" s:$([^ '\'']*) "'" { Literal::Text(s.into()) }

        rule tok_id() -> Literal
            = _ i:$(alpha_() alphanum_()*) { Literal::Id(i.into()) }

        // node
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
            i:tok_integer() { Expr::Literal(i)  }
            s:tok_string() { Expr::Literal(s) }
            i:tok_id() { Expr::Literal(i) }
        }
    }
}

pub fn parse_expr(sql: &str) -> Expr {
    parser::expr(sql).expect("syntax error")
}

#[cfg(test)]
mod tests {
    use super::super::ast::{BinOp, Expr, Literal};
    use super::parse_expr;

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
    #[should_panic(expected = "ParseError")]
    fn parser_fail_1() {
        let input = "1 + 2";
        parse_expr(input);
    }
}
