use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<TokenValue>,
    pub lit: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Keyword(Keywords),
    Builtin(Builtins),
    Identifier,
    Value,
    Symbol(Symbols),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keywords {
    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Builtins {
    Exit,
}

#[derive(Debug, PartialEq)]
pub enum Symbols {
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    String(String),
    Integer(i64),
    Float(f64),
}

pub fn tokenize(data: String) -> Vec<Token> {
    let symbols = vec![';'];
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
        _ => Token {
            kind: TokenKind::Identifier,
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
        } else if ch == &'.' {
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

fn symbol(_symbols: &Vec<char>, _chars: &mut Peekable<Chars<'_>>) -> Token {
    Token {
        kind: TokenKind::Symbol(Symbols::Semicolon),
        value: None,
        lit: ";".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_one_of_each_type() {
        let data = String::from("exit 20 ;");
        let tokens = tokenize(data);
        assert_eq!(tokens.len(), 3);

        let exit_record = tokens.get(0).unwrap();
        assert_eq!(exit_record.kind, TokenKind::Builtin(Builtins::Exit));
        assert_eq!(exit_record.value, None);
        assert_eq!(exit_record.lit, "exit".to_string());

        let twenty_record = tokens.get(1).unwrap();
        assert_eq!(twenty_record.kind, TokenKind::Value);
        assert_eq!(twenty_record.value, Some(TokenValue::Integer(20)));
        assert_eq!(twenty_record.lit, "20".to_string());

        let semi_record = tokens.get(2).unwrap();
        assert_eq!(semi_record.kind, TokenKind::Symbol(Symbols::Semicolon));
        assert_eq!(semi_record.value, None);
        assert_eq!(semi_record.lit, ";".to_string());
    }
}
