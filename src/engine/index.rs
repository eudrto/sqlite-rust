use crate::sql::{BinOp, Expr, Literal};

use super::Value;

pub fn deconstruct_simple_eq(expr: &Expr) -> Option<(&str, Value)> {
    if let Expr::Binary(binop, l, r) = expr {
        if let (BinOp::Eq, Expr::Literal(l), Expr::Literal(r)) = (binop, &**l, &**r) {
            for (a, b) in [(l, r), (r, l)] {
                if let Literal::Id(id) = a {
                    let value = match b {
                        Literal::Id(_) => continue,
                        Literal::Integer(integer) => Value::Integer(*integer),
                        Literal::Text(text) => Value::Text(text.clone()),
                    };

                    return Some((&id, value));
                }
            }
        }
    }

    None
}
