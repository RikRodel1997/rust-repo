use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum AsmNode {
    Program {
        function: Box<AsmNode>,
    },
    Function {
        name: String,
        instructions: Vec<AsmNode>,
    },
    Instruction(Instruction),
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
            AsmNode::Instruction(instruction) => write!(f, "{}", instruction),
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

impl Display for AsmUnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            AsmUnaryOperator::Neg => write!(f, "-"),
            AsmUnaryOperator::Not => write!(f, "~"),
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
            Operand::Register(register) => register.to_asm(),
            Operand::Stack(offset) => format!("{offset}(%rbp)"),
            Operand::Pseudo(pseudo) => pseudo.into(),
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
    Ax,
    R10,
}

impl Register {
    pub fn to_asm(&self) -> String {
        match self {
            Register::Ax => "%eax".into(),
            Register::R10 => "%r10d".into(),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Register::Ax => write!(f, "ax"),
            Register::R10 => write!(f, "r10"),
        }
    }
}
