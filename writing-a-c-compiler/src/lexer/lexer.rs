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
                '~' => result.push(Token::Tilde),
                '+' => result.push(Token::Plus),
                '*' => result.push(Token::Star),
                '%' => result.push(Token::Modulo),
                '=' => match self.chars.peek().expect("unexpected eof after =") {
                    '=' => {
                        result.push(Token::DoubleEqual);
                        self.chars.next();
                    }
                    _ => result.push(Token::Equal),
                },
                '&' => match self.chars.peek().expect("unexpected eof after &") {
                    '&' => {
                        result.push(Token::DoubleAmpersand);
                        self.chars.next();
                    }
                    _ => result.push(Token::Ampersand),
                },
                '|' => match self.chars.peek().expect("unexpected eof after |") {
                    '|' => {
                        result.push(Token::DoublePipe);
                        self.chars.next();
                    }
                    _ => result.push(Token::Pipe),
                },
                '^' => result.push(Token::Carrot),
                '!' => match self.chars.peek().expect("unexpected eof after !") {
                    '=' => {
                        result.push(Token::BangEqual);
                        self.chars.next();
                    }
                    _ => result.push(Token::Bang),
                },
                '>' => match self.chars.peek().expect("unexpected eof after >") {
                    '>' => {
                        result.push(Token::RShift);
                        self.chars.next();
                    }
                    '=' => {
                        result.push(Token::GreaterThanOrEqual);
                        self.chars.next();
                    }
                    _ => {
                        result.push(Token::GreaterThan);
                        self.chars.next();
                    }
                },
                '<' => match self.chars.peek().expect("unexpected eof after <") {
                    '<' => {
                        result.push(Token::LShift);
                        self.chars.next();
                    }
                    '=' => {
                        result.push(Token::LessThanOrEqual);
                        self.chars.next();
                    }
                    _ => {
                        result.push(Token::LessThan);
                        self.chars.next();
                    }
                },
                '-' => match self.chars.peek().expect("unexpected eof after -") {
                    '-' => {
                        result.push(Token::DoubleHyphen);
                        self.chars.next();
                    }
                    _ => result.push(Token::Hyphen),
                },
                '/' => match self.chars.peek().expect("unexpected eof after /") {
                    '/' => {
                        self.skip_comment();
                    }
                    _ => result.push(Token::ForwardSlash),
                },
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
    use super::*;

    #[test]
    fn test_full_function() {
        let input = "int main ( void ) { return 100 ; }";
        let tokens = Lexer::new(&input).lex().expect("lexing failed");

        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".into()),
                Token::OpenParen,
                Token::Void,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Return,
                Token::Constant(100),
                Token::SemiColon,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn test_negate() {
        let input = "int main ( void ) { return -100; }";
        let tokens = Lexer::new(&input).lex().expect("lexing failed");

        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".into()),
                Token::OpenParen,
                Token::Void,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Return,
                Token::Hyphen,
                Token::Constant(100),
                Token::SemiColon,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn test_complement() {
        let input = "int main ( void ) { return ~100; }";
        let tokens = Lexer::new(&input).lex().expect("lexing failed");

        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".into()),
                Token::OpenParen,
                Token::Void,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Return,
                Token::Tilde,
                Token::Constant(100),
                Token::SemiColon,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn test_simple_binary() {
        let input = "int main ( void ) { return 100 + 300; }";
        let tokens = Lexer::new(&input).lex().expect("lexing failed");

        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".into()),
                Token::OpenParen,
                Token::Void,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Return,
                Token::Constant(100),
                Token::Plus,
                Token::Constant(300),
                Token::SemiColon,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn test_grouped() {
        let input = "int main ( void ) { return (6 + 4) * 3 - (5 + 1); }";
        let tokens = Lexer::new(&input).lex().expect("lexing failed");

        assert_eq!(
            tokens,
            vec![
                Token::Int,
                Token::Identifier("main".into()),
                Token::OpenParen,
                Token::Void,
                Token::CloseParen,
                Token::OpenBrace,
                Token::Return,
                Token::OpenParen,
                Token::Constant(6),
                Token::Plus,
                Token::Constant(4),
                Token::CloseParen,
                Token::Star,
                Token::Constant(3),
                Token::Hyphen,
                Token::OpenParen,
                Token::Constant(5),
                Token::Plus,
                Token::Constant(1),
                Token::CloseParen,
                Token::SemiColon,
                Token::CloseBrace,
            ]
        );
    }

    #[test]
    fn test_bitwise_and() {
        let tokens = Lexer::new(&"3 & 5").lex().expect("lexing failed");
        assert_eq!(
            tokens,
            [Token::Constant(3), Token::Ampersand, Token::Constant(5)]
        )
    }

    #[test]
    fn test_bitwise_or() {
        let tokens = Lexer::new(&"1 | 2").lex().expect("lexing failed");
        assert_eq!(
            tokens,
            [Token::Constant(1), Token::Pipe, Token::Constant(2)]
        )
    }

    #[test]
    fn test_bitwise_xor() {
        let tokens = Lexer::new(&"7 ^ 1").lex().expect("lexing failed");
        assert_eq!(
            tokens,
            [Token::Constant(7), Token::Carrot, Token::Constant(1)]
        )
    }

    #[test]
    fn test_bitwise_left_shift() {
        let tokens = Lexer::new(&"35 << 2").lex().expect("lexing failed");
        assert_eq!(
            tokens,
            [Token::Constant(35), Token::LShift, Token::Constant(2)]
        )
    }

    #[test]
    fn test_bitwise_right_shift() {
        let tokens = Lexer::new(&"1000 >> 4").lex().expect("lexing failed");
        assert_eq!(
            tokens,
            [Token::Constant(1000), Token::RShift, Token::Constant(4)]
        )
    }

    #[test]
    fn test_bitwise_right_shift_negative() {
        let tokens = Lexer::new(&"-5 >> 30").lex().expect("lexing failed");
        assert_eq!(
            tokens,
            [
                Token::Hyphen,
                Token::Constant(5),
                Token::RShift,
                Token::Constant(30)
            ]
        )
    }

    #[test]
    fn test_bitwise_precedence() {
        let tokens = Lexer::new(&"40 << 4 + 12 >> 1")
            .lex()
            .expect("lexing failed");

        assert_eq!(
            tokens,
            [
                Token::Constant(40),
                Token::LShift,
                Token::Constant(4),
                Token::Plus,
                Token::Constant(12),
                Token::RShift,
                Token::Constant(1)
            ]
        )
    }

    #[test]
    fn test_bangs() {
        let tokens = Lexer::new(&"!5 ! 30").lex().expect("lexing failed");
        assert_eq!(
            tokens,
            [
                Token::Bang,
                Token::Constant(5),
                Token::Bang,
                Token::Constant(30)
            ]
        )
    }

    #[test]
    fn test_double_tokens() {
        let tokens = Lexer::new(&"!!=5 != 30 && || ==")
            .lex()
            .expect("lexing failed");

        assert_eq!(
            tokens,
            [
                Token::Bang,
                Token::BangEqual,
                Token::Constant(5),
                Token::BangEqual,
                Token::Constant(30),
                Token::DoubleAmpersand,
                Token::DoublePipe,
                Token::DoubleEqual,
            ]
        )
    }

    #[test]
    fn test_lt_lte_gt_gte() {
        let tokens = Lexer::new(&"3 < 4 <= 4 > 3 >= 3")
            .lex()
            .expect("lexing failed");

        assert_eq!(
            tokens,
            [
                Token::Constant(3),
                Token::LessThan,
                Token::Constant(4),
                Token::LessThanOrEqual,
                Token::Constant(4),
                Token::GreaterThan,
                Token::Constant(3),
                Token::GreaterThanOrEqual,
                Token::Constant(3),
            ]
        )
    }
}
