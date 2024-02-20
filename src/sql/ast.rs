#[derive(Debug, PartialEq)]
pub enum BinOp {
    Eq,  /* =, == */
    Neq, /* <>, != */
    Lt,  /* < */
    Lte, /* <= */
    Gt,  /* > */
    Gte, /* >= */
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Text(String),
    Id(String),
}

// #[derive(Debug, PartialEq)]
#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Literal(Literal),
}
