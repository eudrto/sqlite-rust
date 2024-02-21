use std::ops::{Add, Div, Mul, Sub};

use crate::engine::{Row, Value};

use super::ast::{BinOp, Expr, Literal};

impl BinOp {
    fn eval(&self, l: &Value, r: &Value) -> Value {
        match self {
            BinOp::Or => l.or(r),
            BinOp::And => l.and(r),
            BinOp::Eq => Value::from(l == r),
            BinOp::Neq => Value::from(l != r),
            BinOp::Lt => Value::from(l < r),
            BinOp::Lte => Value::from(l <= r),
            BinOp::Gt => Value::from(l > r),
            BinOp::Gte => Value::from(l >= r),
            BinOp::Add => l.add(r),
            BinOp::Sub => l.sub(r),
            BinOp::Mul => l.mul(r),
            BinOp::Div => l.div(r),
        }
    }
}

impl Literal {
    fn eval(&self, row: &Row) -> Value {
        match self {
            Literal::Integer(x) => Value::Integer(*x),
            Literal::Text(x) => Value::Text(x.clone()),
            Literal::Id(id) => row[&id].clone(),
        }
    }
}

impl Expr {
    pub fn eval(&self, row: &Row) -> Value {
        match self {
            Expr::Binary(op, l, r) => op.eval(&l.eval(row), &r.eval(row)),
            Expr::Literal(literal) => literal.eval(row),
        }
    }
}
