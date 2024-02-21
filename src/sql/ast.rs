#[derive(Debug, PartialEq)]
pub enum BinOp {
    Or,  /* OR */
    And, /* AND */
    Eq,  /* =, == */
    Neq, /* <>, != */
    Lt,  /* < */
    Lte, /* <= */
    Gt,  /* > */
    Gte, /* >= */
    Add, /* + */
    Sub, /* - */
    Mul, /* * */
    Div, /* / */
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Integer(i64),
    Text(String),
    Id(String),
}

#[cfg(test)]
impl Literal {
    pub fn new_integer(i: i64) -> Literal {
        Literal::Integer(i)
    }

    pub fn new_text(text: &str) -> Literal {
        Literal::Text(String::from(text))
    }

    pub fn new_id(text: &str) -> Literal {
        Literal::Id(String::from(text))
    }
}

// #[derive(Debug, PartialEq)]
#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Literal(Literal),
}

#[cfg(test)]
impl Expr {
    pub fn new_binary(binop: BinOp, l: Expr, r: Expr) -> Expr {
        Expr::Binary(binop, Box::new(l), Box::new(r))
    }

    pub fn new_literal(literal: Literal) -> Expr {
        Expr::Literal(literal)
    }
}
