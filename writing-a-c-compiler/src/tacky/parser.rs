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

                    instructions.push(TackyInstruction::Unary {
                        operator: TackyUnaryOperator::from(operator),
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

                    instructions.push(TackyInstruction::Binary {
                        operator: TackyBinaryOperator::from(operator),
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
    use super::*;
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
}
