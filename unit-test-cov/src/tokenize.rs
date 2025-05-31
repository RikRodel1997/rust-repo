use std::{iter::Peekable, str::Chars};

pub enum TokenKind {
    Keyword(KeywordKinds),
    Identifier(IdentifierKinds),
    Value(ValueKinds),
    Symbol(SymbolKinds),
    Eof,
}

pub enum ValueKinds {
    String,
    Integer,
    Float,
}

pub enum IdentifierKinds {
    Function,
    Variable,
    Class,
}

pub enum KeywordKinds {
    Let,
    Fun,
    Class,
}

pub enum SymbolKinds {
    Semicolon,
    OpenParen,
    CloseParen,
    Equal,
    EqualEqual,
    Plus,
    Minus,
    Divide,
    Star,
}

pub struct Token {
    kind: TokenKind,
    lit: String,
}

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            let char = self.chars.next();
            match char {
                Some(char) if char.is_alphabetic() => match self.alpha_token() {
                    Ok(tk) => tokens.push(tk),
                    Err(e) => return Err(e),
                },
                Some(c) => return Err(format!("Unknown char found {c}")),
                None => tokens.push(Token {
                    kind: TokenKind::Eof,
                    lit: "".to_string(),
                }),
            }
        }

        Ok(tokens)
    }

    fn alpha_token(&mut self) -> Result<Token, String> {
        self.chars.next();
        Ok(Token {
            kind: TokenKind::Identifier(IdentifierKinds::Variable),
            lit: "test".to_string(),
        })
    }
}
