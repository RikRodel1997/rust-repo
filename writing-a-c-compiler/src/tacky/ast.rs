use std::fmt::{Display, Formatter, Result};

use crate::parser::{BinOp, UnOp};

#[derive(Debug, PartialEq)]
pub enum TackyNode {
    Program {
        function: Box<TackyNode>,
    },
    Function {
        name: String,
        instructions: Vec<TackyInstruction>,
    },
}

impl Display for TackyNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            TackyNode::Program { function } => write!(f, "prog {}", function),
            TackyNode::Function { name, instructions } => {
                write!(f, "fn {}", name)?;
                for instruction in instructions.iter() {
                    write!(f, " {}", instruction)?;
                }
                write!(f, "")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TackyInstruction {
    Return(TackyValue),
    Unary {
        operator: TackyUnOp,
        src: TackyValue,
        dst: TackyValue,
    },
    Binary {
        operator: TackyBinOp,
        src1: TackyValue,
        src2: TackyValue,
        dst: TackyValue,
    },
    Copy {
        src: TackyValue,
        dst: TackyValue,
    },
    Jump {
        target: String,
    },
    JumpIfZero {
        condition: TackyValue,
        target: String,
    },
    JumpIfNotZero {
        condition: TackyValue,
        target: String,
    },
    Label(String),
}

impl Display for TackyInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            TackyInstruction::Return(value) => write!(f, "ret {}", value),
            TackyInstruction::Unary { operator, src, dst } => {
                write!(f, "{operator}{src} > {dst}")
            }
            TackyInstruction::Binary {
                operator,
                src1,
                src2,
                dst,
            } => {
                write!(f, "{operator} {src1} {src2} > {dst}")
            }
            TackyInstruction::Copy { src, dst } => write!(f, "{src} > {dst}"),
            TackyInstruction::Jump { target } => write!(f, "{target}"),
            TackyInstruction::JumpIfZero { condition, target } => {
                write!(f, "{condition} {target}")
            }
            TackyInstruction::JumpIfNotZero { condition, target } => {
                write!(f, "{condition} {target}")
            }
            TackyInstruction::Label(label) => write!(f, "{label}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TackyValue {
    Constant(i64),
    Var(String),
}

impl Display for TackyValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            TackyValue::Constant(value) => write!(f, "{value}"),
            TackyValue::Var(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TackyUnOp {
    Complement,
    Negate,
    Not,
}

impl Display for TackyUnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Complement => write!(f, "~"),
            Self::Negate => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl From<&UnOp> for TackyUnOp {
    fn from(value: &UnOp) -> Self {
        match value {
            UnOp::Complement => Self::Complement,
            UnOp::Negate => Self::Negate,
            UnOp::Not => Self::Not,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TackyBinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    LShift,
    RShift,
    And,
    Or,
    Xor,
    DoubleAmpersand,
    DoublePipe,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl Display for TackyBinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Remainder => write!(f, "%"),
            Self::LShift => write!(f, "<<"),
            Self::RShift => write!(f, ">>"),
            Self::And => write!(f, "&"),
            Self::Or => write!(f, "|"),
            Self::Xor => write!(f, "^"),
            Self::DoubleAmpersand => write!(f, "&&"),
            Self::DoublePipe => write!(f, "||"),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::LessThan => write!(f, "<"),
            Self::LessThanOrEqual => write!(f, ">"),
            Self::GreaterThan => write!(f, "<="),
            Self::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

impl From<&BinOp> for TackyBinOp {
    fn from(operator: &BinOp) -> Self {
        match operator {
            BinOp::Add => TackyBinOp::Add,
            BinOp::Subtract => TackyBinOp::Subtract,
            BinOp::Multiply => TackyBinOp::Multiply,
            BinOp::Divide => TackyBinOp::Divide,
            BinOp::Remainder => TackyBinOp::Remainder,
            BinOp::LShift => TackyBinOp::LShift,
            BinOp::RShift => TackyBinOp::RShift,
            BinOp::And => TackyBinOp::And,
            BinOp::Or => TackyBinOp::Or,
            BinOp::Xor => TackyBinOp::Xor,
            BinOp::DoubleAmpersand => TackyBinOp::DoubleAmpersand,
            BinOp::DoublePipe => TackyBinOp::DoublePipe,
            BinOp::Equal => TackyBinOp::Equal,
            BinOp::NotEqual => TackyBinOp::NotEqual,
            BinOp::LessThan => TackyBinOp::LessThan,
            BinOp::LessThanOrEqual => TackyBinOp::LessThanOrEqual,
            BinOp::GreaterThan => TackyBinOp::GreaterThan,
            BinOp::GreaterThanOrEqual => TackyBinOp::GreaterThanOrEqual,
        }
    }
}
