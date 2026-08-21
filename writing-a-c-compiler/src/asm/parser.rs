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
    ) -> Result<Vec<Instruction>, String> {
        let mut result = vec![];

        for instruction in instructions.iter() {
            match instruction {
                TackyInstruction::Return(value) => {
                    let src = Operand::try_from(value)?;

                    result.push(Instruction::Mov {
                        src,
                        dst: Operand::Register(Register::Eax),
                    });

                    result.push(Instruction::Ret);
                }
                TackyInstruction::Unary { operator, src, dst } => {
                    let operator = AsmUnaryOperator::try_from(operator)?;
                    let src = Operand::try_from(src)?;
                    let dst = Operand::try_from(dst)?;

                    result.push(Instruction::Mov {
                        src,
                        dst: dst.clone(),
                    });
                    result.push(Instruction::Unary {
                        operator,
                        operand: dst,
                    });
                }
                TackyInstruction::Binary {
                    operator,
                    src1,
                    src2,
                    dst,
                } => match operator {
                    TackyBinaryOperator::Divide => {
                        let src1 = Operand::try_from(src1)?;
                        let src2 = Operand::try_from(src2)?;
                        let dst = Operand::try_from(dst)?;

                        result.extend(vec![
                            Instruction::Mov {
                                src: src1,
                                dst: Operand::Register(Register::Eax),
                            },
                            Instruction::Cdq,
                            Instruction::Idiv(src2),
                            Instruction::Mov {
                                src: Operand::Register(Register::Eax),
                                dst,
                            },
                        ]);
                    }
                    TackyBinaryOperator::Remainder => {
                        let src1 = Operand::try_from(src1)?;
                        let src2 = Operand::try_from(src2)?;
                        let dst = Operand::try_from(dst)?;

                        result.extend(vec![
                            Instruction::Mov {
                                src: src1,
                                dst: Operand::Register(Register::Eax),
                            },
                            Instruction::Cdq,
                            Instruction::Idiv(src2),
                            Instruction::Mov {
                                src: Operand::Register(Register::Edx),
                                dst,
                            },
                        ]);
                    }
                    _ => {
                        let operator = AsmBinaryOperator::try_from(operator)?;
                        let src1 = Operand::try_from(src1)?;
                        let src2 = Operand::try_from(src2)?;
                        let dst = Operand::try_from(dst)?;

                        result.push(Instruction::Mov {
                            src: src1,
                            dst: dst.clone(),
                        });
                        result.push(Instruction::Binary {
                            operator,
                            src: src2,
                            dst,
                        });
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
                Instruction::Mov { src, dst } => {
                    if let Some(new_src) = self.replace_pseudo(src.clone()) {
                        *src = new_src;
                    }
                    if let Some(new_dst) = self.replace_pseudo(dst.clone()) {
                        *dst = new_dst;
                    }
                }
                Instruction::Unary { operand, .. } => {
                    if let Some(new_operand) = self.replace_pseudo(operand.clone()) {
                        *operand = new_operand;
                    }
                }
                Instruction::Binary { src, dst, .. } => {
                    if let Some(new_src) = self.replace_pseudo(src.clone()) {
                        *src = new_src;
                    }
                    if let Some(new_dst) = self.replace_pseudo(dst.clone()) {
                        *dst = new_dst;
                    }
                }
                Instruction::Idiv(operand) => {
                    if let Some(new_operand) = self.replace_pseudo(operand.clone()) {
                        *operand = new_operand;
                    }
                }
                _ => {}
            };
        }

        let stack_offset = (self.stack_offset.len() * 4) as i64;
        let alloc_stack = Instruction::AllocateStack(stack_offset);

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

        let new_instructions = insts
            .into_iter()
            .flat_map(|node| match node {
                Instruction::Mov { ref src, ref dst } => {
                    if discriminant(src) == discriminant(dst) {
                        vec![
                            Instruction::Mov {
                                src: src.clone(),
                                dst: Operand::Register(Register::R10d),
                            },
                            Instruction::Mov {
                                src: Operand::Register(Register::R10d),
                                dst: dst.clone(),
                            },
                        ]
                    } else {
                        vec![node]
                    }
                }
                Instruction::Idiv(constant) => {
                    vec![
                        Instruction::Mov {
                            src: constant,
                            dst: Operand::Register(Register::R10d),
                        },
                        Instruction::Idiv(Operand::Register(Register::R10d)),
                    ]
                }
                Instruction::Binary { operator, src, dst } => match operator {
                    AsmBinaryOperator::Add
                    | AsmBinaryOperator::Sub
                    | AsmBinaryOperator::And
                    | AsmBinaryOperator::Or
                    | AsmBinaryOperator::Xor => {
                        vec![
                            Instruction::Mov {
                                src,
                                dst: Operand::Register(Register::R10d),
                            },
                            Instruction::Binary {
                                operator,
                                src: Operand::Register(Register::R10d),
                                dst,
                            },
                        ]
                    }
                    AsmBinaryOperator::LShift | AsmBinaryOperator::RShift => {
                        vec![
                            Instruction::Mov {
                                src,
                                dst: Operand::Register(Register::Ecx),
                            },
                            Instruction::Binary {
                                operator,
                                src: Operand::Register(Register::Cl),
                                dst,
                            },
                        ]
                    }
                    AsmBinaryOperator::Imul => {
                        vec![
                            Instruction::Mov {
                                src: dst.clone(),
                                dst: Operand::Register(Register::R11d),
                            },
                            Instruction::Binary {
                                operator,
                                src,
                                dst: Operand::Register(Register::R11d),
                            },
                            Instruction::Mov {
                                src: Operand::Register(Register::R11d),
                                dst,
                            },
                        ]
                    }
                },
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
    use crate::tacky::TackyValue;

    use super::*;

    fn input(instructions: Vec<TackyInstruction>) -> TackyNode {
        TackyNode::Program {
            function: Box::new(TackyNode::Function {
                name: "main".into(),
                instructions,
            }),
        }
    }

    fn expected(instructions: Vec<Instruction>) -> AsmNode {
        AsmNode::Program {
            function: Box::new(AsmNode::Function {
                name: "main".into(),
                instructions,
            }),
        }
    }

    #[test]
    fn test_basic_binary() -> Result<(), String> {
        let tacky_ir = input(vec![
            TackyInstruction::Binary {
                operator: TackyBinaryOperator::Or,
                src1: TackyValue::Constant(3),
                src2: TackyValue::Constant(5),
                dst: TackyValue::Var("tmp.0".into()),
            },
            TackyInstruction::Return(TackyValue::Var("tmp.0".into())),
        ]);
        let mut asm_parser = AsmParser::new(&tacky_ir);
        asm_parser.parse()?;
        asm_parser.parse_pseudo()?;
        asm_parser.fix_instructions()?;

        assert_eq!(
            asm_parser.asm_ast.unwrap(),
            expected(vec![
                Instruction::AllocateStack(4),
                Instruction::Mov {
                    src: Operand::Imm(3),
                    dst: Operand::Stack(-4)
                },
                Instruction::Mov {
                    src: Operand::Imm(5),
                    dst: Operand::Register(Register::R10d)
                },
                Instruction::Binary {
                    operator: AsmBinaryOperator::Or,
                    src: Operand::Register(Register::R10d),
                    dst: Operand::Stack(-4)
                },
                Instruction::Mov {
                    src: Operand::Stack(-4),
                    dst: Operand::Register(Register::Eax)
                },
                Instruction::Ret,
            ])
        );

        Ok(())
    }
}
