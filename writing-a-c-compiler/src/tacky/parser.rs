use crate::{
    parser::{Expression, Node, Statement},
    tacky::{TackyBinOp, TackyInstruction, TackyNode, TackyUnOp, TackyValue},
};

pub struct TackyParser<'a> {
    pub ast: &'a Node,
    pub temp_count: u32,
    pub temp_false_count: u32,
    pub temp_true_count: u32,
    pub temp_end_count: u32,
}

impl<'a> TackyParser<'a> {
    pub fn new(ast: &'a Node) -> Self {
        Self {
            ast,
            temp_count: 0,
            temp_false_count: 0,
            temp_true_count: 0,
            temp_end_count: 0,
        }
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
                        operator: TackyUnOp::from(operator),
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
                    let operator = TackyBinOp::from(operator);

                    match operator {
                        TackyBinOp::DoubleAmpersand => {
                            let dst = {
                                let tmp = format!("tmp.{}", self.temp_count);
                                self.temp_count += 1;
                                TackyValue::Var(tmp)
                            };

                            let false_label = format!("tmp_false.{}", self.temp_false_count);
                            self.temp_false_count += 1;

                            let end_label = format!("tmp_end.{}", self.temp_end_count);
                            self.temp_end_count += 1;

                            let left = self.emit_expression(&left, instructions)?;
                            instructions.push(TackyInstruction::JumpIfZero {
                                condition: left,
                                target: false_label.clone(),
                            });

                            let right = self.emit_expression(&right, instructions)?;
                            instructions.push(TackyInstruction::JumpIfZero {
                                condition: right,
                                target: false_label.clone(),
                            });

                            instructions.push(TackyInstruction::Copy {
                                src: TackyValue::Constant(1),
                                dst: dst.clone(),
                            });

                            instructions.push(TackyInstruction::Jump {
                                target: end_label.clone(),
                            });

                            instructions.push(TackyInstruction::Label(false_label.clone()));

                            instructions.push(TackyInstruction::Copy {
                                src: TackyValue::Constant(0),
                                dst: dst.clone(),
                            });

                            instructions.push(TackyInstruction::Label(end_label.clone()));
                            Ok(dst)
                        }
                        TackyBinOp::DoublePipe => {
                            let dst = {
                                let tmp = format!("tmp.{}", self.temp_count);
                                self.temp_count += 1;
                                TackyValue::Var(tmp)
                            };

                            let true_label = format!("tmp_true.{}", self.temp_true_count);
                            self.temp_true_count += 1;

                            let end_label = format!("tmp_end.{}", self.temp_end_count);
                            self.temp_end_count += 1;

                            let left = self.emit_expression(&left, instructions)?;
                            instructions.push(TackyInstruction::JumpIfNotZero {
                                condition: left,
                                target: true_label.clone(),
                            });

                            let right = self.emit_expression(&right, instructions)?;
                            instructions.push(TackyInstruction::JumpIfNotZero {
                                condition: right,
                                target: true_label.clone(),
                            });

                            instructions.push(TackyInstruction::Copy {
                                src: TackyValue::Constant(0),
                                dst: dst.clone(),
                            });

                            instructions.push(TackyInstruction::Jump {
                                target: end_label.clone(),
                            });

                            instructions.push(TackyInstruction::Label(true_label.clone()));

                            instructions.push(TackyInstruction::Copy {
                                src: TackyValue::Constant(1),
                                dst: dst.clone(),
                            });

                            instructions.push(TackyInstruction::Label(end_label.clone()));
                            Ok(dst)
                        }
                        _ => {
                            let src_left = self.emit_expression(&left, instructions)?;
                            let src_right = self.emit_expression(&right, instructions)?;
                            let dst = {
                                let tmp = format!("tmp.{}", self.temp_count);
                                self.temp_count += 1;
                                TackyValue::Var(tmp)
                            };

                            instructions.push(TackyInstruction::Binary {
                                operator,
                                src1: src_left,
                                src2: src_right,
                                dst: dst.clone(),
                            });
                            Ok(dst)
                        }
                    }
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
    use crate::parser::{BinOp, UnOp};

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

    fn binary(operator: BinOp, left: Expression, right: Expression) -> Expression {
        Expression::Binary {
            operator,
            left: Box::new(Node::Expression(left)),
            right: Box::new(Node::Expression(right)),
        }
    }

    fn unary(operator: UnOp, expression: Expression) -> Expression {
        Expression::Unary {
            operator,
            expression: Box::new(Node::Expression(expression)),
        }
    }

    #[test]
    fn test_less_than_or_equal() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinOp::LessThanOrEqual,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::LessThanOrEqual,
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
    fn test_not() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(unary(UnOp::Not, Expression::Constant(3))),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Unary {
                operator: TackyUnOp::Not,
                src: TackyValue::Constant(3),
                dst: TackyValue::Var("tmp.0".into()),
            },
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);

        assert_eq!(ast, expected);

        Ok(())
    }

    #[test]
    fn test_double_ampersand() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinOp::DoubleAmpersand,
                Expression::Constant(0),
                Expression::Constant(0),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::JumpIfZero {
                condition: TackyValue::Constant(0),
                target: "tmp_false.0".to_string(),
            },
            TackyInstruction::JumpIfZero {
                condition: TackyValue::Constant(0),
                target: "tmp_false.0".to_string(),
            },
            TackyInstruction::Copy {
                src: TackyValue::Constant(1),
                dst: TackyValue::Var("tmp.0".to_string()),
            },
            TackyInstruction::Jump {
                target: "tmp_end.0".to_string(),
            },
            TackyInstruction::Label("tmp_false.0".to_string()),
            TackyInstruction::Copy {
                src: TackyValue::Constant(0),
                dst: TackyValue::Var("tmp.0".to_string()),
            },
            TackyInstruction::Label("tmp_end.0".to_string()),
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);

        assert_eq!(ast, expected);

        Ok(())
    }

    #[test]
    fn test_double_pipe() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinOp::DoublePipe,
                Expression::Constant(0),
                Expression::Constant(0),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::JumpIfNotZero {
                condition: TackyValue::Constant(0),
                target: "tmp_true.0".to_string(),
            },
            TackyInstruction::JumpIfNotZero {
                condition: TackyValue::Constant(0),
                target: "tmp_true.0".to_string(),
            },
            TackyInstruction::Copy {
                src: TackyValue::Constant(0),
                dst: TackyValue::Var("tmp.0".to_string()),
            },
            TackyInstruction::Jump {
                target: "tmp_end.0".to_string(),
            },
            TackyInstruction::Label("tmp_true.0".to_string()),
            TackyInstruction::Copy {
                src: TackyValue::Constant(1),
                dst: TackyValue::Var("tmp.0".to_string()),
            },
            TackyInstruction::Label("tmp_end.0".to_string()),
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);

        assert_eq!(ast, expected);

        Ok(())
    }

    #[test]
    fn test_equal() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinOp::Equal,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::Equal,
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
    fn test_bitwise_or() -> Result<(), String> {
        let ast = input(Node::Statement(Statement::Return(Box::new(
            Node::Expression(binary(
                BinOp::Or,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::Or,
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
                BinOp::Xor,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::Xor,
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
                BinOp::LShift,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::LShift,
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
                BinOp::RShift,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::RShift,
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
                BinOp::And,
                Expression::Constant(3),
                Expression::Constant(5),
            )),
        ))));

        let ast = TackyParser::new(&ast).parse().expect("parsing failed");

        let expected = expected(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::And,
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
