use std::mem::discriminant;
use std::{collections::HashMap, vec};

use crate::{
    asm::ast::{AsmNode, AsmUnaryOperator, Instruction, Operand, Register},
    tacky::{TackyInstruction, TackyNode, TackyUnaryOperator, TackyValue},
};

#[derive(Debug)]
pub struct AsmParser<'a> {
    pub tacky_ir: &'a TackyNode,
    pub asm_ast: Option<AsmNode>,
    pub vars: HashMap<String, Operand>,
    pub stack_offset: Vec<i64>,
}

impl<'a> AsmParser<'a> {
    pub fn new(tacky_ir: &'a TackyNode) -> Self {
        Self {
            tacky_ir,
            asm_ast: None,
            vars: HashMap::new(),
            stack_offset: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        if let TackyNode::Program { function } = &self.tacky_ir {
            let asm_ast = AsmNode::Program {
                function: Box::new(self.to_asm_node(*&function)?),
            };
            self.asm_ast = Some(asm_ast);
            Ok(())
        } else {
            Err(format!("expected Program, got {}", self.tacky_ir))
        }
    }

    pub fn parse_pseudo(&mut self) -> Result<(), String> {
        let mut new_instructions = vec![];
        if let Some(AsmNode::Program { function }) = &mut self.asm_ast {
            if let AsmNode::Function { instructions, .. } = &mut **function {
                new_instructions = std::mem::take(instructions);
            }
        }

        for instruction in new_instructions.iter_mut() {
            match instruction {
                AsmNode::Instruction(Instruction::Mov { src, dst }) => {
                    if let Some(new_src) = self.replace_pseudo(src.clone()) {
                        *src = new_src;
                    }
                    if let Some(new_dst) = self.replace_pseudo(dst.clone()) {
                        *dst = new_dst;
                    }
                }
                AsmNode::Instruction(Instruction::Unary { operand, .. }) => {
                    if let Some(new_operand) = self.replace_pseudo(operand.clone()) {
                        *operand = new_operand;
                    }
                }
                _ => {}
            };
        }

        let alloc_stack = AsmNode::Instruction(Instruction::AllocateStack(
            (self.stack_offset.len() * 4) as i64,
        ));

        new_instructions.insert(0, alloc_stack);

        if let Some(AsmNode::Program { function }) = &mut self.asm_ast {
            if let AsmNode::Function { instructions, .. } = &mut **function {
                *instructions = new_instructions;
            }
        }

        Ok(())
    }

    pub fn replace_mov(&mut self) -> Result<(), String> {
        let mut insts = vec![];
        if let Some(AsmNode::Program { function }) = &mut self.asm_ast {
            if let AsmNode::Function { instructions, .. } = &mut **function {
                insts = std::mem::take(instructions);
            }
        }

        let new_instructions: Vec<AsmNode> = insts
            .into_iter()
            .flat_map(|node| match node {
                AsmNode::Instruction(Instruction::Mov { ref src, ref dst }) => {
                    if discriminant(src) == discriminant(dst) {
                        vec![
                            AsmNode::Instruction(Instruction::Mov {
                                src: src.clone(),
                                dst: Operand::Register(Register::R10),
                            }),
                            AsmNode::Instruction(Instruction::Mov {
                                src: Operand::Register(Register::R10),
                                dst: dst.clone(),
                            }),
                        ]
                    } else {
                        vec![node.clone()]
                    }
                }
                _ => vec![node],
            })
            .collect();

        if let Some(AsmNode::Program { function }) = &mut self.asm_ast {
            if let AsmNode::Function { instructions, .. } = &mut **function {
                *instructions = new_instructions;
            }
        }

        Ok(())
    }

    fn replace_pseudo(&mut self, operand: Operand) -> Option<Operand> {
        if let Operand::Pseudo(pseudo) = operand {
            if let Some(existing) = self.vars.get(&pseudo) {
                return Some(existing.clone());
            }

            let last_offset = *self.stack_offset.last().unwrap_or(&0);
            let new_offset = last_offset - 4;
            let stack = Operand::Stack(new_offset);

            self.vars.insert(pseudo, stack.clone());
            self.stack_offset.push(new_offset);
            return Some(stack);
        }
        None
    }

    fn to_asm_node(&self, node: &TackyNode) -> Result<AsmNode, String> {
        match node {
            TackyNode::Function { name, instructions } => {
                let name = name.clone();
                let instructions = self.to_asm_instructions(instructions)?;

                Ok(AsmNode::Function { name, instructions })
            }
            _ => Err(format!("unknown tacky node to parse to ASM node {}", node)),
        }
    }

    fn to_asm_instructions(
        &self,
        instructions: &Vec<TackyInstruction>,
    ) -> Result<Vec<AsmNode>, String> {
        let mut result = vec![];
        for instruction in instructions.iter() {
            match instruction {
                TackyInstruction::Return(value) => {
                    let mov = AsmNode::Instruction(Instruction::Mov {
                        src: self.to_asm_operand(&value)?,
                        dst: Operand::Register(Register::Ax),
                    });
                    result.push(mov);
                    let ret = AsmNode::Instruction(Instruction::Ret);
                    result.push(ret);
                }
                TackyInstruction::Unary { operator, src, dst } => {
                    let src = self.to_asm_operand(&src)?;
                    let dst = self.to_asm_operand(&dst)?;
                    let mov = AsmNode::Instruction(Instruction::Mov {
                        src,
                        dst: dst.clone(),
                    });
                    result.push(mov);
                    let operator = match operator {
                        TackyUnaryOperator::Complement => AsmUnaryOperator::Not,
                        TackyUnaryOperator::Negate => AsmUnaryOperator::Neg,
                    };
                    let unary = AsmNode::Instruction(Instruction::Unary {
                        operator,
                        operand: dst,
                    });
                    result.push(unary);
                }
            }
        }

        Ok(result)
    }

    fn to_asm_operand(&self, value: &TackyValue) -> Result<Operand, String> {
        match value {
            TackyValue::Constant(constant) => Ok(Operand::Imm(*constant)),
            TackyValue::Var(var) => Ok(Operand::Pseudo(var.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::{Lexer, parser::Parser, tacky::TackyParser};

    #[test]
    fn test_01() -> Result<(), String> {
        let tests = vec![
            ("multi_digit.c", "fn(main, 100->ax, ret)"),
            ("newlines.c", "fn(main, 0->ax, ret)"),
            ("no_newlines.c", "fn(main, 0->ax, ret)"),
            ("return_0.c", "fn(main, 0->ax, ret)"),
            ("return_2.c", "fn(main, 2->ax, ret)"),
            ("spaces.c", "fn(main, 0->ax, ret)"),
            ("tabs.c", "fn(main, 0->ax, ret)"),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/01/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;

            assert_eq!(
                asm_parser
                    .asm_ast
                    .expect("expected asm_ast to be present")
                    .to_string(),
                expected
            );
        }

        Ok(())
    }

    #[test]
    fn test_02_psuedos() -> Result<(), String> {
        let tests = vec![
            (
                "bitwise_int_min.c",
                "fn(main, 2147483647->tmp.0, -tmp.0, tmp.0->tmp.1, ~tmp.1, tmp.1->ax, ret)",
            ),
            (
                "bitwise_zero.c",
                "fn(main, 0->tmp.0, ~tmp.0, tmp.0->ax, ret)",
            ),
            ("bitwise.c", "fn(main, 12->tmp.0, ~tmp.0, tmp.0->ax, ret)"),
            ("neg_zero.c", "fn(main, 0->tmp.0, -tmp.0, tmp.0->ax, ret)"),
            ("neg.c", "fn(main, 5->tmp.0, -tmp.0, tmp.0->ax, ret)"),
            (
                "negate_int_max.c",
                "fn(main, 2147483647->tmp.0, -tmp.0, tmp.0->ax, ret)",
            ),
            (
                "nested_ops_2.c",
                "fn(main, 0->tmp.0, ~tmp.0, tmp.0->tmp.1, -tmp.1, tmp.1->ax, ret)",
            ),
            (
                "nested_ops.c",
                "fn(main, 3->tmp.0, -tmp.0, tmp.0->tmp.1, ~tmp.1, tmp.1->ax, ret)",
            ),
            ("parens_2.c", "fn(main, 2->tmp.0, ~tmp.0, tmp.0->ax, ret)"),
            (
                "parens_3.c",
                "fn(main, 4->tmp.0, -tmp.0, tmp.0->tmp.1, -tmp.1, tmp.1->ax, ret)",
            ),
            ("parens.c", "fn(main, 2->tmp.0, -tmp.0, tmp.0->ax, ret)"),
            (
                "redundant_parens.c",
                "fn(main, 10->tmp.0, -tmp.0, tmp.0->ax, ret)",
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;
            assert_eq!(
                asm_parser.asm_ast.expect("no asm_ast present").to_string(),
                expected,
            );
        }
        Ok(())
    }

    #[test]
    fn test_02_replaced_pseudos() -> Result<(), String> {
        let tests = vec![
            (
                "bitwise_int_min.c",
                "fn(main, alloc 8, 2147483647->-4, --4, -4->-8, ~-8, -8->ax, ret)",
            ),
            (
                "bitwise_zero.c",
                "fn(main, alloc 4, 0->-4, ~-4, -4->ax, ret)",
            ),
            ("bitwise.c", "fn(main, alloc 4, 12->-4, ~-4, -4->ax, ret)"),
            ("neg_zero.c", "fn(main, alloc 4, 0->-4, --4, -4->ax, ret)"),
            ("neg.c", "fn(main, alloc 4, 5->-4, --4, -4->ax, ret)"),
            (
                "negate_int_max.c",
                "fn(main, alloc 4, 2147483647->-4, --4, -4->ax, ret)",
            ),
            (
                "nested_ops_2.c",
                "fn(main, alloc 8, 0->-4, ~-4, -4->-8, --8, -8->ax, ret)",
            ),
            (
                "nested_ops.c",
                "fn(main, alloc 8, 3->-4, --4, -4->-8, ~-8, -8->ax, ret)",
            ),
            ("parens_2.c", "fn(main, alloc 4, 2->-4, ~-4, -4->ax, ret)"),
            (
                "parens_3.c",
                "fn(main, alloc 8, 4->-4, --4, -4->-8, --8, -8->ax, ret)",
            ),
            ("parens.c", "fn(main, alloc 4, 2->-4, --4, -4->ax, ret)"),
            (
                "redundant_parens.c",
                "fn(main, alloc 4, 10->-4, --4, -4->ax, ret)",
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;
            asm_parser.parse_pseudo()?;

            assert_eq!(
                asm_parser.asm_ast.expect("no asm_ast present").to_string(),
                expected,
            );
        }
        Ok(())
    }

    #[test]
    fn test_replaced_mov() -> Result<(), String> {
        let tests = vec![
            (
                "nested_ops_2.c",
                "fn(main, alloc 8, 0->-4, ~-4, -4->r10, r10->-8, --8, -8->ax, ret)",
            ),
            (
                "nested_ops.c",
                "fn(main, alloc 8, 3->-4, --4, -4->r10, r10->-8, ~-8, -8->ax, ret)",
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;
            asm_parser.parse_pseudo()?;
            asm_parser.replace_mov()?;

            assert_eq!(
                asm_parser.asm_ast.expect("no asm_ast present").to_string(),
                expected,
            );
        }
        Ok(())
    }

    #[test]
    fn test_alloc_stack() -> Result<(), String> {
        let tests = vec![
            (
                "nested_ops_2.c",
                "fn(main, alloc 8, 0->-4, ~-4, -4->r10, r10->-8, --8, -8->ax, ret)",
            ),
            (
                "bitwise_zero.c",
                "fn(main, alloc 4, 0->-4, ~-4, -4->ax, ret)",
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;
            asm_parser.parse_pseudo()?;
            asm_parser.replace_mov()?;

            assert_eq!(
                asm_parser.asm_ast.expect("no asm_ast present").to_string(),
                expected,
            );
        }
        Ok(())
    }
}
