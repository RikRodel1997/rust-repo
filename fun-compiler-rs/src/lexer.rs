use crate::SourceChars;
use crate::tokens::{Token, TokenKind};

const WHITESPACES: [char; 3] = [' ', '\r', '\n'];
const DELIMITERS: [char; 7] = [':', '=', '(', ')', ',', '{', '}'];
const COMMENTS: [char; 1] = [';'];
const KEYWORDS: [&str; 1] = ["defun"];
const TYPES: [&str; 1] = ["integer"];

pub struct Lexer<'a> {
    chars: SourceChars<'a>,
    peeked: Option<Token>,
    pub position: usize,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: SourceChars<'a>) -> Self {
        Lexer {
            chars,
            peeked: None,
            position: 0,
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.peeked.is_some() {
            return self.peeked.take();
        }

        while let Some(char) = self.next_char() {
            if self.comment(&char) || self.whitespace(&char) {
                continue;
            }

            match self.delimiter(&char) {
                Some(token) => return Some(token),
                None => {}
            }

            return Some(self.alphanumeric(&char));
        }

        None
    }

    pub fn peek_token(&mut self) -> Option<Token> {
        if self.peeked.is_none() {
            self.peeked = self.next_token();
        }

        self.peeked.clone()
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn whitespace(&mut self, char: &char) -> bool {
        if WHITESPACES.contains(&char) {
            if char == &'\n' {
                self.line += 1;
            }
            self.position += char.len_utf8();
            true
        } else {
            false
        }
    }

    fn comment(&mut self, char: &char) -> bool {
        if COMMENTS.contains(&char) {
            while let Some(char) = self.next_char() {
                if char == '\n' {
                    self.line += 1;
                    break;
                }
                self.next_char();
                self.position += char.len_utf8();
            }
            true
        } else {
            false
        }
    }

    fn delimiter(&mut self, char: &char) -> Option<Token> {
        if DELIMITERS.contains(&char) {
            let delimiter = Some(Token::new(
                self.position,
                self.position + char.len_utf8(),
                self.line,
                char.clone().into(),
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
            ));
            self.position += char.len_utf8();
            return delimiter;
        }
        None
    }

    fn alphanumeric(&mut self, char: &char) -> Token {
        let mut literal = String::new();
        let start_pos = self.position;
        literal.push(*char);
        self.position += char.len_utf8();

        while let Some(&char) = self.peek_char() {
            if char.is_alphanumeric() {
                literal.push(char);
                self.next_char();
                self.position += char.len_utf8();
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

        Token::new(start_pos, self.position, self.line, literal, kind)
    }
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

        let chars = source.chars().peekable();
        let mut it_position = 0;

        let mut lexer = Lexer::new(chars);

        while let Some(tok) = lexer.next_token() {
            assert_eq!(tok, expected[it_position]);
            it_position += 1;
        }
    }

    #[test]
    fn test_peek_token() {
        let source = "a:integer";

        let chars = source.chars().peekable();

        let mut lexer = Lexer::new(chars);
        let peeked_identifier = lexer.peek_token();
        assert_eq!(
            peeked_identifier,
            Some(Token {
                start: 0,
                end: 1,
                line: 1,
                literal: "a".into(),
                kind: TokenKind::Identifier
            })
        );

        let token_identifier = lexer.next_token();
        assert_eq!(
            token_identifier,
            Some(Token {
                start: 0,
                end: 1,
                line: 1,
                literal: "a".into(),
                kind: TokenKind::Identifier
            })
        );

        let colon = lexer.next_token();
        assert_eq!(
            colon,
            Some(Token {
                start: 1,
                end: 2,
                line: 1,
                literal: ":".into(),
                kind: TokenKind::Colon
            })
        );

        let peeked_type = lexer.peek_token();
        assert_eq!(
            peeked_type,
            Some(Token {
                start: 2,
                end: 9,
                line: 1,
                literal: "integer".into(),
                kind: TokenKind::Type
            })
        );

        let type_token = lexer.next_token();
        assert_eq!(
            type_token,
            Some(Token {
                start: 2,
                end: 9,
                line: 1,
                literal: "integer".into(),
                kind: TokenKind::Type
            })
        );
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

        let chars = source.chars().peekable();
        let mut it_position = 0;

        let mut lexer = Lexer::new(chars);

        while let Some(tok) = lexer.next_token() {
            assert_eq!(tok, expected[it_position]);
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

        let chars = source.chars().peekable();
        let mut it_position = 0;

        let mut lexer = Lexer::new(chars);

        while let Some(tok) = lexer.next_token() {
            assert_eq!(tok, expected[it_position]);
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

        let chars = source.chars().peekable();
        let mut it_position = 0;

        let mut lexer = Lexer::new(chars);

        while let Some(tok) = lexer.next_token() {
            assert_eq!(tok, expected[it_position]);
            it_position += 1;
        }
    }
}
