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
                '&' => result.push(Token::Ampersand),
                '|' => result.push(Token::Pipe),
                '^' => result.push(Token::Carrot),
                '>' => {
                    let c = self.chars.peek().expect("unexpected eof after >");
                    if *c == '>' {
                        result.push(Token::RShift);
                        self.chars.next();
                    } else {
                        return Err(format!("unexpected character after '>' {c}"));
                    }
                }
                '<' => {
                    let c = self.chars.peek().expect("unexpected eof after <");
                    if *c == '<' {
                        result.push(Token::LShift);
                        self.chars.next();
                    } else {
                        return Err(format!("unexpected character after '<' {c}"));
                    }
                }
                '-' => {
                    let c = self.chars.peek().expect("unexpected eof after -");
                    if *c == '-' {
                        result.push(Token::DoubleHyphen);
                        self.chars.next();
                    } else {
                        result.push(Token::Hyphen)
                    }
                }
                '/' => {
                    let c = self
                        .chars
                        .peek()
                        .expect("unexpected eof after forward slash");

                    if *c == '/' {
                        self.skip_comment();
                    } else {
                        result.push(Token::ForwardSlash);
                    }
                }
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

    fn get_actual(file_path: String) -> String {
        let input = fs::read_to_string(file_path).expect("unable to read file");
        let tokens = Lexer::new(&input).lex().expect("lexing failed");

        tokens
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn test_01() {
        let tests = vec![
            ("multi_digit.c", "int main ( void ) { return 100 ; }"),
            ("newlines.c", "int main ( void ) { return 0 ; }"),
            ("no_newlines.c", "int main ( void ) { return 0 ; }"),
            ("return_0.c", "int main ( void ) { return 0 ; }"),
            ("return_2.c", "int main ( void ) { return 2 ; }"),
            ("spaces.c", "int main ( void ) { return 0 ; }"),
            ("tabs.c", "int main ( void ) { return 0 ; }"),
        ];

        for (file, expected) in tests.into_iter() {
            let actual = get_actual(format!("files/01/{file}"));

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_02() {
        fn expected(variance: &str) -> String {
            let mut expected = String::from("int main ( void ) { return ");
            expected.push_str(variance);
            expected.push_str(" ; }");
            expected
        }

        let tests = vec![
            ("bitwise_int_min.c", expected("~ - 2147483647")),
            ("bitwise_zero.c", expected("~ 0")),
            ("bitwise.c", expected("~ 12")),
            ("neg_zero.c", expected("- 0")),
            ("neg.c", expected("- 5")),
            ("negate_int_max.c", expected("- 2147483647")),
            ("nested_ops_2.c", expected("- ~ 0")),
            ("nested_ops.c", expected("~ - 3")),
            ("parens_2.c", expected("~ ( 2 )")),
            ("parens_3.c", expected("- ( - 4 )")),
            ("parens.c", expected("( - 2 )")),
            ("redundant_parens.c", expected("- ( ( ( ( 10 ) ) ) )")),
        ];

        for (file, expected) in tests.into_iter() {
            let actual = get_actual(format!("files/02/{file}"));

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_03() {
        fn expected(variance: &str) -> String {
            let mut expected = String::from("int main ( void ) { return ");
            expected.push_str(variance);
            expected.push_str(" ; }");
            expected
        }

        let tests = vec![
            ("add.c", expected("1 + 2")),
            ("associativity_2.c", expected("6 / 3 / 2")),
            (
                "associativity_3.c",
                expected("( 3 / 2 * 4 ) + ( 5 - 4 + 3 )"),
            ),
            (
                "associativity_and_precedence.c",
                expected("5 * 4 / 2 - 3 % ( 2 + 1 )"),
            ),
            ("associativity.c", expected("1 - 2 - 3")),
            ("div_neg.c", expected("( - 12 ) / 5")),
            ("div.c", expected("4 / 2")),
            ("mod.c", expected("4 % 2")),
            ("mult.c", expected("2 * 3")),
            ("parens.c", expected("2 * ( 3 + 4 )")),
            ("precedence.c", expected("2 + 3 * 4")),
            ("sub_neg.c", expected("2 - - 1")),
            ("sub.c", expected("1 - 2")),
            ("unop_add.c", expected("~ 2 + 3")),
            ("unop_parens.c", expected("~ ( 1 + 1 )")),
        ];

        for (file, expected) in tests.into_iter() {
            let actual = get_actual(format!("files/03/{file}"));

            assert_eq!(actual, expected);
        }
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
}
