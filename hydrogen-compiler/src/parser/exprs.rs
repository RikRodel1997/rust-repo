use crate::tokens::{Token, TokenValue};

#[derive(Debug, PartialEq)]
pub enum Exprs {
    Lit(ExprLit),
    Bin(ExprBin),
    Ident(ExprIdent),
}

#[derive(Debug, PartialEq)]
pub struct ExprLit {
    pub value: TokenValue,
}

#[derive(Debug, PartialEq)]
pub struct ExprIdent {
    pub token: Token,
}

#[derive(Debug, PartialEq)]
pub struct ExprBin {
    pub lhs: Token,
    pub rhs: Token,
    pub operator: BinOps,
}

#[derive(Debug, PartialEq)]
pub enum BinOps {
    Addition,
    Multiplication,
}
