use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum AsmNode {
    Program {
        function: Box<AsmNode>,
    },
    Function {
        name: String,
        instructions: Vec<AsmNode>,
    },
    Instruction(Instruction),
    Operand(Operand),
}

impl Display for AsmNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            AsmNode::Program { function } => write!(f, "Program({})", function),
            AsmNode::Function { name, instructions } => {
                write!(f, "Function({}", name)?;
                while let Some(instruction) = instructions.iter().next() {
                    write!(f, ",{}", instruction)?;
                }
                write!(f, ")")
            }
            AsmNode::Instruction(instruction) => write!(f, "{}", instruction),
            AsmNode::Operand(operand) => write!(f, "{}", operand),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Instruction::Mov { src, dst } => write!(f, "{} {}", src, dst),
            Instruction::Ret => write!(f, "ret"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Imm(i64),
    Register,
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Operand::Imm(value) => write!(f, "{}", value),
            Operand::Register => write!(f, "operand"),
        }
    }
}
