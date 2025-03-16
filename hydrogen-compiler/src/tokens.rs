use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<TokenValue>,
    pub lit: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Keyword(Keywords),
    Builtin(Builtins),
    Identifier(IdentifierKinds),
    Value,
    Symbol(Symbols),
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keywords {
    Let,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdentifierKinds {
    Variable,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Builtins {
    Exit,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbols {
    Semicolon,
    OpenParen,
    CloseParen,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    String(String),
    Integer(i64),
    Float(f64),
}

pub fn tokenize(data: String) -> Vec<Token> {
    let symbols = vec![';', '(', ')'];
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: String = String::new();
    let mut chars = data.chars().peekable();

    while let Some(char) = chars.peek() {
        if char.is_whitespace() {
            chars.next();
            continue;
        }

        if char.is_alphabetic() {
            tokens.push(alphabetic(&mut buf, &mut chars));
            buf.clear();
        } else if char.is_numeric() {
            tokens.push(numeric(&mut buf, &mut chars));
            buf.clear();
        } else if symbols.contains(char) {
            tokens.push(symbol(&symbols, &mut chars));
            chars.next();
        }
    }

    tokens
}

pub fn debug_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("TOKEN DEBUG: {:?}", token);
    }
}

fn alphabetic(buf: &mut String, chars: &mut Peekable<Chars<'_>>) -> Token {
    while let Some(ch) = chars.peek() {
        if ch.is_alphanumeric() {
            buf.push(*ch);
            chars.next();
        } else {
            break;
        }
    }

    match buf.as_str() {
        "exit" => Token {
            kind: TokenKind::Builtin(Builtins::Exit),
            value: None,
            lit: "exit".to_string(),
        },
        "let" => Token {
            kind: TokenKind::Keyword(Keywords::Let),
            value: None,
            lit: "let".to_string(),
        },
        _ => Token {
            kind: TokenKind::Identifier(IdentifierKinds::Variable),
            value: None,
            lit: buf.to_string(),
        },
    }
}

fn numeric(buf: &mut String, chars: &mut Peekable<Chars<'_>>) -> Token {
    while let Some(ch) = chars.peek() {
        if ch.is_numeric() {
            buf.push(*ch);
            chars.next();
        } else if *ch == '.' {
            buf.push(*ch);
            chars.next();
        } else {
            break;
        }
    }

    let value = match buf.contains(".") {
        false => Some(TokenValue::Integer(
            buf.parse::<i64>()
                .expect(&format!("buf is not an integer! {buf}")),
        )),
        true => Some(TokenValue::Float(
            buf.parse::<f64>()
                .expect(&format!("buf is not a float! {buf}")),
        )),
    };

    Token {
        kind: TokenKind::Value,
        value,
        lit: buf.to_string(),
    }
}

fn symbol(symbols: &Vec<char>, chars: &mut Peekable<Chars<'_>>) -> Token {
    let token = Token {
        kind: TokenKind::Unknown,
        value: None,
        lit: "".to_string(),
    };
    while let Some(ch) = chars.peek() {
        if *ch == '(' {
            return Token {
                kind: TokenKind::Symbol(Symbols::OpenParen),
                value: None,
                lit: "(".to_string(),
            };
        } else if *ch == ')' {
            return Token {
                kind: TokenKind::Symbol(Symbols::CloseParen),
                value: None,
                lit: ")".to_string(),
            };
        } else if *ch == ';' {
            return Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),
                value: None,
                lit: ";".to_string(),
            };
        } else {
            break;
        }
    }
    token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_one_of_each_type() {
        let data = String::from("exit(20);");
        let tokens = tokenize(data);
        let expected = vec![
            Token {
                kind: TokenKind::Builtin(Builtins::Exit),
                value: None,
                lit: "exit".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::OpenParen),
                value: None,
                lit: "(".to_string(),
            },
            Token {
                kind: TokenKind::Value,
                value: Some(TokenValue::Integer(20)),
                lit: "20".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::CloseParen),
                value: None,
                lit: ")".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),
                value: None,
                lit: ";".to_string(),
            },
        ];
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens, expected);
    }
}
