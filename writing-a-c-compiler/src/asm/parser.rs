use std::mem::discriminant;
use std::{collections::HashMap, vec};

use crate::asm::ast::{AsmBinOp, CondCode};
use crate::tacky::TackyBinOp;
use crate::{
    asm::ast::{AsmNode, AsmUnOp, Instruction, Operand, Register},
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
                TackyInstruction::Return(value) => result.extend(vec![
                    Instruction::Mov {
                        src: Operand::try_from(value)?,
                        dst: Operand::Register(Register::Eax),
                    },
                    Instruction::Ret,
                ]),
                TackyInstruction::Unary { operator, src, dst } => {
                    let operator = AsmUnOp::try_from(operator)?;
                    let src = Operand::try_from(src)?;
                    let dst = Operand::try_from(dst)?;

                    match operator {
                        AsmUnOp::Not => result.extend(vec![
                            Instruction::Cmp {
                                operand1: Operand::Imm(0),
                                operand2: src,
                            },
                            Instruction::Mov {
                                src: Operand::Imm(0),
                                dst: dst.clone(),
                            },
                            Instruction::SetCC {
                                cond_code: CondCode::E,
                                operand: dst,
                            },
                        ]),
                        _ => result.extend(vec![
                            Instruction::Mov {
                                src,
                                dst: dst.clone(),
                            },
                            Instruction::Unary {
                                operator,
                                operand: dst,
                            },
                        ]),
                    }
                }
                TackyInstruction::Binary {
                    operator,
                    src1,
                    src2,
                    dst,
                } => match operator {
                    TackyBinOp::Divide | TackyBinOp::Remainder => result.extend(vec![
                        Instruction::Mov {
                            src: Operand::try_from(src1)?,
                            dst: Operand::Register(Register::Eax),
                        },
                        Instruction::Cdq,
                        Instruction::Idiv(Operand::try_from(src2)?),
                        Instruction::Mov {
                            src: Operand::Register(match operator {
                                TackyBinOp::Divide => Register::Eax,
                                TackyBinOp::Remainder => Register::Edx,
                                _ => {
                                    let message = format!("unknown operator {operator}");
                                    panic!("{message}");
                                }
                            }),
                            dst: Operand::try_from(dst)?,
                        },
                    ]),
                    TackyBinOp::Equal
                    | TackyBinOp::NotEqual
                    | TackyBinOp::GreaterThan
                    | TackyBinOp::GreaterThanOrEqual
                    | TackyBinOp::LessThan
                    | TackyBinOp::LessThanOrEqual => {
                        let dst = Operand::try_from(dst)?;

                        result.extend(vec![
                            Instruction::Cmp {
                                operand1: Operand::try_from(src2)?,
                                operand2: Operand::try_from(src1)?,
                            },
                            Instruction::Mov {
                                src: Operand::Imm(0),
                                dst: dst.clone(),
                            },
                            Instruction::SetCC {
                                cond_code: CondCode::try_from(operator)?,
                                operand: dst,
                            },
                        ]);
                    }
                    _ => {
                        let dst = Operand::try_from(dst)?;

                        result.extend(vec![
                            Instruction::Mov {
                                src: Operand::try_from(src1)?,
                                dst: dst.clone(),
                            },
                            Instruction::Binary {
                                operator: AsmBinOp::try_from(operator)?,
                                src: Operand::try_from(src2)?,
                                dst,
                            },
                        ]);
                    }
                },
                TackyInstruction::JumpIfZero { condition, target } => result.extend(vec![
                    Instruction::Cmp {
                        operand1: Operand::Imm(0),
                        operand2: Operand::try_from(condition)?,
                    },
                    Instruction::JumpCC {
                        cond_code: CondCode::E,
                        target: target.to_string(),
                    },
                ]),
                TackyInstruction::JumpIfNotZero { condition, target } => result.extend(vec![
                    Instruction::Cmp {
                        operand1: Operand::Imm(0),
                        operand2: Operand::try_from(condition)?,
                    },
                    Instruction::JumpCC {
                        cond_code: CondCode::Ne,
                        target: target.to_string(),
                    },
                ]),
                TackyInstruction::Jump { target } => {
                    result.push(Instruction::Jmp(target.to_string()))
                }
                TackyInstruction::Copy { src, dst } => result.push(Instruction::Mov {
                    src: Operand::try_from(src)?,
                    dst: Operand::try_from(dst)?,
                }),
                TackyInstruction::Label(label) => {
                    result.push(Instruction::Label(label.to_string()))
                }
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
                Instruction::Cmp { operand1, operand2 } => {
                    if let Some(new_operand1) = self.replace_pseudo(operand1.clone()) {
                        *operand1 = new_operand1;
                    }
                    if let Some(new_operand2) = self.replace_pseudo(operand2.clone()) {
                        *operand2 = new_operand2;
                    }
                }
                Instruction::SetCC { operand, .. } => {
                    if let Some(new_operand) = self.replace_pseudo(operand.clone()) {
                        *operand = new_operand;
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
                    AsmBinOp::Add
                    | AsmBinOp::Sub
                    | AsmBinOp::And
                    | AsmBinOp::Or
                    | AsmBinOp::Xor => {
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
                    AsmBinOp::LShift | AsmBinOp::RShift => {
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
                    AsmBinOp::Imul => {
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
                    _ => panic!("unknown bin operator for asm conversion"),
                },
                Instruction::Cmp { operand1, operand2 } => {
                    vec![
                        Instruction::Mov {
                            src: operand2,
                            dst: Operand::Register(Register::R11d),
                        },
                        Instruction::Cmp {
                            operand1,
                            operand2: Operand::Register(Register::R11d),
                        },
                    ]
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
    fn test_cmp() -> Result<(), String> {
        let tacky_ir = input(vec![
            TackyInstruction::JumpIfZero {
                condition: TackyValue::Constant(123),
                target: "tmp_false.0".to_string(),
            },
            TackyInstruction::JumpIfZero {
                condition: TackyValue::Constant(523),
                target: "tmp_false.0".to_string(),
            },
            TackyInstruction::Copy {
                src: TackyValue::Constant(12),
                dst: TackyValue::Var("tmp.0".to_string()),
            },
            TackyInstruction::Jump {
                target: "tmp_end.0".to_string(),
            },
            TackyInstruction::Label("tmp_false.0".to_string()),
            TackyInstruction::Copy {
                src: TackyValue::Constant(73432),
                dst: TackyValue::Var("tmp.0".to_string()),
            },
            TackyInstruction::Label("tmp_end.0".to_string()),
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
                    src: Operand::Imm(123),
                    dst: Operand::Register(Register::R11d)
                },
                Instruction::Cmp {
                    operand1: Operand::Imm(0),
                    operand2: Operand::Register(Register::R11d)
                },
                Instruction::JumpCC {
                    cond_code: CondCode::E,
                    target: "tmp_false.0".to_string(),
                },
                Instruction::Mov {
                    src: Operand::Imm(523),
                    dst: Operand::Register(Register::R11d)
                },
                Instruction::Cmp {
                    operand1: Operand::Imm(0),
                    operand2: Operand::Register(Register::R11d)
                },
                Instruction::JumpCC {
                    cond_code: CondCode::E,
                    target: "tmp_false.0".to_string(),
                },
                Instruction::Mov {
                    src: Operand::Imm(12),
                    dst: Operand::Stack(-4)
                },
                Instruction::Jmp("tmp_end.0".to_string()),
                Instruction::Label("tmp_false.0".to_string()),
                Instruction::Mov {
                    src: Operand::Imm(73432),
                    dst: Operand::Stack(-4)
                },
                Instruction::Label("tmp_end.0".to_string()),
                Instruction::Mov {
                    src: Operand::Stack(-4),
                    dst: Operand::Register(Register::Eax)
                },
                Instruction::Ret,
            ])
        );

        Ok(())
    }

    #[test]
    fn test_less_than_or_equal() -> Result<(), String> {
        let tacky_ir = input(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::LessThanOrEqual,
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
                    dst: Operand::Register(Register::R11d)
                },
                Instruction::Cmp {
                    operand1: Operand::Imm(5),
                    operand2: Operand::Register(Register::R11d)
                },
                Instruction::Mov {
                    src: Operand::Imm(0),
                    dst: Operand::Stack(-4)
                },
                Instruction::SetCC {
                    cond_code: CondCode::Le,
                    operand: Operand::Stack(-4)
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

    #[test]
    fn test_binary() -> Result<(), String> {
        let tacky_ir = input(vec![
            TackyInstruction::Binary {
                operator: TackyBinOp::Or,
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
                    operator: AsmBinOp::Or,
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
