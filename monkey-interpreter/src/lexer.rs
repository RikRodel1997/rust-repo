use crate::token::{Token, TokenType, lookup_identifier};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    pub input: String,
    pub pos: usize,
    pub read_pos: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            pos: 0,
            read_pos: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            '+' => Token::new(TokenType::Plus, String::from("+")),
            '(' => Token::new(TokenType::LParen, String::from("(")),
            ')' => Token::new(TokenType::RParen, String::from(")")),
            '{' => Token::new(TokenType::LBrace, String::from("{")),
            '}' => Token::new(TokenType::RBrace, String::from("}")),
            ',' => Token::new(TokenType::Comma, String::from(",")),
            ';' => Token::new(TokenType::SemiColon, String::from(";")),
            '-' => Token::new(TokenType::Minus, String::from("-")),
            '*' => Token::new(TokenType::Asterisk, String::from("*")),
            '<' => Token::new(TokenType::LessThan, String::from("<")),
            '>' => Token::new(TokenType::GreaterThan, String::from(">")),
            '/' => Token::new(TokenType::Slash, String::from("/")),
            '[' => Token::new(TokenType::LBracket, String::from("[")),
            ']' => Token::new(TokenType::RBracket, String::from("]")),
            ':' => Token::new(TokenType::Colon, String::from(":")),
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::Equal, String::from("=="))
                } else {
                    Token::new(TokenType::Assign, String::from("="))
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new(TokenType::NotEqual, String::from("!="))
                } else {
                    Token::new(TokenType::Bang, String::from("!"))
                }
            }
            '\"' => Token::new(TokenType::String, self.read_string()),
            '\0' => Token::new(TokenType::Eof, String::from("")),
            _ => {
                if self.is_letter(self.ch) {
                    let identifier = self.read_identifier();
                    let token_type = lookup_identifier(&identifier);
                    return Token::new(token_type, identifier);
                } else if self.is_digit(self.ch) {
                    let number = self.read_number();
                    return Token::new(TokenType::Integer, number);
                } else {
                    Token::new(TokenType::Illegal, String::from(""))
                }
            }
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.pos;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[start_pos..self.pos].to_string()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.pos;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        self.input[start_pos..self.pos].to_string()
    }

    fn read_string(&mut self) -> String {
        let start_pos = self.pos + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' {
                break;
            }
        }
        self.input[start_pos..self.pos].to_string()
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self
                .input
                .chars()
                .nth(self.read_pos)
                .expect("failed to read a character from input");
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&mut self) -> char {
        if self.read_pos >= self.input.len() {
            return '\0';
        }
        return self
            .input
            .chars()
            .nth(self.read_pos)
            .expect("failed to read a character from input");
    }

    fn is_letter(&mut self, ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(&mut self, ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = String::from(
            "let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
            
            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
            \"foobar\"
            \"foo bar\"
            [1, 2];
            {\"foo\":\"bar\"}",
        );
        let expected = vec![
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Function,
                literal: String::from("fn"),
            },
            Token {
                token_type: TokenType::LParen,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::RParen,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("x"),
            },
            Token {
                token_type: TokenType::Plus,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("y"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("result"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("add"),
            },
            Token {
                token_type: TokenType::LParen,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("ten"),
            },
            Token {
                token_type: TokenType::RParen,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Bang,
                literal: String::from("!"),
            },
            Token {
                token_type: TokenType::Minus,
                literal: String::from("-"),
            },
            Token {
                token_type: TokenType::Slash,
                literal: String::from("/"),
            },
            Token {
                token_type: TokenType::Asterisk,
                literal: String::from("*"),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::LessThan,
                literal: String::from("<"),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::GreaterThan,
                literal: String::from(">"),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::If,
                literal: String::from("if"),
            },
            Token {
                token_type: TokenType::LParen,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::LessThan,
                literal: String::from("<"),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::RParen,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::Return,
                literal: String::from("return"),
            },
            Token {
                token_type: TokenType::True,
                literal: String::from("true"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Else,
                literal: String::from("else"),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::Return,
                literal: String::from("return"),
            },
            Token {
                token_type: TokenType::False,
                literal: String::from("false"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::Equal,
                literal: String::from("=="),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("10"),
            },
            Token {
                token_type: TokenType::NotEqual,
                literal: String::from("!="),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("9"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::String,
                literal: String::from("foobar"),
            },
            Token {
                token_type: TokenType::String,
                literal: String::from("foo bar"),
            },
            Token {
                token_type: TokenType::LBracket,
                literal: String::from("["),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("1"),
            },
            Token {
                token_type: TokenType::Comma,
                literal: String::from(","),
            },
            Token {
                token_type: TokenType::Integer,
                literal: String::from("2"),
            },
            Token {
                token_type: TokenType::RBracket,
                literal: String::from("]"),
            },
            Token {
                token_type: TokenType::SemiColon,
                literal: String::from(";"),
            },
            Token {
                token_type: TokenType::LBrace,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::String,
                literal: String::from("foo"),
            },
            Token {
                token_type: TokenType::Colon,
                literal: String::from(":"),
            },
            Token {
                token_type: TokenType::String,
                literal: String::from("bar"),
            },
            Token {
                token_type: TokenType::RBrace,
                literal: String::from("}"),
            },
            Token {
                token_type: TokenType::Eof,
                literal: String::from(""),
            },
        ];

        let mut lexer = Lexer::new(input);
        for token in expected.iter() {
            assert_eq!(lexer.next_token(), *token);
        }
    }
}
