use std::fmt::{Display, Formatter, Result};

use crate::parser::UnaryOperator;

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
            TackyNode::Program { function } => write!(f, "Program({})", function),
            TackyNode::Function { name, instructions } => {
                write!(f, "Function({}", name)?;
                while let Some(instruction) = instructions.iter().next() {
                    write!(f, ",{}", instruction)?;
                }
                write!(f, ")")
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
}

impl Display for TackyInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            TackyInstruction::Return(value) => write!(f, "{}", value),
            TackyInstruction::Unary { operator, src, dst } => {
                write!(f, "{} {} {}", operator, src, dst)
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

impl TryFrom<&UnaryOperator> for TackyUnaryOperator {
    type Error = String;

    fn try_from(value: &UnaryOperator) -> std::result::Result<Self, Self::Error> {
        match value {
            UnaryOperator::Complement => Ok(Self::Complement),
            UnaryOperator::Negate => Ok(Self::Negate),
        }
    }
}
