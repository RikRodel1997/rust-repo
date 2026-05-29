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
            Node::Program { function } => write!(f, "prog {}", function),
            Node::Function { name, body } => write!(f, "fn {} {}", name, body),
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
            Self::Return(expr) => write!(f, "ret {expr}"),
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
    Binary {
        operator: BinaryOperator,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Constant(integer) => write!(f, "{integer}"),
            Self::Unary {
                operator,
                expression,
            } => write!(f, "{}{}", operator, expression),
            Self::Binary {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", operator, left, right),
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

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    LShift,
    RShift,
    And,
    Or,
    Xor,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Remainder => write!(f, "%"),
            Self::LShift => write!(f, "<<"),
            Self::RShift => write!(f, ">>"),
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
        }
    }
}

impl TryFrom<&Token> for BinaryOperator {
    type Error = String;

    fn try_from(value: &Token) -> std::result::Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Add),
            Token::Hyphen => Ok(Self::Subtract),
            Token::Star => Ok(Self::Multiply),
            Token::ForwardSlash => Ok(Self::Divide),
            Token::Modulo => Ok(Self::Remainder),
            Token::LShift => Ok(Self::LShift),
            Token::RShift => Ok(Self::RShift),
            Token::Ampersand => Ok(Self::And),
            Token::Pipe => Ok(Self::Or),
            Token::Carrot => Ok(Self::Xor),
            _ => Err(format!("{value} is an invalid binary operator")),
        }
    }
}
