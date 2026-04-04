use core::fmt;
use std::hash::Hash;

use crate::ast::{
    Program,
    expressions::{self, *},
    statements::*,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Program(Program),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    BlockStatement(BlockStatement),
    ExpressionStatement(ExpressionStatement),
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(Boolean),
    PrefixExpression(Box<PrefixExpression>),
    InfixExpression(Box<InfixExpression>),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(Box<CallExpression>),
    String(StringLiteral),
    Array(Array),
    IndexExpression(IndexExpression),
    Hash(expressions::Hash),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Program(program) => write!(f, "{}", program),
            Self::LetStatement(stmt) => write!(f, "{}", stmt),
            Self::ReturnStatement(stmt) => write!(f, "{}", stmt),
            Self::BlockStatement(stmt) => write!(f, "{}", stmt),
            Self::ExpressionStatement(stmt) => write!(f, "{}", stmt),
            Self::Identifier(expr) => write!(f, "{}", expr),
            Self::IntegerLiteral(expr) => write!(f, "{}", expr),
            Self::Boolean(expr) => write!(f, "{}", expr),
            Self::PrefixExpression(expr) => write!(f, "{}", expr),
            Self::InfixExpression(expr) => write!(f, "{}", expr),
            Self::IfExpression(expr) => write!(f, "{}", expr),
            Self::FunctionLiteral(expr) => write!(f, "{}", expr),
            Self::CallExpression(expr) => write!(f, "{}", expr),
            Self::String(string) => write!(f, "{}", string),
            Self::Array(array) => write!(f, "{}", array),
            Self::IndexExpression(expr) => write!(f, "{}", expr),
            Self::Hash(hash) => write!(f, "{}", hash),
        }
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else if self < other {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {}
}
