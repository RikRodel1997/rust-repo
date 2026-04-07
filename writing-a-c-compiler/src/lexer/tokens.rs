use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Constant(i64),
    Int,
    Void,
    Return,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    SemiColon,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Identifier(s) => write!(f, "{s}"),
            Self::Constant(i) => write!(f, "{i}"),
            Self::Int => write!(f, "int"),
            Self::Void => write!(f, "void"),
            Self::Return => write!(f, "return"),
            Self::OpenParen => write!(f, "("),
            Self::CloseParen => write!(f, ")"),
            Self::OpenBrace => write!(f, "{{"),
            Self::CloseBrace => write!(f, "}}"),
            Self::SemiColon => write!(f, ";"),
        }
    }
}
