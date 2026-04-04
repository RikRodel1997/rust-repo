use std::fmt;

use crate::ast::expressions::Identifier;
use crate::ast::node::Node;
use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<Node>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Box<Node>) -> Self {
        LetStatement { token, name, value }
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Let ")?;
        write!(f, "{} {}", self.name.token.literal, &self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Box<Node>>,
}

impl ReturnStatement {
    pub fn new(token: Token, return_value: Option<Box<Node>>) -> Self {
        ReturnStatement {
            token,
            return_value,
        }
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.return_value {
            Some(value) => write!(f, "return {}", value),
            None => write!(f, "return None"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<Node>>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Option<Box<Node>>) -> Self {
        ExpressionStatement { token, expression }
    }
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.expression {
            Some(expr) => write!(f, "{}", expr),
            None => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Node>,
}

impl BlockStatement {
    pub fn new(token: Token, statements: Vec<Node>) -> Self {
        BlockStatement { token, statements }
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block ")?;
        for statement in self.statements.iter() {
            write!(f, "Stmt {}", statement)?;
        }
        Ok(())
    }
}
