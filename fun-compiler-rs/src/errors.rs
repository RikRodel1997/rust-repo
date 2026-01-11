use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Parser(ParserError),
    Lexer(LexerError),
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    EmptyInput,
    UnexpectedToken,
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    EmptyInput,
    UnexpectedEndOfInput,
    UnexpectedToken,
    InvalidInteger,
    InvalidType,
    UnknownIdentifier,
}
