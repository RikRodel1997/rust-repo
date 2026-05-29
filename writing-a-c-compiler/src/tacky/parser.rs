use crate::{
    parser::{Expression, Node, Statement},
    tacky::{TackyBinaryOperator, TackyInstruction, TackyNode, TackyUnaryOperator, TackyValue},
};

pub struct TackyParser<'a> {
    pub ast: &'a Node,
    pub temp_count: u32,
}

impl<'a> TackyParser<'a> {
    pub fn new(ast: &'a Node) -> Self {
        Self { ast, temp_count: 0 }
    }

    pub fn parse(&mut self) -> Result<TackyNode, String> {
        if let Node::Program { function } = &self.ast {
            Ok(TackyNode::Program {
                function: Box::new(self.to_tacky_node(&*function)?),
            })
        } else {
            Err(format!("expected Program, got {}", self.ast))
        }
    }

    fn to_tacky_node(&mut self, node: &Node) -> Result<TackyNode, String> {
        match node {
            Node::Function { name, body } => {
                if let Node::Identifier(value) = &**name {
                    Ok(TackyNode::Function {
                        name: value.to_string(),
                        instructions: self.to_tacky_instructions(&**body)?,
                    })
                } else {
                    Err(format!("expected Function name to be Ident, got {}", name))
                }
            }
            _ => Err(format!("unknown node to parse to ASM node {}", node)),
        }
    }

    fn to_tacky_instructions(&mut self, node: &Node) -> Result<Vec<TackyInstruction>, String> {
        let mut instructions = vec![];
        match node {
            Node::Statement(Statement::Return(expression)) => match &**expression {
                Node::Expression(expression) => {
                    let value = self.emit_expression(expression, &mut instructions)?;
                    instructions.push(TackyInstruction::Return(value));
                }
                _ => return Err(format!("unable to convert {node} to TACKY instruction")),
            },
            _ => return Err(format!("unable to convert {node} to TACKY instruction")),
        };
        Ok(instructions)
    }

    fn emit_expression(
        &mut self,
        expression: &Expression,
        instructions: &mut Vec<TackyInstruction>,
    ) -> Result<TackyValue, String> {
        match expression {
            Expression::Constant(value) => Ok(TackyValue::Constant(*value)),
            Expression::Unary {
                operator,
                expression,
            } => {
                if let Node::Expression(expression) = &**expression {
                    let inner_value = self.emit_expression(&expression, instructions)?;
                    let dst = {
                        let tmp = format!("tmp.{}", self.temp_count);
                        self.temp_count += 1;
                        TackyValue::Var(tmp)
                    };

                    let operator = TackyUnaryOperator::try_from(operator)
                        .expect("unexpected unary operator in TACKY");
                    instructions.push(TackyInstruction::Unary {
                        operator,
                        src: inner_value,
                        dst: dst.clone(),
                    });
                    Ok(dst)
                } else {
                    return Err(format!("{expression} is not a valid expression"));
                }
            }
            Expression::Binary {
                operator,
                left,
                right,
            } => {
                if let (Node::Expression(left), Node::Expression(right)) = (&**left, &**right) {
                    let src_left = self.emit_expression(&left, instructions)?;
                    let src_right = self.emit_expression(&right, instructions)?;
                    let dst = {
                        let tmp = format!("tmp.{}", self.temp_count);
                        self.temp_count += 1;
                        TackyValue::Var(tmp)
                    };

                    let operator = TackyBinaryOperator::try_from(operator)
                        .expect("unexpected binary operator in TACKY");
                    instructions.push(TackyInstruction::Binary {
                        operator,
                        src1: src_left,
                        src2: src_right,
                        dst: dst.clone(),
                    });
                    Ok(dst)
                } else {
                    return Err(format!("{left} or {right} is not a valid expression"));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::Lexer;
    use crate::Parser;
    use crate::parser::BinaryOperator;

    fn input(body: Node) -> Node {
        Node::Program {
            function: Box::new(Node::Function {
                name: Box::new(Node::Identifier("main".into())),
                body: Box::new(body),
            }),
        }
    }

    fn expected(instructions: Vec<TackyInstruction>) -> TackyNode {
        TackyNode::Program {
            function: Box::new(TackyNode::Function {
                name: "main".into(),
                instructions,
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
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::Or,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinaryOperator::Or,
                src1: TackyValue::Constant(3),
                src2: TackyValue::Constant(5),
                dst: TackyValue::Var("tmp.0".into()),
            },
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);

        assert_eq!(ast, expected);
        Ok(())
    }

    #[test]
    fn test_bitwise_xor() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::Xor,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinaryOperator::Xor,
                src1: TackyValue::Constant(3),
                src2: TackyValue::Constant(5),
                dst: TackyValue::Var("tmp.0".into()),
            },
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);

        assert_eq!(ast, expected);
        Ok(())
    }

    #[test]
    fn test_bitwise_lshift() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::LShift,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinaryOperator::LShift,
                src1: TackyValue::Constant(3),
                src2: TackyValue::Constant(5),
                dst: TackyValue::Var("tmp.0".into()),
            },
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);

