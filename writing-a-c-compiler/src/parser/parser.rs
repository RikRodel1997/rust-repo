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
    use std::fs;

    use super::*;
    use crate::Lexer;

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

    #[test]
    fn test_01() -> Result<(), String> {
        let tests = vec![
            ("multi_digit.c", "prog fn main ret 100"),
            ("newlines.c", "prog fn main ret 0"),
            ("no_newlines.c", "prog fn main ret 0"),
            ("return_0.c", "prog fn main ret 0"),
            ("return_2.c", "prog fn main ret 2"),
            ("spaces.c", "prog fn main ret 0"),
            ("tabs.c", "prog fn main ret 0"),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/01/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let actual = Parser::new(&tokens).parse()?;

            assert_eq!(actual.to_string(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_02() -> Result<(), String> {
        let tests = vec![
            ("bitwise_int_min.c", "prog fn main ret ~-2147483647"),
            ("bitwise_zero.c", "prog fn main ret ~0"),
            ("bitwise.c", "prog fn main ret ~12"),
            ("neg_zero.c", "prog fn main ret -0"),
            ("neg.c", "prog fn main ret -5"),
            ("negate_int_max.c", "prog fn main ret -2147483647"),
            ("nested_ops_2.c", "prog fn main ret -~0"),
            ("nested_ops.c", "prog fn main ret ~-3"),
            ("parens_2.c", "prog fn main ret ~2"),
            ("parens_3.c", "prog fn main ret --4"),
            ("parens.c", "prog fn main ret -2"),
            ("redundant_parens.c", "prog fn main ret -10"),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let actual = Parser::new(&tokens).parse()?;

            assert_eq!(actual.to_string(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_03_simple() -> Result<(), String> {
        let expected = Node::Program {
            function: Box::new(Node::Function {
                name: Box::new(Node::Identifier("main".into())),
                body: Box::new(Node::Statement(Statement::Return(Box::new(
                    Node::Expression(binary(
                        BinaryOperator::Subtract,
                        binary(
                            BinaryOperator::Multiply,
                            Expression::Constant(1),
                            Expression::Constant(2),
                        ),
                        binary(
                            BinaryOperator::Multiply,
                            Expression::Constant(3),
                            binary(
                                BinaryOperator::Add,
                                Expression::Constant(4),
                                Expression::Constant(5),
                            ),
                        ),
                    )),
                )))),
            }),
        };

        let tokens = Lexer::new("int main(void) { return 1 * 2 - 3 * (4 + 5); }").lex()?;
        let actual = Parser::new(&tokens).parse()?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_03() -> Result<(), String> {
        fn expected(variance: &str) -> String {
            format!("prog fn main ret ({variance})")
        }

        let tests = vec![
            ("add.c", expected("+ 1 2")),
            ("associativity_2.c", expected("/ (/ 6 3) 2")),
            (
                "associativity_3.c",
                expected("+ (* (/ 3 2) 4) (+ (- 5 4) 3)"),
            ),
            (
                "associativity_and_precedence.c",
                expected("- (/ (* 5 4) 2) (% 3 (+ 2 1))"),
            ),
            ("associativity.c", expected("- (- 1 2) 3")),
            ("div_neg.c", expected("/ -12 5")),
            ("div.c", expected("/ 4 2")),
            ("mod.c", expected("% 4 2")),
            ("mult.c", expected("* 2 3")),
            ("parens.c", expected("* 2 (+ 3 4)")),
            ("precedence.c", expected("+ 2 (* 3 4)")),
            ("sub_neg.c", expected("- 2 -1")),
            ("sub.c", expected("- 1 2")),
            ("unop_add.c", expected("+ ~2 3")),
            ("unop_parens.c", "prog fn main ret ~(+ 1 1)".into()),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/03/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let actual = Parser::new(&tokens).parse()?;

            assert_eq!(actual.to_string(), expected);
        }

        Ok(())
    }
}
