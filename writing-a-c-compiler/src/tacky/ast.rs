use std::fmt::{Display, Formatter, Result};

use crate::parser::{BinaryOperator, UnaryOperator};

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
        operator: TackyUnaryOperator,
        src: TackyValue,
        dst: TackyValue,
    },
    Binary {
        operator: TackyBinaryOperator,
        src1: TackyValue,
        src2: TackyValue,
        dst: TackyValue,
    },
}

impl Display for TackyInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            TackyInstruction::Return(value) => write!(f, "ret {}", value),
            TackyInstruction::Unary { operator, src, dst } => {
                write!(f, "{}{} > {}", operator, src, dst)
            }
            TackyInstruction::Binary {
                operator,
                src1,
                src2,
                dst,
            } => {
                write!(f, "{} {} {} > {}", operator, src1, src2, dst)
            }
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
pub enum TackyUnaryOperator {
    Complement,
    Negate,
}

impl Display for TackyUnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Complement => write!(f, "~"),
            Self::Negate => write!(f, "-"),
        }
    }
}

impl From<&UnaryOperator> for TackyUnaryOperator {
    fn from(value: &UnaryOperator) -> Self {
        match value {
            UnaryOperator::Complement => Self::Complement,
            UnaryOperator::Negate => Self::Negate,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TackyBinaryOperator {
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
}

impl Display for TackyBinaryOperator {
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
        }
    }
}

impl From<&BinaryOperator> for TackyBinaryOperator {
    fn from(operator: &BinaryOperator) -> Self {
        match operator {
            BinaryOperator::Add => TackyBinaryOperator::Add,
            BinaryOperator::Subtract => TackyBinaryOperator::Subtract,
            BinaryOperator::Multiply => TackyBinaryOperator::Multiply,
            BinaryOperator::Divide => TackyBinaryOperator::Divide,
            BinaryOperator::Remainder => TackyBinaryOperator::Remainder,
            BinaryOperator::LShift => TackyBinaryOperator::LShift,
            BinaryOperator::RShift => TackyBinaryOperator::RShift,
            BinaryOperator::And => TackyBinaryOperator::And,
            BinaryOperator::Or => TackyBinaryOperator::Or,
            BinaryOperator::Xor => TackyBinaryOperator::Xor,
        }
    }
}
