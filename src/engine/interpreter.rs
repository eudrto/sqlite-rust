use std::{
    iter,
    ops::{Add, Div, Mul, Sub},
};

use crate::sql::{BinOp, Expr, Literal};

use super::{Row, Table, Value};

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
    fn eval_select(self, table: &Table) -> Box<dyn Iterator<Item = Value>> {
        match self {
            Literal::Integer(x) => Box::new(iter::repeat(Value::Integer(x))),
            Literal::Text(x) => Box::new(iter::repeat(Value::Text(x))),
            Literal::Id(id) => Box::new(table.get(&id).cloned().collect::<Vec<_>>().into_iter()),
        }
    }
}

impl Expr {
    pub fn eval_select(self, table: &Table) -> Box<dyn Iterator<Item = Value>> {
        match self {
            Expr::Binary(op, l, r) => Box::new(
                l.eval_select(table)
                    .zip(r.eval_select(table))
                    .map(move |(l, r)| op.eval(&l, &r)),
            ),
            Expr::Literal(literal) => literal.eval_select(table),
        }
    }
}

impl Literal {
    fn eval_where(&self, row: &Row) -> Value {
        match self {
            Literal::Integer(x) => Value::Integer(*x),
            Literal::Text(x) => Value::Text(x.clone()),
            Literal::Id(id) => row[&id].clone(),
        }
    }
}

impl Expr {
    pub fn eval_where(&self, row: &Row) -> Value {
        match self {
            Expr::Binary(op, l, r) => op.eval(&l.eval_where(row), &r.eval_where(row)),
            Expr::Literal(literal) => literal.eval_where(row),
        }
    }
}
