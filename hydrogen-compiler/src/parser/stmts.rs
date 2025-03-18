use super::exprs::NodeExpr;

#[derive(Debug)]
pub enum Stmts {
    Exit(NodeStmtExit),
    Let(NodeStmtLet),
}

#[derive(Debug)]
pub struct NodeStmtExit {
    pub expr: Option<NodeExpr>,
}

#[derive(Debug)]
pub struct NodeStmtLet {
    pub ident: String,
    pub expr: Option<NodeExpr>,
}
