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
                self.parse_expression(0)?,
            )))),
            _ => Err(format!("unknown token in statement {}", token)),
        };

        self.expect(Token::SemiColon)?;
        self.expect(Token::CloseBrace)?;
        expr
    }

    fn parse_expression(&mut self, min_precedence: i8) -> Result<Node, String> {
        let mut left = Box::new(self.parse_factor()?);

        while let Some(token) = self.tokens.peek() {
            let token_precedence = Self::precedence(token);
            if token_precedence > min_precedence {
                let op = self.tokens.next().unwrap();

                left = Box::new(Node::Expression(Expression::Binary {
                    operator: BinaryOperator::try_from(op)?,
                    left,
                    right: Box::new(self.parse_expression(token_precedence + 1)?),
                }));
            } else {
                break;
            }
        }

        Ok(*left)
    }

    fn parse_factor(&mut self) -> Result<Node, String> {
        let token = self.tokens.next().expect("expected a token in factor");

        match token {
            Token::Constant(constant) => Ok(Node::Expression(Expression::Constant(*constant))),
            Token::Hyphen | Token::Tilde => {
                let operator = UnaryOperator::try_from(token)?;
                let expression = Box::new(self.parse_factor()?);
                Ok(Node::Expression(Expression::Unary {
                    operator,
                    expression,
                }))
            }
            Token::OpenParen => {
                let expression = self.parse_expression(0)?;
                self.expect(Token::CloseParen)?;
                Ok(expression)
            }
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

    fn precedence(token: &'a Token) -> i8 {
        match token {
            Token::Star | Token::ForwardSlash | Token::Modulo => 100,
            Token::Plus | Token::Hyphen => 95,
            Token::LShift | Token::RShift => 85,
            Token::Ampersand => 80,
            Token::Carrot => 75,
            Token::Pipe => 70,
            _ => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(tokens: Vec<Token>) -> Vec<Token> {
        let mut result = vec![
            Token::Int,
            Token::Identifier("main".into()),
            Token::OpenParen,
            Token::Void,
            Token::CloseParen,
            Token::OpenBrace,
            Token::Return,
        ];
        result.extend(tokens);
        result.extend(vec![Token::SemiColon, Token::CloseBrace]);
        result
    }

    fn expected(body: Node) -> Node {
        Node::Program {
            function: Box::new(Node::Function {
                name: Box::new(Node::Identifier("main".into())),
                body: Box::new(body),
            }),
        }
    }

    fn binary(operator: BinaryOperator, left: Expression, right: Expression) -> Expression {
        Expression::Binary {
            operator,
            left: Box::new(Node::Expression(left)),
            right: Box::new(Node::Expression(right)),
        }
    }

    #[test]
    fn test_nested_binary() -> Result<(), String> {
        let tokens = input(vec![
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
        ]);

        let ast = Parser::new(&tokens).parse().expect("parsing failed");

        let expected = expected(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::Subtract,
                binary(
                    BinaryOperator::Multiply,
                    binary(
                        BinaryOperator::Add,
                        Expression::Constant(6),
                        Expression::Constant(4),
                    ),
                    Expression::Constant(3),
                ),
                binary(
                    BinaryOperator::Add,
                    Expression::Constant(5),
                    Expression::Constant(1),
                ),
            )),
        ))));

        assert_eq!(ast, expected);

        Ok(())
    }

    #[test]
    fn test_bitwise_or() -> Result<(), String> {
        let tokens = input(vec![Token::Constant(3), Token::Pipe, Token::Constant(5)]);
        let ast = Parser::new(&tokens).parse().expect("parsing failed");

        let expected = expected(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::Or,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        assert_eq!(ast, expected);
        Ok(())
    }

    #[test]
    fn test_bitwise_precedence() -> Result<(), String> {
        let tokens = input(vec![
            Token::Constant(3),
            Token::Carrot,
            Token::Constant(5),
            Token::Ampersand,
            Token::Constant(12),
            Token::LShift,
            Token::Constant(100),
        ]);
        let ast = Parser::new(&tokens).parse().expect("parsing failed");

        let expected = expected(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::Xor,
                Expression::Constant(3),
                binary(
                    BinaryOperator::And,
                    Expression::Constant(5),
                    binary(
                        BinaryOperator::LShift,
                        Expression::Constant(12),
                        Expression::Constant(100),
                    ),
                ),
            )),
        ))));

        assert_eq!(ast, expected);
        Ok(())
    }

    #[test]
    fn test_bitwise_arithmetic_precedence() -> Result<(), String> {
        let tokens = input(vec![
            Token::Constant(3),
            Token::Plus,
            Token::OpenParen,
            Token::Tilde,
            Token::Constant(5),
            Token::Pipe,
            Token::Constant(12),
            Token::CloseParen,
            Token::LShift,
            Token::Constant(100),
        ]);
        let ast = Parser::new(&tokens).parse().expect("parsing failed");

        let expected = expected(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::LShift,
                binary(
                    BinaryOperator::Add,
                    Expression::Constant(3),
                    binary(
                        BinaryOperator::Or,
                        Expression::Unary {
                            operator: UnaryOperator::Complement,
                            expression: Box::new(Node::Expression(Expression::Constant(5))),
                        },
                        Expression::Constant(12),
                    ),
                ),
                Expression::Constant(100),
            )),
        ))));

        assert_eq!(ast, expected);
        Ok(())
    }
}
