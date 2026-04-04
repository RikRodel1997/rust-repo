use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}

#[derive(Eq, Hash, Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Integer,
    String,
    Assign,
    Plus,
    Comma,
    SemiColon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Function,
    Let,
    Bang,
    Minus,
    Slash,
    Asterisk,
    LessThan,
    GreaterThan,
    If,
    Else,
    Return,
    True,
    False,
    Equal,
    NotEqual,
    Colon,
}

pub fn lookup_identifier(identifier: &str) -> TokenType {
    let keywords = HashMap::from([
        ("fn", TokenType::Function),
        ("let", TokenType::Let),
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("return", TokenType::Return),
    ]);
    match keywords.get(identifier) {
        Some(keyword) => *keyword,
        None => TokenType::Ident,
    }
}
