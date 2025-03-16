use crate::tokens::Token;

#[derive(Debug)]
pub struct NodeExpr {
    pub kind: Exprs,
    pub token: Token,
}

#[derive(Debug)]
pub enum Exprs {
    Literal,
    Identifier,
}
