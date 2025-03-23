use super::exprs::Exprs;

#[derive(Debug, PartialEq)]
pub enum Stmts {
    Exit(NodeStmtExit),
    Let(NodeStmtLet),
}

#[derive(Debug, PartialEq)]
pub struct NodeStmtExit {
    pub expr: Exprs,
}

#[derive(Debug, PartialEq)]
pub struct NodeStmtLet {
    pub ident: String,
    pub expr: Exprs,
}
