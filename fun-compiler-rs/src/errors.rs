use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Empty,
}