        assert_eq!(ast, expected);

        Ok(())
    }

    #[test]
    fn test_bitwise_rshift() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::RShift,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinaryOperator::RShift,
                src1: TackyValue::Constant(3),
                src2: TackyValue::Constant(5),
                dst: TackyValue::Var("tmp.0".into()),
            },
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);

        assert_eq!(ast, expected);

        Ok(())
    }

    #[test]
    fn test_bitwise_and() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinaryOperator::And,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinaryOperator::And,
                src1: TackyValue::Constant(3),
                src2: TackyValue::Constant(5),
                dst: TackyValue::Var("tmp.0".into()),
            },
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);

        assert_eq!(ast, expected);

        Ok(())
    }

    #[test]
    fn test_02() {
        fn expected(variance: &str) -> String {
            format!("prog fn main {variance}")
        }

        let tests = vec![
            (
                "bitwise_int_min.c",
                expected("-2147483647 > tmp.0 ~tmp.0 > tmp.1 ret tmp.1"),
            ),
            ("bitwise_zero.c", expected("~0 > tmp.0 ret tmp.0")),
            ("bitwise.c", expected("~12 > tmp.0 ret tmp.0")),
            ("neg_zero.c", expected("-0 > tmp.0 ret tmp.0")),
            ("neg.c", expected("-5 > tmp.0 ret tmp.0")),
            (
                "negate_int_max.c",
                expected("-2147483647 > tmp.0 ret tmp.0"),
            ),
            (
                "nested_ops_2.c",
                expected("~0 > tmp.0 -tmp.0 > tmp.1 ret tmp.1"),
            ),
            (
                "nested_ops.c",
                expected("-3 > tmp.0 ~tmp.0 > tmp.1 ret tmp.1"),
            ),
            ("parens_2.c", expected("~2 > tmp.0 ret tmp.0")),
            (
                "parens_3.c",
                expected("-4 > tmp.0 -tmp.0 > tmp.1 ret tmp.1"),
            ),
            ("parens.c", expected("-2 > tmp.0 ret tmp.0")),
            ("redundant_parens.c", expected("-10 > tmp.0 ret tmp.0")),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex().expect("lexing failed");
            let ast = Parser::new(&tokens).parse().expect("parsing failed");
            let actual = TackyParser::new(&ast)
                .parse()
                .expect("tacky parsing failed");

            assert_eq!(actual.to_string(), expected);
        }
    }

    #[test]
    fn test_03() -> Result<(), String> {
        fn expected(variance: &str) -> String {
            format!("prog fn main {variance}")
        }

        let tests = vec![
            ("add.c", expected("+ 1 2 > tmp.0 ret tmp.0")),
            (
                "associativity_2.c",
                expected("/ 6 3 > tmp.0 / tmp.0 2 > tmp.1 ret tmp.1"),
            ),
            (
                "associativity_3.c",
                expected(
                    "/ 3 2 > tmp.0 * tmp.0 4 > tmp.1 - 5 4 > tmp.2 + tmp.2 3 > tmp.3 + tmp.1 tmp.3 > tmp.4 ret tmp.4",
                ),
            ),
            (
                "associativity_and_precedence.c",
                expected(
                    "* 5 4 > tmp.0 / tmp.0 2 > tmp.1 + 2 1 > tmp.2 % 3 tmp.2 > tmp.3 - tmp.1 tmp.3 > tmp.4 ret tmp.4",
                ),
            ),
            (
                "associativity.c",
                expected("- 1 2 > tmp.0 - tmp.0 3 > tmp.1 ret tmp.1"),
            ),
            (
                "div_neg.c",
                expected("-12 > tmp.0 / tmp.0 5 > tmp.1 ret tmp.1"),
            ),
            ("div.c", expected("/ 4 2 > tmp.0 ret tmp.0")),
            ("mod.c", expected("% 4 2 > tmp.0 ret tmp.0")),
            ("mult.c", expected("* 2 3 > tmp.0 ret tmp.0")),
            (
                "parens.c",
                expected("+ 3 4 > tmp.0 * 2 tmp.0 > tmp.1 ret tmp.1"),
            ),
            (
                "precedence.c",
                expected("* 3 4 > tmp.0 + 2 tmp.0 > tmp.1 ret tmp.1"),
            ),
            (
                "sub_neg.c",
                expected("-1 > tmp.0 - 2 tmp.0 > tmp.1 ret tmp.1"),
            ),
            ("sub.c", expected("- 1 2 > tmp.0 ret tmp.0")),
            (
                "unop_add.c",
                expected("~2 > tmp.0 + tmp.0 3 > tmp.1 ret tmp.1"),
            ),
            (
                "unop_parens.c",
                expected("+ 1 1 > tmp.0 ~tmp.0 > tmp.1 ret tmp.1"),
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/03/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex().expect("lexing failed");
            let ast = Parser::new(&tokens).parse().expect("parsing failed");
            let actual = TackyParser::new(&ast)
                .parse()
                .expect("tacky parsing failed");

            assert_eq!(actual.to_string(), expected);
        }

        Ok(())
    }
}
