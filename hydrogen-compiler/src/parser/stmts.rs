use super::{ParseError, exprs::NodeExpr};

#[derive(Debug)]
pub enum Stmts {
    Exit(NodeStmtExit),
    Let(NodeStmtLet),
}

#[derive(Debug)]
pub struct NodeStmtExit {
    pub expr: Result<NodeExpr, ParseError>,
}

#[derive(Debug)]
pub struct NodeStmtLet {
    pub ident: String,
    pub expr: Result<NodeExpr, ParseError>,
}
