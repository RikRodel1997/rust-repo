use std::{fmt, iter::Peekable, str::Chars};

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
    Ident(IdentKinds),
    Value(TokenValue),
    Symbol(Symbols),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keywords {
    Let,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdentKinds {
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
    Plus,
    Star,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Integer(i64),
    Float(f64),
}

pub fn tokenize(data: String) -> Vec<Token> {
    let single_char = vec![';', '(', ')', '=', '+', '*'];
    let mut tokens: Vec<Result<Token, TokenizeError>> = Vec::new();
    let mut chars = data.chars().peekable();

    while let Some(ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        if ch.is_alphabetic() {
            tokens.push(alphabetic(&mut chars));
        } else if ch.is_numeric() {
            tokens.push(numeric(&mut chars));
        } else if single_char.contains(ch) {
            tokens.push(symbol(&mut chars));
            chars.next();
        } else {
            panic!("Unable to determine path for ch {ch}");
        }
    }

    let tokens: Result<Vec<Token>, TokenizeError> = tokens.into_iter().collect();
    match tokens {
        Ok(valid_tokens) => valid_tokens,
        Err(e) => {
            panic!("{}", e.message);
        }
    }
}

fn alphabetic(chars: &mut Peekable<Chars<'_>>) -> Result<Token, TokenizeError> {
    let mut buf = String::new();
    while let Some(ch) = chars.peek() {
        if ch.is_alphanumeric() {
            buf.push(*ch);
            chars.next();
        } else {
            break;
        }
    }

    if BUILTINS.contains(&buf.as_str()) {
        return builtin_token(&buf);
    } else if KEYWORDS.contains(&buf.as_str()) {
        return keyword_token(&buf);
    } else {
        return Ok(Token {
            kind: TokenKind::Ident(IdentKinds::Variable),
            lit: buf.to_string(),
        });
    };
}

fn builtin_token(buf: &str) -> Result<Token, TokenizeError> {
    match buf {
        "exit" => Ok(Token {
            kind: TokenKind::Builtin(Builtins::Exit),
            lit: "exit".to_string(),
        }),
        _ => Err(TokenizeError {
            message: format!("Unexpected builtin: {buf}"),
        }),
    }
}

fn keyword_token(buf: &str) -> Result<Token, TokenizeError> {
    match buf {
        "let" => Ok(Token {
            kind: TokenKind::Keyword(Keywords::Let),
            lit: "let".to_string(),
        }),
        _ => Err(TokenizeError {
            message: format!("Unexpected keyword: {buf}"),
        }),
    }
}

fn numeric(chars: &mut Peekable<Chars<'_>>) -> Result<Token, TokenizeError> {
    let mut buf = String::new();
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

    match buf.contains(".") {
        false => match buf.parse::<i64>() {
            Ok(num) => Ok(Token {
                kind: TokenKind::Value(TokenValue::Integer(num)),
                lit: format!("{num}"),
            }),
            Err(_) => Err(TokenizeError {
                message: format!("buf {buf} is not an integer!"),
            }),
        },
        true => match buf.parse::<f64>() {
            Ok(num) => Ok(Token {
                kind: TokenKind::Value(TokenValue::Float(num)),
                lit: format!("{:?}", num),
            }),
            Err(_) => Err(TokenizeError {
                message: format!("buf {buf} is not a float!"),
            }),
        },
    }
}

fn symbol(chars: &mut Peekable<Chars<'_>>) -> Result<Token, TokenizeError> {
    let ch = chars.peek().unwrap();
    match ch {
        '(' => {
            return Ok(Token {
                kind: TokenKind::Symbol(Symbols::OpenParen),
                lit: "(".to_string(),
            });
        }
        ')' => {
            return Ok(Token {
                kind: TokenKind::Symbol(Symbols::CloseParen),
                lit: ")".to_string(),
            });
        }
        ';' => {
            return Ok(Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),
                lit: ";".to_string(),
            });
        }
        '=' => {
            chars.next();
            let next_char = chars.peek().unwrap();
            return is_one_or_two_char('=', next_char);
        }
        '+' => {
            return Ok(Token {
                kind: TokenKind::Symbol(Symbols::Plus),
                lit: "+".to_string(),
            });
        }
        '*' => {
            return Ok(Token {
                kind: TokenKind::Symbol(Symbols::Star),
                lit: "*".to_string(),
            });
        }
        _ => {
            return Err(TokenizeError {
                message: format!("Unexpected symbol {ch}"),
            });
        }
    }
}

fn is_one_or_two_char(ch: char, next_char: &char) -> Result<Token, TokenizeError> {
    match next_char {
        '=' => Ok(Token {
            kind: TokenKind::Symbol(Symbols::EqualEqual),
            lit: "==".to_string(),
        }),
        ' ' => match ch {
            '=' => Ok(Token {
                kind: TokenKind::Symbol(Symbols::Equal),
                lit: "=".to_string(),
            }),
            _ => Err(TokenizeError {
                message: format!("{ch} is not a valid char for is_one_or_two_char"),
            }),
        },
        _ => Err(TokenizeError {
            message: format!(
                "Unable to determine next token after ch: {ch}, next_char {next_char}"
            ),
        }),
    }
}

pub fn debug_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("TOKEN DEBUG: {:?}", token);
    }
}

pub struct TokenizeError {
    message: String,
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl fmt::Debug for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
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
                kind: TokenKind::Ident(IdentKinds::Variable),
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
