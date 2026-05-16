use crate::{
    parser::{Expression, Node, Statement},
    tacky::{TackyInstruction, TackyNode, TackyUnaryOperator, TackyValue},
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
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::Lexer;
    use crate::Parser;

    #[test]
    fn test_02() {
        fn new_complement(src: TackyValue, dst: TackyValue) -> TackyInstruction {
            TackyInstruction::Unary {
                operator: TackyUnaryOperator::Complement,
                src,
                dst,
            }
        }

        fn new_negate(src: TackyValue, dst: TackyValue) -> TackyInstruction {
            TackyInstruction::Unary {
                operator: TackyUnaryOperator::Negate,
                src,
                dst,
            }
        }

        fn new_return(val: TackyValue) -> TackyInstruction {
            TackyInstruction::Return(val)
        }

        fn expected(instructions: Vec<TackyInstruction>) -> TackyNode {
            TackyNode::Program {
                function: Box::new(TackyNode::Function {
                    name: "main".into(),
                    instructions,
                }),
            }
        }

        let tests = vec![
            (
                "bitwise_int_min.c",
                expected(vec![
                    new_negate(
                        TackyValue::Constant(2147483647),
                        TackyValue::Var("tmp.0".into()),
                    ),
                    new_complement(
                        TackyValue::Var("tmp.0".into()),
                        TackyValue::Var("tmp.1".into()),
                    ),
                    new_return(TackyValue::Var("tmp.1".into())),
                ]),
            ),
            (
                "bitwise_zero.c",
                expected(vec![
                    new_complement(TackyValue::Constant(0), TackyValue::Var("tmp.0".into())),
                    new_return(TackyValue::Var("tmp.0".into())),
                ]),
            ),
            (
                "bitwise.c",
                expected(vec![
                    new_complement(TackyValue::Constant(12), TackyValue::Var("tmp.0".into())),
                    new_return(TackyValue::Var("tmp.0".into())),
                ]),
            ),
            (
                "neg_zero.c",
                expected(vec![
                    new_negate(TackyValue::Constant(0), TackyValue::Var("tmp.0".into())),
                    new_return(TackyValue::Var("tmp.0".into())),
                ]),
            ),
            (
                "neg.c",
                expected(vec![
                    new_negate(TackyValue::Constant(5), TackyValue::Var("tmp.0".into())),
                    new_return(TackyValue::Var("tmp.0".into())),
                ]),
            ),
            (
                "negate_int_max.c",
                expected(vec![
                    new_negate(
                        TackyValue::Constant(2147483647),
                        TackyValue::Var("tmp.0".into()),
                    ),
                    new_return(TackyValue::Var("tmp.0".into())),
                ]),
            ),
            (
                "nested_ops_2.c",
                expected(vec![
                    new_complement(TackyValue::Constant(0), TackyValue::Var("tmp.0".into())),
                    new_negate(
                        TackyValue::Var("tmp.0".into()),
                        TackyValue::Var("tmp.1".into()),
                    ),
                    new_return(TackyValue::Var("tmp.1".into())),
                ]),
            ),
            (
                "nested_ops.c",
                expected(vec![
                    new_negate(TackyValue::Constant(3), TackyValue::Var("tmp.0".into())),
                    new_complement(
                        TackyValue::Var("tmp.0".into()),
                        TackyValue::Var("tmp.1".into()),
                    ),
                    new_return(TackyValue::Var("tmp.1".into())),
                ]),
            ),
            (
                "parens_2.c",
                expected(vec![
                    new_complement(TackyValue::Constant(2), TackyValue::Var("tmp.0".into())),
                    new_return(TackyValue::Var("tmp.0".into())),
                ]),
            ),
            (
                "parens_3.c",
                expected(vec![
                    new_negate(TackyValue::Constant(4), TackyValue::Var("tmp.0".into())),
                    new_negate(
                        TackyValue::Var("tmp.0".into()),
                        TackyValue::Var("tmp.1".into()),
                    ),
                    new_return(TackyValue::Var("tmp.1".into())),
                ]),
            ),
            (
                "parens.c",
                expected(vec![
                    new_negate(TackyValue::Constant(2), TackyValue::Var("tmp.0".into())),
                    new_return(TackyValue::Var("tmp.0".into())),
                ]),
            ),
            (
                "redundant_parens.c",
                expected(vec![
                    new_negate(TackyValue::Constant(10), TackyValue::Var("tmp.0".into())),
                    new_return(TackyValue::Var("tmp.0".into())),
                ]),
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex().expect("lexing failed");
            let ast = Parser::new(&tokens).parse().expect("parsing failed");
            let actual = TackyParser::new(&ast)
                .parse()
                .expect("tacky parsing failed");

            assert_eq!(actual, expected);
        }
    }
}
