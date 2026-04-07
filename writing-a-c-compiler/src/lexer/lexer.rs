use std::{iter::Peekable, str::Chars};

use crate::lexer::tokens::*;

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.trim().chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut result = Vec::new();
        while let Some(c) = self.chars.next() {
            match c {
                ' ' | '\t' | '\n' | '\r' => continue,
                '(' => result.push(Token::OpenParen),
                ')' => result.push(Token::CloseParen),
                '{' => result.push(Token::OpenBrace),
                '}' => result.push(Token::CloseBrace),
                ';' => result.push(Token::SemiColon),
                '/' => self.skip_comment(),
                _ => {
                    if c.is_alphabetic() {
                        result.push(self.identifier(c));
                    } else if c.is_numeric() {
                        result.push(self.constant(c));
                    } else {
                        return Err(format!("Invalid character {c}"));
                    }
                }
            }
        }
        Ok(result)
    }

    fn skip_comment(&mut self) {
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                break;
            }
        }
    }

    fn constant<'b>(&mut self, c: char) -> Token {
        let mut buffer = String::from(c);
        while let Some(ch) = self.chars.peek() {
            if ch.is_numeric() {
                buffer.push(*ch);
                self.chars.next();
                continue;
            }
            break;
        }
        let constant = buffer
            .parse::<i64>()
            .expect("constant is not parseable to i64");
        Token::Constant(constant)
    }

    fn identifier<'b>(&mut self, c: char) -> Token {
        let mut buffer = String::from(c);
        while let Some(ch) = self.chars.peek() {
            if ch.is_alphanumeric() {
                buffer.push(*ch);
                self.chars.next();
                continue;
            }
            break;
        }
        match buffer.as_str() {
            "int" => Token::Int,
            "void" => Token::Void,
            "return" => Token::Return,
            _ => Token::Identifier(buffer),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_01() {
        fn expected(constant: i64) -> Vec<Token> {
            return vec![
                Token::Int,
                Token::Identifier("main".into()),
                Token::OpenParen,
                Token::Void,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Return,
                Token::Constant(constant),
                Token::SemiColon,
                Token::CloseBrace,
            ];
        }

        let tests = vec![
            ("multi_digit.c", expected(100)),
            ("newlines.c", expected(0)),
            ("no_newlines.c", expected(0)),
            ("return_0.c", expected(0)),
            ("return_2.c", expected(2)),
            ("spaces.c", expected(0)),
            ("tabs.c", expected(0)),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/01/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let actual = Lexer::new(&input).lex().expect("lexing failed");
            assert_eq!(actual, expected);
        }
    }
}
