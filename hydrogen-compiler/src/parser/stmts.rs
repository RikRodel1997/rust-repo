use crate::tokens::Token;

use super::exprs::NodeExpr;

#[derive(Debug)]
pub struct NodeStmt {
    pub stmt: Option<Stmts>,
}

#[derive(Debug)]
pub enum Stmts {
    Exit(NodeStmtExit),
    // Let(NodeStmtLet),
}

#[derive(Debug)]
pub struct NodeStmtExit {
    pub expr: Option<NodeExpr>,
}

#[derive(Debug)]
pub struct NodeStmtLet {
    pub token: Token,
    pub expr: Option<NodeExpr>,
}
