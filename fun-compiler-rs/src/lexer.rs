use crate::SourceChars;
use crate::errors::Error;
use crate::tokens::{Token, TokenKind};

const WHITESPACES: [char; 3] = [' ', '\r', '\n'];
const DELIMITERS: [char; 7] = [':', '=', '(', ')', ',', '{', '}'];
const COMMENT: char = ';';
const KEYWORDS: [&str; 1] = ["defun"];
const TYPES: [&str; 1] = ["integer"];

// TODO: Make this a struct with a .peek() and .next() method
pub fn lex(
    chars: &mut SourceChars,
    position: &mut usize,
    line: &mut usize,
) -> Option<Result<Token, Error>> {
    let mut literal = String::new();

    while let Some(char) = chars.next() {
        let char_size = char.len_utf8();
        let is_delimiter = DELIMITERS.contains(&char);
        let is_whitespace = WHITESPACES.contains(&char);
        let is_comment = char == COMMENT;

        if is_comment {
            while let Some(char) = chars.next() {
                if char == '\n' {
                    *line += 1;
                    break;
                }
                chars.next();
                *position += char_size;
            }
            continue;
        }

        if is_whitespace {
            if char == '\n' {
                *line += 1;
            }
            *position += char_size;
            continue;
        }

        if is_delimiter {
            let delimiter = Some(Ok(Token::new(
                *position,
                *position + char_size,
                *line,
                char.into(),
                match char {
                    ':' => TokenKind::Colon,
                    '=' => TokenKind::Equal,
                    '(' => TokenKind::LeftParen,
                    ')' => TokenKind::RightParen,
                    ',' => TokenKind::Comma,
                    '{' => TokenKind::LeftBrace,
                    '}' => TokenKind::RightBrace,
                    _ => panic!("Unexpected delimiter"),
                },
            )));
            *position += char_size;
            return delimiter;
        }

        if !is_whitespace && !is_delimiter {
            if char.is_alphanumeric() {
                let literal_start = *position;
                literal.push(char);
                *position += char_size;

                while let Some(&char) = chars.peek() {
                    if char.is_alphanumeric() {
                        literal.push(char);
                        chars.next();
                        *position += char_size;
                    } else {
                        break;
                    }
                }

                let kind = match literal {
                    _ if KEYWORDS.contains(&literal.as_str()) => TokenKind::Keyword,
                    _ if TYPES.contains(&literal.as_str()) => TokenKind::Type,
                    _ if literal.chars().next().unwrap().is_numeric() => TokenKind::Value,
                    _ => TokenKind::Identifier,
                };

                return Some(Ok(Token::new(
                    literal_start,
                    *position,
                    *line,
                    literal,
                    kind,
                )));
            }
        }
        continue;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespaces() {
        let source = "a:integer\na : integer";
        let expected = vec![
            Token {
                start: 0,
                end: 1,
                line: 1,
                literal: "a".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 1,
                end: 2,
                line: 1,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 2,
                end: 9,
                line: 1,
                literal: "integer".into(),
                kind: TokenKind::Type,
            },
            Token {
                start: 10,
                end: 11,
                line: 2,
                literal: "a".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 12,
                end: 13,
                line: 2,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 14,
                end: 21,
                line: 2,
                literal: "integer".into(),
                kind: TokenKind::Type,
            },
        ];

        let mut chars = source.chars().peekable();
        let mut position = 0;
        let mut line = 1;
        let mut it_position = 0;

        while let Some(tok) = lex(&mut chars, &mut position, &mut line) {
            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), expected[it_position]);
            it_position += 1;
        }
    }

    #[test]
    fn test_comments() {
        let source = "a:integer ; test comment\n;another test comment\na : integer";
        let expected = vec![
            Token {
                start: 0,
                end: 1,
                line: 1,
                literal: "a".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 1,
                end: 2,
                line: 1,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 2,
                end: 9,
                line: 1,
                literal: "integer".into(),
                kind: TokenKind::Type,
            },
            Token {
                start: 10,
                end: 11,
                line: 2,
                literal: "a".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 12,
                end: 13,
                line: 2,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 14,
                end: 21,
                line: 2,
                literal: "integer".into(),
                kind: TokenKind::Type,
            },
        ];

        let mut chars = source.chars().peekable();
        let mut position = 0;
        let mut line = 1;
        let mut it_position = 0;

        while let Some(tok) = lex(&mut chars, &mut position, &mut line) {
            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), expected[it_position]);
            it_position += 1;
        }
    }

    #[test]
    fn test_variable_declaration_lex() {
        let source = "a : integer = 0\na := 1";
        let expected = vec![
            Token {
                start: 0,
                end: 1,
                line: 1,
                literal: "a".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 2,
                end: 3,
                line: 1,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 4,
                end: 11,
                line: 1,
                literal: "integer".into(),
                kind: TokenKind::Type,
            },
            Token {
                start: 12,
                end: 13,
                line: 1,
                literal: "=".into(),
                kind: TokenKind::Equal,
            },
            Token {
                start: 14,
                end: 15,
                line: 1,
                literal: "0".into(),
                kind: TokenKind::Value,
            },
            Token {
                start: 16,
                end: 17,
                line: 2,
                literal: "a".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 18,
                end: 19,
                line: 2,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 19,
                end: 20,
                line: 2,
                literal: "=".into(),
                kind: TokenKind::Equal,
            },
            Token {
                start: 21,
                end: 22,
                line: 2,
                literal: "1".into(),
                kind: TokenKind::Value,
            },
        ];

        let mut chars = source.chars().peekable();
        let mut position = 0;
        let mut line = 1;
        let mut it_position = 0;
        while let Some(tok) = lex(&mut chars, &mut position, &mut line) {
            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), expected[it_position]);
            it_position += 1;
        }
    }

    #[test]
    fn test_function_declaration_lex() {
        let source = "defun foo(a:integer, b:integer):integer {\n\n}";
        let expected = vec![
            Token {
                start: 0,
                end: 5,
                line: 1,
                literal: "defun".into(),
                kind: TokenKind::Keyword,
            },
            Token {
                start: 6,
                end: 9,
                line: 1,
                literal: "foo".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 9,
                end: 10,
                line: 1,
                literal: "(".into(),
                kind: TokenKind::LeftParen,
            },
            Token {
                start: 10,
                end: 11,
                line: 1,
                literal: "a".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 11,
                end: 12,
                line: 1,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 12,
                end: 19,
                line: 1,
                literal: "integer".into(),
                kind: TokenKind::Type,
            },
            Token {
                start: 19,
                end: 20,
                line: 1,
                literal: ",".into(),
                kind: TokenKind::Comma,
            },
            Token {
                start: 21,
                end: 22,
                line: 1,
                literal: "b".into(),
                kind: TokenKind::Identifier,
            },
            Token {
                start: 22,
                end: 23,
                line: 1,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 23,
                end: 30,
                line: 1,
                literal: "integer".into(),
                kind: TokenKind::Type,
            },
            Token {
                start: 30,
                end: 31,
                line: 1,
                literal: ")".into(),
                kind: TokenKind::RightParen,
            },
            Token {
                start: 31,
                end: 32,
                line: 1,
                literal: ":".into(),
                kind: TokenKind::Colon,
            },
            Token {
                start: 32,
                end: 39,
                line: 1,
                literal: "integer".into(),
                kind: TokenKind::Type,
            },
            Token {
                start: 40,
                end: 41,
                line: 1,
                literal: "{".into(),
                kind: TokenKind::LeftBrace,
            },
            Token {
                start: 43,
                end: 44,
                line: 3,
                literal: "}".into(),
                kind: TokenKind::RightBrace,
            },
        ];

        let mut chars = source.chars().peekable();
        let mut position = 0;
        let mut line = 1;
        let mut it_position = 0;
        while let Some(tok) = lex(&mut chars, &mut position, &mut line) {
            assert!(tok.is_ok());
            assert_eq!(tok.unwrap(), expected[it_position]);
            it_position += 1;
        }
    }
}
