use crate::tokens::{Token, TokenValue};

#[derive(Debug)]
pub struct NodeExpr {
    pub kind: Exprs,
    pub token: Token,
}

#[derive(Debug)]
pub enum Exprs {
    Literal,
    Ident,
    Binary(NodeExprBinary),
}

#[derive(Debug)]
pub struct NodeExprBinary {
    pub lhs: TokenValue,
    pub rhs: TokenValue,
    pub operator: BinaryKinds,
}

#[derive(Debug)]
pub enum BinaryKinds {
    Addition,
    Multiplication,
}
