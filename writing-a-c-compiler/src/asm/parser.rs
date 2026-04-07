use crate::{
    asm::ast::{AsmNode, Instruction, Operand},
    parser::{Expression, Node, Statement},
};

pub struct AsmParser<'a> {
    pub ast: &'a Node,
}

impl<'a> AsmParser<'a> {
    pub fn new(ast: &'a Node) -> Self {
        Self { ast }
    }

    pub fn parse(&mut self) -> Result<AsmNode, String> {
        if let Node::Program { function } = &self.ast {
            Ok(AsmNode::Program {
                function: Box::new(self.to_asm_node(*&function)?),
            })
        } else {
            Err(format!("expected Program, got {}", self.ast))
        }
    }

    fn to_asm_node(&self, node: &Node) -> Result<AsmNode, String> {
        match node {
            Node::Function { name, body } => {
                if let Node::Identifier(value) = &**name {
                    Ok(AsmNode::Function {
                        name: value.to_string(),
                        instructions: self.to_asm_instructions(*&body)?,
                    })
                } else {
                    Err(format!("expected Function name to be Ident, got {}", name))
                }
            }
            Node::Expression(expr) => match expr {
                Expression::Constant(value) => Ok(AsmNode::Operand(Operand::Imm(*value))),
            },
            _ => Err(format!("unknown node to parse to ASM node {}", node)),
        }
    }

    fn to_asm_instructions(&self, node: &Node) -> Result<Vec<AsmNode>, String> {
        match node {
            Node::Statement(statement) => match statement {
                Statement::Return(expression) => Ok(vec![
                    AsmNode::Instruction(Instruction::Mov {
                        src: self.to_asm_operand(&expression)?,
                        dst: Operand::Register,
                    }),
                    AsmNode::Instruction(Instruction::Ret),
                ]),
            },
            _ => Err(format!("unknown node to parse to instructions {}", node)),
        }
    }

    fn to_asm_operand(&self, node: &Node) -> Result<Operand, String> {
        match node {
            Node::Expression(expression) => match expression {
                Expression::Constant(constant) => Ok(Operand::Imm(*constant)),
            },
            _ => Err(format!("invalid node type to parse to operand {}", node)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::{Lexer, parser::Parser};

    #[test]
    fn test_01() {
        fn expected(value: i64) -> AsmNode {
            return AsmNode::Program {
                function: Box::new(AsmNode::Function {
                    name: "main".into(),
                    instructions: vec![
                        AsmNode::Instruction(Instruction::Mov {
                            src: Operand::Imm(value),
                            dst: Operand::Register,
                        }),
                        AsmNode::Instruction(Instruction::Ret),
                    ],
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
            let ast = Parser::new(&tokens).parse().expect("parsing failed");
            let actual = AsmParser::new(&ast).parse().expect("asm parsing failed");

            assert_eq!(actual, expected);
        }
    }
}
