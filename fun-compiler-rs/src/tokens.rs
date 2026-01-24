#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub literal: String,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(start: usize, end: usize, line: usize, literal: String, kind: TokenKind) -> Self {
        Self {
            start,
            end,
            line,
            literal,
            kind,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    Keyword,
    Type,
    Value,
    Colon,
    Equal,
    LeftParen,
    RightParen,
    Comma,
    LeftBrace,
    RightBrace,
}
