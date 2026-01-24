use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct ParserError {
    pub kind: ParserErrorKind,
    pub message: String,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, message: String) -> Self {
        Self { kind, message }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParserErrorKind {
    EmptyInput,
    UnexpectedToken,
    VariableAlreadyDeclared,
    InvalidInteger,
    InvalidType,
    UnknownIdentifier,
}
