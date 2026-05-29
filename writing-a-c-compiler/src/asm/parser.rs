use std::mem::discriminant;
use std::{collections::HashMap, vec};

use crate::asm::ast::AsmBinaryOperator;
use crate::tacky::TackyBinaryOperator;
use crate::{
    asm::ast::{AsmNode, AsmUnaryOperator, Instruction, Operand, Register},
    tacky::{TackyInstruction, TackyNode},
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
            self.asm_ast = Some(AsmNode::Program {
                function: Box::new(self.to_asm_node(*&function)?),
            });
            Ok(())
        } else {
            Err(format!("expected Program, got {}", self.tacky_ir))
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
                    let src = Operand::try_from(value)?;

                    result.push(AsmNode::Instruction(Instruction::Mov {
                        src,
                        dst: Operand::Register(Register::Ax),
                    }));

                    result.push(AsmNode::Instruction(Instruction::Ret));
                }
                TackyInstruction::Unary { operator, src, dst } => {
                    let operator = AsmUnaryOperator::try_from(operator)?;
                    let src = Operand::try_from(src)?;
                    let dst = Operand::try_from(dst)?;

                    result.push(AsmNode::Instruction(Instruction::Mov {
                        src,
                        dst: dst.clone(),
                    }));

                    result.push(AsmNode::Instruction(Instruction::Unary {
                        operator,
                        operand: dst,
                    }));
                }
                TackyInstruction::Binary {
                    operator,
                    src1,
                    src2,
                    dst,
                } => match operator {
                    TackyBinaryOperator::Add
                    | TackyBinaryOperator::Subtract
                    | TackyBinaryOperator::Multiply
                    | TackyBinaryOperator::And
                    | TackyBinaryOperator::Or
                    | TackyBinaryOperator::Xor => {
                        let operator = AsmBinaryOperator::try_from(operator)?;
                        let src1 = Operand::try_from(src1)?;
                        let src2 = Operand::try_from(src2)?;
                        let dst = Operand::try_from(dst)?;

                        result.push(AsmNode::Instruction(Instruction::Mov {
                            src: src1,
                            dst: dst.clone(),
                        }));

                        result.push(AsmNode::Instruction(Instruction::Binary {
                            operator,
                            src: src2,
                            dst,
                        }));
                    }
                    TackyBinaryOperator::LShift | TackyBinaryOperator::RShift => {
                        let operator = AsmBinaryOperator::try_from(operator)?;
                        let src1 = Operand::try_from(src1)?;
                        let src2 = Operand::try_from(src2)?;
                        let dst = Operand::try_from(dst)?;

                        result.push(AsmNode::Instruction(Instruction::Mov {
                            src: src1,
                            dst: dst.clone(),
                        }));

                        result.push(AsmNode::Instruction(Instruction::Binary {
                            operator,
                            src: src2,
                            dst,
                        }));
                    }
                    TackyBinaryOperator::Divide => {
                        let src1 = Operand::try_from(src1)?;
                        let src2 = Operand::try_from(src2)?;
                        let dst = Operand::try_from(dst)?;

                        result.extend(vec![
                            AsmNode::Instruction(Instruction::Mov {
                                src: src1,
                                dst: Operand::Register(Register::Ax),
                            }),
                            AsmNode::Instruction(Instruction::Cdq),
                            AsmNode::Instruction(Instruction::Idiv(src2)),
                            AsmNode::Instruction(Instruction::Mov {
                                src: Operand::Register(Register::Ax),
                                dst,
                            }),
                        ]);
                    }
                    TackyBinaryOperator::Remainder => {
                        let src1 = Operand::try_from(src1)?;
                        let src2 = Operand::try_from(src2)?;
                        let dst = Operand::try_from(dst)?;

                        result.extend(vec![
                            AsmNode::Instruction(Instruction::Mov {
                                src: src1,
                                dst: Operand::Register(Register::Ax),
                            }),
                            AsmNode::Instruction(Instruction::Cdq),
                            AsmNode::Instruction(Instruction::Idiv(src2)),
                            AsmNode::Instruction(Instruction::Mov {
                                src: Operand::Register(Register::Dx),
                                dst,
                            }),
                        ]);
                    }
                },
            }
        }

        Ok(result)
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
                AsmNode::Instruction(Instruction::Binary { src, dst, .. }) => {
                    if let Some(new_src) = self.replace_pseudo(src.clone()) {
                        *src = new_src;
                    }
                    if let Some(new_dst) = self.replace_pseudo(dst.clone()) {
                        *dst = new_dst;
                    }
                }
                AsmNode::Instruction(Instruction::Idiv(operand)) => {
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

    pub fn fix_instructions(&mut self) -> Result<(), String> {
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
                        vec![node]
                    }
                }
                AsmNode::Instruction(Instruction::Idiv(constant)) => {
                    vec![
                        AsmNode::Instruction(Instruction::Mov {
                            src: constant,
                            dst: Operand::Register(Register::R10),
                        }),
                        AsmNode::Instruction(Instruction::Idiv(Operand::Register(Register::R10))),
                    ]
                }
                AsmNode::Instruction(Instruction::Binary { operator, src, dst }) => {
                    match operator {
                        AsmBinaryOperator::Add
                        | AsmBinaryOperator::Sub
                        | AsmBinaryOperator::And
                        | AsmBinaryOperator::Or
                        | AsmBinaryOperator::Xor => {
                            vec![
                                AsmNode::Instruction(Instruction::Mov {
                                    src,
                                    dst: Operand::Register(Register::R10),
                                }),
                                AsmNode::Instruction(Instruction::Binary {
                                    operator,
                                    src: Operand::Register(Register::R10),
                                    dst,
                                }),
                            ]
                        }
                        AsmBinaryOperator::LShift | AsmBinaryOperator::RShift => {
                            vec![
                                AsmNode::Instruction(Instruction::Mov {
                                    src,
                                    dst: Operand::Register(Register::Cx),
                                }),
                                AsmNode::Instruction(Instruction::Binary {
                                    operator,
                                    src: Operand::Register(Register::Cl),
                                    dst,
                                }),
                            ]
                        }
                        AsmBinaryOperator::Imul => {
                            vec![
                                AsmNode::Instruction(Instruction::Mov {
                                    src: dst.clone(),
                                    dst: Operand::Register(Register::R11),
                                }),
                                AsmNode::Instruction(Instruction::Binary {
                                    operator,
                                    src,
                                    dst: Operand::Register(Register::R11),
                                }),
                                AsmNode::Instruction(Instruction::Mov {
                                    src: Operand::Register(Register::R11),
                                    dst,
                                }),
                            ]
                        }
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
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::{Lexer, parser::Parser, tacky::TackyParser};

    #[test]
    fn test_01() -> Result<(), String> {
        fn expected(variance: &str) -> String {
            format!("fn(main, {variance}, ret)")
        }

        let tests = vec![
            ("multi_digit.c", expected("100->%eax")),
            ("newlines.c", expected("0->%eax")),
            ("no_newlines.c", expected("0->%eax")),
            ("return_0.c", expected("0->%eax")),
            ("return_2.c", expected("2->%eax")),
            ("spaces.c", expected("0->%eax")),
            ("tabs.c", expected("0->%eax")),
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
        fn expected(variance: &str) -> String {
            format!("fn(main, {variance}, ret)")
        }

        let tests = vec![
            (
                "bitwise_int_min.c",
                expected("2147483647->tmp.0, -tmp.0, tmp.0->tmp.1, ~tmp.1, tmp.1->%eax"),
            ),
            ("bitwise_zero.c", expected("0->tmp.0, ~tmp.0, tmp.0->%eax")),
            ("bitwise.c", expected("12->tmp.0, ~tmp.0, tmp.0->%eax")),
            ("neg_zero.c", expected("0->tmp.0, -tmp.0, tmp.0->%eax")),
            ("neg.c", expected("5->tmp.0, -tmp.0, tmp.0->%eax")),
            (
                "negate_int_max.c",
                expected("2147483647->tmp.0, -tmp.0, tmp.0->%eax"),
            ),
            (
                "nested_ops_2.c",
                expected("0->tmp.0, ~tmp.0, tmp.0->tmp.1, -tmp.1, tmp.1->%eax"),
            ),
            (
                "nested_ops.c",
                expected("3->tmp.0, -tmp.0, tmp.0->tmp.1, ~tmp.1, tmp.1->%eax"),
            ),
            ("parens_2.c", expected("2->tmp.0, ~tmp.0, tmp.0->%eax")),
            (
                "parens_3.c",
                expected("4->tmp.0, -tmp.0, tmp.0->tmp.1, -tmp.1, tmp.1->%eax"),
            ),
            ("parens.c", expected("2->tmp.0, -tmp.0, tmp.0->%eax")),
            (
                "redundant_parens.c",
                expected("10->tmp.0, -tmp.0, tmp.0->%eax"),
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
        fn expected(stack_size: i8, variance: &str) -> String {
            format!("fn(main, alloc {stack_size}, {variance}, ret)")
        }

        let tests = vec![
            (
                "bitwise_int_min.c",
                expected(8, "2147483647->-4, --4, -4->-8, ~-8, -8->%eax"),
            ),
            ("bitwise_zero.c", expected(4, "0->-4, ~-4, -4->%eax")),
            ("bitwise.c", expected(4, "12->-4, ~-4, -4->%eax")),
            ("neg_zero.c", expected(4, "0->-4, --4, -4->%eax")),
            ("neg.c", expected(4, "5->-4, --4, -4->%eax")),
            (
                "negate_int_max.c",
                expected(4, "2147483647->-4, --4, -4->%eax"),
            ),
            (
                "nested_ops_2.c",
                expected(8, "0->-4, ~-4, -4->-8, --8, -8->%eax"),
            ),
            (
                "nested_ops.c",
                expected(8, "3->-4, --4, -4->-8, ~-8, -8->%eax"),
            ),
            ("parens_2.c", expected(4, "2->-4, ~-4, -4->%eax")),
            (
                "parens_3.c",
                expected(8, "4->-4, --4, -4->-8, --8, -8->%eax"),
            ),
            ("parens.c", expected(4, "2->-4, --4, -4->%eax")),
            ("redundant_parens.c", expected(4, "10->-4, --4, -4->%eax")),
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
    fn test_02_fix_instructions() -> Result<(), String> {
        let tests = vec![
            (
                "nested_ops_2.c",
                "fn(main, alloc 8, 0->-4, ~-4, -4->%r10d, %r10d->-8, --8, -8->%eax, ret)",
            ),
            (
                "nested_ops.c",
                "fn(main, alloc 8, 3->-4, --4, -4->%r10d, %r10d->-8, ~-8, -8->%eax, ret)",
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
            asm_parser.fix_instructions()?;

            assert_eq!(
                asm_parser.asm_ast.expect("no asm_ast present").to_string(),
                expected,
            );
        }
        Ok(())
    }

    #[test]
    fn test_03() -> Result<(), String> {
        fn expected(variance: &str, stack_size: i8) -> String {
            format!("fn(main, alloc {stack_size}, {variance}, ret)")
        }

        let tests = vec![
            ("add.c", expected("1->-4, + 2 -4, -4->%eax", 4)),
            (
                "associativity_2.c",
                expected(
                    "6->%eax, cdq, / 3, %eax->-4, -4->%eax, cdq, / 2, %eax->-8, -8->%eax",
                    8,
                ),
            ),
            (
                "associativity_3.c",
                expected(
                    "3->%eax, cdq, / 2, %eax->-4, -4->-8, * 4 -8, 5->-12, - 4 -12, -12->-16, + 3 -16, -8->-20, + -16 -20, -20->%eax",
                    20,
                ),
            ),
            (
                "associativity_and_precedence.c",
                expected(
                    "5->-4, * 4 -4, -4->%eax, cdq, / 2, %eax->-8, 2->-12, + 1 -12, 3->%eax, cdq, / -12, %edx->-16, -8->-20, - -16 -20, -20->%eax",
                    20,
                ),
            ),
            (
                "associativity.c",
                expected("1->-4, - 2 -4, -4->-8, - 3 -8, -8->%eax", 8),
            ),
            (
                "div_neg.c",
                expected("12->-4, --4, -4->%eax, cdq, / 5, %eax->-8, -8->%eax", 8),
            ),
            (
                "div.c",
                expected("4->%eax, cdq, / 2, %eax->-4, -4->%eax", 4),
            ),
            (
                "mod.c",
                expected("4->%eax, cdq, / 2, %edx->-4, -4->%eax", 4),
            ),
            ("mult.c", expected("2->-4, * 3 -4, -4->%eax", 4)),
            (
                "parens.c",
                expected("3->-4, + 4 -4, 2->-8, * -4 -8, -8->%eax", 8),
            ),
            (
                "precedence.c",
                expected("3->-4, * 4 -4, 2->-8, + -4 -8, -8->%eax", 8),
            ),
            (
                "sub_neg.c",
                expected("1->-4, --4, 2->-8, - -4 -8, -8->%eax", 8),
            ),
            ("sub.c", expected("1->-4, - 2 -4, -4->%eax", 4)),
            (
                "unop_add.c",
                expected("2->-4, ~-4, -4->-8, + 3 -8, -8->%eax", 8),
            ),
            (
                "unop_parens.c",
                expected("1->-4, + 1 -4, -4->-8, ~-8, -8->%eax", 8),
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/03/{file}");
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
    fn test_03_fix_instructions() -> Result<(), String> {
        fn expected(variance: &str, stack_size: i8) -> String {
            format!("fn(main, alloc {stack_size}, {variance}, ret)")
        }

        let tests = vec![
            (
                "add.c",
                expected("1->-4, 2->%r10d, + %r10d -4, -4->%eax", 4),
            ),
            (
                "associativity_2.c",
                expected(
                    "6->%eax, cdq, 3->%r10d, / %r10d, %eax->-4, -4->%eax, cdq, 2->%r10d, / %r10d, %eax->-8, -8->%eax",
                    8,
                ),
            ),
            (
                "associativity_3.c",
                expected(
                    "3->%eax, cdq, 2->%r10d, / %r10d, %eax->-4, -4->%r10d, %r10d->-8, -8->%r11d, * 4 %r11d, %r11d->-8, 5->-12, 4->%r10d, - %r10d -12, -12->%r10d, %r10d->-16, 3->%r10d, + %r10d -16, -8->%r10d, %r10d->-20, -16->%r10d, + %r10d -20, -20->%eax",
                    20,
                ),
            ),
            (
                "associativity_and_precedence.c",
                expected(
                    "5->-4, -4->%r11d, * 4 %r11d, %r11d->-4, -4->%eax, cdq, 2->%r10d, / %r10d, %eax->-8, 2->-12, 1->%r10d, + %r10d -12, 3->%eax, cdq, -12->%r10d, / %r10d, %edx->-16, -8->%r10d, %r10d->-20, -16->%r10d, - %r10d -20, -20->%eax",
                    20,
                ),
            ),
            (
                "associativity.c",
                expected(
                    "1->-4, 2->%r10d, - %r10d -4, -4->%r10d, %r10d->-8, 3->%r10d, - %r10d -8, -8->%eax",
                    8,
                ),
            ),
            (
                "div_neg.c",
                expected(
                    "12->-4, --4, -4->%eax, cdq, 5->%r10d, / %r10d, %eax->-8, -8->%eax",
                    8,
                ),
            ),
            (
                "div.c",
                expected("4->%eax, cdq, 2->%r10d, / %r10d, %eax->-4, -4->%eax", 4),
            ),
            (
                "mod.c",
                expected("4->%eax, cdq, 2->%r10d, / %r10d, %edx->-4, -4->%eax", 4),
            ),
            (
                "mult.c",
                expected("2->-4, -4->%r11d, * 3 %r11d, %r11d->-4, -4->%eax", 4),
            ),
            (
                "parens.c",
                expected(
                    "3->-4, 4->%r10d, + %r10d -4, 2->-8, -8->%r11d, * -4 %r11d, %r11d->-8, -8->%eax",
                    8,
                ),
            ),
            (
                "precedence.c",
                expected(
                    "3->-4, -4->%r11d, * 4 %r11d, %r11d->-4, 2->-8, -4->%r10d, + %r10d -8, -8->%eax",
                    8,
                ),
            ),
            (
                "sub_neg.c",
                expected("1->-4, --4, 2->-8, -4->%r10d, - %r10d -8, -8->%eax", 8),
            ),
            (
                "sub.c",
                expected("1->-4, 2->%r10d, - %r10d -4, -4->%eax", 4),
            ),
            (
                "unop_add.c",
                expected(
                    "2->-4, ~-4, -4->%r10d, %r10d->-8, 3->%r10d, + %r10d -8, -8->%eax",
                    8,
                ),
            ),
            (
                "unop_parens.c",
                expected(
                    "1->-4, 1->%r10d, + %r10d -4, -4->%r10d, %r10d->-8, ~-8, -8->%eax",
                    8,
                ),
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/03/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;
            asm_parser.parse_pseudo()?;
            asm_parser.fix_instructions()?;

            assert_eq!(
                asm_parser.asm_ast.expect("no asm_ast present").to_string(),
                expected,
            );
        }
        Ok(())
    }
}
