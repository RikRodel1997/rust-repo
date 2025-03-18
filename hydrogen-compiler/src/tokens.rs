use std::{iter::Peekable, str::Chars};

const KEYWORDS: [&str; 1] = ["let"];
const BUILTINS: [&str; 1] = ["exit"];

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lit: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Keyword(Keywords),
    Builtin(Builtins),
    Identifier(IdentifierKinds),
    Value(TokenValue),
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
    Equal,
    EqualEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Integer(i64),
    Float(f64),
}

pub fn tokenize(data: String) -> Vec<Token> {
    let single_char = vec![';', '(', ')', '='];
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
        } else if single_char.contains(char) {
            tokens.push(symbol(&mut chars));
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

    if BUILTINS.contains(&buf.as_str()) {
        return builtin_token(buf);
    } else if KEYWORDS.contains(&buf.as_str()) {
        return keyword_token(buf);
    } else {
        return Token {
            kind: TokenKind::Identifier(IdentifierKinds::Variable),
            lit: buf.to_string(),
        };
    };
}

fn builtin_token(buf: &str) -> Token {
    match buf {
        "exit" => Token {
            kind: TokenKind::Builtin(Builtins::Exit),
            lit: "exit".to_string(),
        },
        _ => {
            eprintln!("Received unexpected builtin {buf}");
            Token {
                kind: TokenKind::Unknown,
                lit: "unknown".to_string(),
            }
        }
    }
}

fn keyword_token(buf: &str) -> Token {
    match buf {
        "let" => Token {
            kind: TokenKind::Keyword(Keywords::Let),

            lit: "let".to_string(),
        },
        _ => {
            eprintln!("Received unexpected keyword {buf}");
            Token {
                kind: TokenKind::Unknown,

                lit: "unknown".to_string(),
            }
        }
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
        kind: TokenKind::Value(value.expect("value is None")),
        lit: buf.to_string(),
    }
}

fn symbol(chars: &mut Peekable<Chars<'_>>) -> Token {
    let token = Token {
        kind: TokenKind::Unknown,

        lit: "".to_string(),
    };
    while let Some(ch) = chars.peek() {
        if *ch == '(' {
            return Token {
                kind: TokenKind::Symbol(Symbols::OpenParen),

                lit: "(".to_string(),
            };
        } else if *ch == ')' {
            return Token {
                kind: TokenKind::Symbol(Symbols::CloseParen),

                lit: ")".to_string(),
            };
        } else if *ch == ';' {
            return Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),

                lit: ";".to_string(),
            };
        } else if *ch == '=' {
            chars.next(); // We know its an equals now
            let next_char = chars.peek();
            return is_one_or_two_char('=', next_char);
        } else {
            break;
        }
    }
    token
}

fn is_one_or_two_char(ch: char, next_char: Option<&char>) -> Token {
    match next_char {
        Some('=') => {
            return Token {
                kind: TokenKind::Symbol(Symbols::EqualEqual),

                lit: "==".to_string(),
            };
        }
        Some(' ') => {
            if ch == '=' {
                return Token {
                    kind: TokenKind::Symbol(Symbols::Equal),

                    lit: "=".to_string(),
                };
            } else {
                panic!("{ch} is not a valid char for is_one_or_two_char");
            }
        }
        None => {
            panic!("Unexpected end of input after '='");
        }
        _ => {
            panic!("Unable to determine next token after '='");
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_one_of_each_type() {
        let data = String::from("let test = 4.0 exit(20); ==");
        let tokens = tokenize(data);
        let expected = vec![
            Token {
                kind: TokenKind::Keyword(Keywords::Let),
                lit: "let".to_string(),
            },
            Token {
                kind: TokenKind::Identifier(IdentifierKinds::Variable),
                lit: "test".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Equal),
                lit: "=".to_string(),
            },
            Token {
                kind: TokenKind::Value(TokenValue::Float(4.0)),
                lit: "4.0".to_string(),
            },
            Token {
                kind: TokenKind::Builtin(Builtins::Exit),
                lit: "exit".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::OpenParen),
                lit: "(".to_string(),
            },
            Token {
                kind: TokenKind::Value(TokenValue::Integer(20)),
                lit: "20".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::CloseParen),
                lit: ")".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),
                lit: ";".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::EqualEqual),
                lit: "==".to_string(),
            },
        ];
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens, expected);
    }
}

pub fn debug_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("TOKEN DEBUG: {:?}", token);
    }
}
