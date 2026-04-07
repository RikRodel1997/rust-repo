use std::{iter::Peekable, slice::Iter};

use crate::lexer::tokens::Token;
use crate::parser::ast::*;

pub struct Parser<'a> {
    pub tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Node, String> {
        let program = self.parse_program()?;

        Ok(program)
    }

    fn parse_program(&mut self) -> Result<Node, String> {
        Ok(Node::Program {
            function: Box::new(self.parse_function()?),
        })
    }

    fn parse_function(&mut self) -> Result<Node, String> {
        self.expect(Token::Int)?;
        self.expect(Token::Identifier("main".into()))?;
        self.expect(Token::OpenParen)?;
        self.expect(Token::Void)?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        Ok(Node::Function {
            name: Box::new(self.parse_identifier()?),
            body: Box::new(self.parse_statement()?),
        })
    }

    fn parse_statement(&mut self) -> Result<Node, String> {
        let token = self
            .tokens
            .next()
            .expect("expected a token in statement, found None");

        let expr = match token {
            Token::Return => Ok(Node::Statement(Statement::Return(Box::new(
                self.parse_expression()?,
            )))),
            _ => Err(format!("unknown token in statement {}", token)),
        };

        self.expect(Token::SemiColon)?;
        self.expect(Token::CloseBrace)?;
        expr
    }

    fn parse_expression(&mut self) -> Result<Node, String> {
        let token = self
            .tokens
            .next()
            .expect("expected a token in expression, found None");

        match token {
            Token::Constant(constant) => Ok(Node::Expression(Expression::Constant(*constant))),
            _ => Err(format!("unknown token in expression {}", token)),
        }
    }

    fn parse_identifier(&mut self) -> Result<Node, String> {
        Ok(Node::Identifier("main".into()))
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        let token = self.tokens.next().expect("expected a token, found None");
        if *token != expected {
            Err(format!("expected {}, found {}", expected, token))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::Lexer;

    #[test]
    fn test_01() {
        fn expected(value: i64) -> Node {
            return Node::Program {
                function: Box::new(Node::Function {
                    name: Box::new(Node::Identifier("main".into())),
                    body: Box::new(Node::Statement(Statement::Return(Box::new(
                        Node::Expression(Expression::Constant(value)),
                    )))),
                }),
            };
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
            let tokens = Lexer::new(&input).lex().expect("lexing failed");
            let actual = Parser::new(&tokens).parse().expect("parsing failed");
            assert_eq!(actual, expected);
        }
    }
}
