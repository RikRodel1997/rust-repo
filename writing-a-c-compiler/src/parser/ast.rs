use std::fmt::{Display, Formatter, Result};

use crate::lexer::tokens::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Program { function: Box<Node> },
    Function { name: Box<Node>, body: Box<Node> },
    Identifier(String),
    Statement(Statement),
    Expression(Expression),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Node::Program { function } => write!(f, "Program({})", function),
            Node::Function { name, body } => write!(f, "Function({},{})", name, body),
            Node::Identifier(value) => write!(f, "{}", value),
            Node::Statement(statement) => write!(f, "{}", statement),
            Node::Expression(expression) => write!(f, "{}", expression),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Box<Node>),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Return(expr) => write!(f, "Return({expr})"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Constant(i64),
    Unary {
        operator: UnaryOperator,
        expression: Box<Node>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Constant(integer) => write!(f, "Constant({integer})"),
            Self::Unary {
                operator,
                expression,
            } => write!(f, "Unary({},{})", operator, expression),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Complement => write!(f, "~"),
            Self::Negate => write!(f, "-"),
        }
    }
}

impl TryFrom<&Token> for UnaryOperator {
    type Error = String;

    fn try_from(value: &Token) -> std::result::Result<Self, Self::Error> {
        match value {
            Token::Tilde => Ok(Self::Complement),
            Token::Hyphen => Ok(Self::Negate),
            _ => Err(format!("{value} is an invalid unary operator")),
        }
    }
}
