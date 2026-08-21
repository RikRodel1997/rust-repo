use std::fmt::{Display, Formatter, Result};

use crate::tacky::{TackyBinaryOperator, TackyUnaryOperator, TackyValue};

#[derive(Debug, PartialEq, Clone)]
pub enum AsmNode {
    Program {
        function: Box<AsmNode>,
    },
    Function {
        name: String,
        instructions: Vec<Instruction>,
    },
}

impl Display for AsmNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            AsmNode::Program { function } => write!(f, "{}", function),
            AsmNode::Function { name, instructions } => {
                write!(f, "fn({}", name)?;
                for instruction in instructions {
                    write!(f, ", {}", instruction)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Mov {
        src: Operand,
        dst: Operand,
    },
    Unary {
        operator: AsmUnaryOperator,
        operand: Operand,
    },
    Binary {
        operator: AsmBinaryOperator,
        src: Operand,
        dst: Operand,
    },
    Idiv(Operand),
    Cdq,
    AllocateStack(i64),
    Ret,
}

impl Instruction {
    pub fn to_asm(&self) -> String {
        match self {
            Instruction::Mov { src, dst } => {
                format!("movl\t{}, {}", src.to_asm(), dst.to_asm())
            }
            Instruction::Unary { operator, operand } => {
                format!("{}\t{}", operator.to_asm(), operand.to_asm())
            }
            Instruction::Binary { operator, src, dst } => match operator {
                AsmBinaryOperator::Imul
                | AsmBinaryOperator::Add
                | AsmBinaryOperator::Sub
                | AsmBinaryOperator::And
                | AsmBinaryOperator::Or
                | AsmBinaryOperator::Xor => {
                    format!("{}\t{}, {}", operator.to_asm(), src.to_asm(), dst.to_asm(),)
                }
                AsmBinaryOperator::LShift | AsmBinaryOperator::RShift => {
                    format!("{}\t%cl, {}", operator.to_asm(), dst.to_asm(),)
                }
            },
            Instruction::Cdq => format!("cdq"),
            Instruction::Idiv(operand) => format!("idivl\t{}", operand.to_asm()),
            Instruction::Ret => {
                let mut ret = String::from("movq\t%rbp, %rsp\n");
                ret.push_str("\tpopq\t%rbp\n");
                ret.push_str("\tret");
                ret
            }
            Instruction::AllocateStack(offset) => format!("subq\t${offset}, %rsp"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Instruction::Mov { src, dst } => write!(f, "{src}->{dst}"),
            Instruction::Unary { operator, operand } => write!(f, "{operator}{operand}"),
            Instruction::Binary { operator, src, dst } => write!(f, "{operator} {src} {dst}"),
            Instruction::Cdq => write!(f, "cdq"),
            Instruction::Idiv(operand) => write!(f, "/ {operand}"),
            Instruction::AllocateStack(value) => write!(f, "alloc {value}"),
            Instruction::Ret => write!(f, "ret"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AsmUnaryOperator {
    Neg,
    Not,
}

impl AsmUnaryOperator {
    pub fn to_asm(&self) -> String {
        match self {
            AsmUnaryOperator::Neg => "negl".into(),
            AsmUnaryOperator::Not => "notl".into(),
        }
    }
}

impl TryFrom<&TackyUnaryOperator> for AsmUnaryOperator {
    type Error = String;

    fn try_from(value: &TackyUnaryOperator) -> std::result::Result<Self, Self::Error> {
        match value {
            TackyUnaryOperator::Negate => Ok(Self::Neg),
            TackyUnaryOperator::Complement => Ok(Self::Not),
        }
    }
}

impl Display for AsmUnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            AsmUnaryOperator::Neg => write!(f, "-"),
            AsmUnaryOperator::Not => write!(f, "~"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AsmBinaryOperator {
    Add,
    Sub,
    Imul,
    And,
    Or,
    Xor,
    LShift,
    RShift,
}

impl AsmBinaryOperator {
    pub fn to_asm(&self) -> String {
        match self {
            AsmBinaryOperator::Add => "addl".into(),
            AsmBinaryOperator::Sub => "subl".into(),
            AsmBinaryOperator::Imul => "imull".into(),
            AsmBinaryOperator::And => "andl".into(),
            AsmBinaryOperator::Or => "orl ".into(),
            AsmBinaryOperator::Xor => "xorl".into(),
            AsmBinaryOperator::LShift => "shll".into(),
            AsmBinaryOperator::RShift => "shrl".into(),
        }
    }
}

impl TryFrom<&TackyBinaryOperator> for AsmBinaryOperator {
    type Error = String;

    fn try_from(value: &TackyBinaryOperator) -> std::result::Result<Self, Self::Error> {
        match value {
            TackyBinaryOperator::Add => Ok(Self::Add),
            TackyBinaryOperator::Subtract => Ok(Self::Sub),
            TackyBinaryOperator::Multiply => Ok(Self::Imul),
            TackyBinaryOperator::And => Ok(Self::And),
            TackyBinaryOperator::Or => Ok(Self::Or),
            TackyBinaryOperator::Xor => Ok(Self::Xor),
            TackyBinaryOperator::LShift => Ok(Self::LShift),
            TackyBinaryOperator::RShift => Ok(Self::RShift),
            _ => Err(format!("unknown TACKY binary operator {value}")),
        }
    }
}

impl Display for AsmBinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            AsmBinaryOperator::Add => write!(f, "+"),
            AsmBinaryOperator::Sub => write!(f, "-"),
            AsmBinaryOperator::Imul => write!(f, "*"),
            AsmBinaryOperator::And => write!(f, "&"),
            AsmBinaryOperator::Or => write!(f, "|"),
            AsmBinaryOperator::Xor => write!(f, "^"),
            AsmBinaryOperator::LShift => write!(f, "<<"),
            AsmBinaryOperator::RShift => write!(f, ">>"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Imm(i64),
    Register(Register),
    Pseudo(String),
    Stack(i64),
}

impl Operand {
    pub fn to_asm(&self) -> String {
        match self {
            Operand::Imm(value) => format!("${value}"),
            Operand::Register(register) => register.to_string(),
            Operand::Stack(offset) => format!("{offset}(%rbp)"),
            Operand::Pseudo(pseudo) => pseudo.into(),
        }
    }
}

impl TryFrom<&TackyValue> for Operand {
    type Error = String;

    fn try_from(value: &TackyValue) -> std::result::Result<Self, Self::Error> {
        match value {
            TackyValue::Constant(constant) => Ok(Self::Imm(*constant)),
            TackyValue::Var(var) => Ok(Self::Pseudo(var.clone())),
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Operand::Imm(value) => write!(f, "{}", value),
            Operand::Register(register) => write!(f, "{register}"),
            Operand::Pseudo(pseudo) => write!(f, "{pseudo}"),
            Operand::Stack(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    Eax,
    Ecx,
    Edx,
    R10d,
    R11d,
    Cl, // bitwise left and right shift
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Register::Eax => write!(f, "%eax"),
            Register::Ecx => write!(f, "%ecx"),
            Register::Edx => write!(f, "%edx"),
            Register::R10d => write!(f, "%r10d"),
            Register::R11d => write!(f, "%r11d"),
            Register::Cl => write!(f, "%cl"),
        }
    }
}
