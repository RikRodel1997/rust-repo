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
            Token::Hyphen | Token::Tilde => {
                let operator = UnaryOperator::try_from(token)?;
                let expression = Box::new(self.parse_expression()?);
                Ok(Node::Expression(Expression::Unary {
                    operator,
                    expression,
                }))
            }
            Token::OpenParen => {
                let expression = self.parse_expression()?;
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
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::Lexer;

    fn program_node(body: Node) -> Node {
        Node::Program {
            function: Box::new(Node::Function {
                name: Box::new(Node::Identifier("main".into())),
                body: Box::new(body),
            }),
        }
    }

    #[test]
    fn test_01() -> Result<(), String> {
        fn expected(value: i64) -> Node {
            program_node(Node::Statement(Statement::Return(Box::new(
                Node::Expression(Expression::Constant(value)),
            ))))
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
            let tokens = Lexer::new(&input).lex()?;
            let actual = Parser::new(&tokens).parse()?;
            assert_eq!(actual, expected);
        }

        Ok(())
    }

    #[test]
    fn test_02() -> Result<(), String> {
        fn expected(value: Expression) -> Node {
            program_node(Node::Statement(Statement::Return(Box::new(
                Node::Expression(value),
            ))))
        }

        let tests = vec![
            (
                "bitwise_int_min.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Complement,
                    expression: Box::new(Node::Expression(Expression::Unary {
                        operator: UnaryOperator::Negate,
                        expression: Box::new(Node::Expression(Expression::Constant(2147483647))),
                    })),
                }),
            ),
            (
                "bitwise_zero.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Complement,
                    expression: Box::new(Node::Expression(Expression::Constant(0))),
                }),
            ),
            (
                "bitwise.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Complement,
                    expression: Box::new(Node::Expression(Expression::Constant(12))),
                }),
            ),
            (
                "neg_zero.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Negate,
                    expression: Box::new(Node::Expression(Expression::Constant(0))),
                }),
            ),
            (
                "neg.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Negate,
                    expression: Box::new(Node::Expression(Expression::Constant(5))),
                }),
            ),
            (
                "negate_int_max.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Negate,
                    expression: Box::new(Node::Expression(Expression::Constant(2147483647))),
                }),
            ),
            (
                "nested_ops_2.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Negate,
                    expression: Box::new(Node::Expression(Expression::Unary {
                        operator: UnaryOperator::Complement,
                        expression: Box::new(Node::Expression(Expression::Constant(0))),
                    })),
                }),
            ),
            (
                "nested_ops.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Complement,
                    expression: Box::new(Node::Expression(Expression::Unary {
                        operator: UnaryOperator::Negate,
                        expression: Box::new(Node::Expression(Expression::Constant(3))),
                    })),
                }),
            ),
            (
                "parens_2.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Complement,
                    expression: Box::new(Node::Expression(Expression::Constant(2))),
                }),
            ),
            (
                "parens_3.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Negate,
                    expression: Box::new(Node::Expression(Expression::Unary {
                        operator: UnaryOperator::Negate,
                        expression: Box::new(Node::Expression(Expression::Constant(4))),
                    })),
                }),
            ),
            (
                "parens.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Negate,
                    expression: Box::new(Node::Expression(Expression::Constant(2))),
                }),
            ),
            (
                "redundant_parens.c",
                expected(Expression::Unary {
                    operator: UnaryOperator::Negate,
                    expression: Box::new(Node::Expression(Expression::Constant(10))),
                }),
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let actual = Parser::new(&tokens).parse()?;
            assert_eq!(actual, expected);
        }

        Ok(())
    }
}
