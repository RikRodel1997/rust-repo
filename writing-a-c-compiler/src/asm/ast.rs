use std::fmt::{Display, Formatter, Result};

use crate::tacky::{TackyBinOp, TackyUnOp, TackyValue};

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
        operator: AsmUnOp,
        operand: Operand,
    },
    Binary {
        operator: AsmBinOp,
        src: Operand,
        dst: Operand,
    },
    Idiv(Operand),
    Cdq,
    AllocateStack(i64),
    Ret,
    Cmp {
        operand1: Operand,
        operand2: Operand,
    },
    Jmp(String),
    JumpCC {
        cond_code: CondCode,
        target: String,
    },
    SetCC {
        cond_code: CondCode,
        operand: Operand,
    },
    Label(String),
}

impl Instruction {
    pub fn to_asm(&self) -> String {
        match self {
            Instruction::Mov { src, dst } => {
                format!(
                    "\n\tmovl\t{}, {}",
                    src.to_asm(RegisterSize::Four),
                    dst.to_asm(RegisterSize::Four)
                )
            }
            Instruction::Unary { operator, operand } => match operator {
                _ => format!(
                    "{}\t{}",
                    operator.to_asm(),
                    operand.to_asm(RegisterSize::Four)
                ),
            },
            Instruction::Binary { operator, src, dst } => match operator {
                AsmBinOp::Imul
                | AsmBinOp::Add
                | AsmBinOp::Sub
                | AsmBinOp::And
                | AsmBinOp::Or
                | AsmBinOp::Xor => {
                    format!(
                        "\n\t{}\t{}, {}",
                        operator.to_asm(),
                        src.to_asm(RegisterSize::Four),
                        dst.to_asm(RegisterSize::Four),
                    )
                }
                AsmBinOp::LShift | AsmBinOp::RShift => {
                    format!(
                        "\n\t{}\t%cl, {}",
                        operator.to_asm(),
                        dst.to_asm(RegisterSize::Four),
                    )
                }
                _ => panic!("unknown bin operator for asm conversion"),
            },
            Instruction::Cdq => format!("\n\tcdq"),
            Instruction::Idiv(operand) => {
                format!("\n\tidivl\t{}", operand.to_asm(RegisterSize::Four))
            }
            Instruction::Ret => {
                let mut ret = String::from("\n\tmovq\t%rbp, %rsp\n");
                ret.push_str("\tpopq\t%rbp\n");
                ret.push_str("\tret\n");
                ret
            }
            Instruction::AllocateStack(offset) => format!("\tsubq\t${offset}, %rsp"),
            Instruction::Cmp { operand1, operand2 } => format!(
                "\n\tcmpl\t{}, {}",
                operand1.to_asm(RegisterSize::Four),
                operand2.to_asm(RegisterSize::Four)
            ),
            Instruction::Jmp(identifier) => format!("\n\tjmp\t.L{identifier}"),
            Instruction::JumpCC { cond_code, target } => format!("\n\tj{cond_code}\t.L{target}"),
            Instruction::SetCC { cond_code, operand } => {
                println!("cond_code {}", cond_code);
                format!("\n\tset{cond_code}\t{}", operand.to_asm(RegisterSize::One))
            }
            Instruction::Label(identifier) => format!("\n.L{identifier}:"),
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
            Instruction::Cmp { operand1, operand2 } => write!(f, "cmpl\t{operand1}, {operand2}"),
            Instruction::Jmp(identifier) => write!(f, "jmp\t{identifier}"),
            Instruction::JumpCC { cond_code, target } => write!(f, "je\t${cond_code}, {target}"),
            Instruction::SetCC { cond_code, operand } => write!(f, "je\t${cond_code}, {operand}"),
            Instruction::Label(identifier) => write!(f, "label\t{identifier}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AsmUnOp {
    Neg,
    Compl,
    Not,
}

impl AsmUnOp {
    pub fn to_asm(&self) -> String {
        match self {
            AsmUnOp::Neg => "\n\tnegl".into(),
            AsmUnOp::Compl => "\n\tnotl".into(),
            AsmUnOp::Not => panic!("! has no direct asm translation"),
        }
    }
}

impl TryFrom<&TackyUnOp> for AsmUnOp {
    type Error = String;

    fn try_from(value: &TackyUnOp) -> std::result::Result<Self, Self::Error> {
        match value {
            TackyUnOp::Negate => Ok(Self::Neg),
            TackyUnOp::Complement => Ok(Self::Compl),
            TackyUnOp::Not => Ok(Self::Not),
        }
    }
}

impl Display for AsmUnOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            AsmUnOp::Neg => write!(f, "-"),
            AsmUnOp::Compl => write!(f, "~"),
            AsmUnOp::Not => write!(f, "!"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AsmBinOp {
    Add,
    Sub,
    Imul,
    And,
    Or,
    Xor,
    LShift,
    RShift,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl AsmBinOp {
    pub fn to_asm(&self) -> String {
        match self {
            AsmBinOp::Add => "addl".into(),
            AsmBinOp::Sub => "subl".into(),
            AsmBinOp::Imul => "imull".into(),
            AsmBinOp::And => "andl".into(),
            AsmBinOp::Or => "orl ".into(),
            AsmBinOp::Xor => "xorl".into(),
            AsmBinOp::LShift => "shll".into(),
            AsmBinOp::RShift => "shrl".into(),
            AsmBinOp::Equal => "e".into(),
            AsmBinOp::NotEqual => "ne".into(),
            AsmBinOp::GreaterThan => "g".into(),
            AsmBinOp::GreaterThanOrEqual => "ge".into(),
            AsmBinOp::LessThan => "l".into(),
            AsmBinOp::LessThanOrEqual => "le".into(),
        }
    }
}

impl TryFrom<&TackyBinOp> for AsmBinOp {
    type Error = String;

    fn try_from(value: &TackyBinOp) -> std::result::Result<Self, Self::Error> {
        match value {
            TackyBinOp::Add => Ok(Self::Add),
            TackyBinOp::Subtract => Ok(Self::Sub),
            TackyBinOp::Multiply => Ok(Self::Imul),
            TackyBinOp::And => Ok(Self::And),
            TackyBinOp::Or => Ok(Self::Or),
            TackyBinOp::Xor => Ok(Self::Xor),
            TackyBinOp::LShift => Ok(Self::LShift),
            TackyBinOp::RShift => Ok(Self::RShift),
            TackyBinOp::Equal => Ok(Self::Equal),
            TackyBinOp::NotEqual => Ok(Self::NotEqual),
            TackyBinOp::GreaterThan => Ok(Self::GreaterThan),
            TackyBinOp::GreaterThanOrEqual => Ok(Self::GreaterThanOrEqual),
            TackyBinOp::LessThan => Ok(Self::LessThan),
            TackyBinOp::LessThanOrEqual => Ok(Self::LessThanOrEqual),
            _ => Err(format!("unknown TACKY binary operator {value}")),
        }
    }
}

impl Display for AsmBinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            AsmBinOp::Add => write!(f, "+"),
            AsmBinOp::Sub => write!(f, "-"),
            AsmBinOp::Imul => write!(f, "*"),
            AsmBinOp::And => write!(f, "&"),
            AsmBinOp::Or => write!(f, "|"),
            AsmBinOp::Xor => write!(f, "^"),
            AsmBinOp::LShift => write!(f, "<<"),
            AsmBinOp::RShift => write!(f, ">>"),
            AsmBinOp::Equal => write!(f, "=="),
            AsmBinOp::NotEqual => write!(f, "!="),
            AsmBinOp::GreaterThan => write!(f, ">"),
            AsmBinOp::GreaterThanOrEqual => write!(f, ">="),
            AsmBinOp::LessThan => write!(f, "<"),
            AsmBinOp::LessThanOrEqual => write!(f, "<="),
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
    pub fn to_asm(&self, register_size: RegisterSize) -> String {
        match self {
            Operand::Imm(value) => format!("${value}"),
            Operand::Register(register) => register.to_asm(register_size),
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
            Operand::Register(register) => write!(f, "{:?}", register),
            Operand::Pseudo(pseudo) => write!(f, "{pseudo}"),
            Operand::Stack(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RegisterSize {
    One,
    Four,
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

impl Register {
    fn to_asm(&self, size: RegisterSize) -> String {
        match &self {
            Register::Eax => match size {
                RegisterSize::One => format!("%al"),
                RegisterSize::Four => format!("%eax"),
            },
            Register::Ecx => format!("%ecx"),
            Register::Edx => match size {
                RegisterSize::One => format!("%dl"),
                RegisterSize::Four => format!("%edx"),
            },
            Register::R10d => match size {
                RegisterSize::One => format!("%r10b"),
                RegisterSize::Four => format!("%r10d"),
            },
            Register::R11d => match size {
                RegisterSize::One => format!("%r11b"),
                RegisterSize::Four => format!("%r11d"),
            },
            Register::Cl => format!("%cl"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CondCode {
    E,
    Ne,
    G,
    Ge,
    L,
    Le,
}

impl TryFrom<&TackyBinOp> for CondCode {
    type Error = String;

    fn try_from(value: &TackyBinOp) -> std::result::Result<Self, Self::Error> {
        match value {
            TackyBinOp::Equal => Ok(Self::E),
            TackyBinOp::NotEqual => Ok(Self::Ne),
            TackyBinOp::GreaterThan => Ok(Self::G),
            TackyBinOp::GreaterThanOrEqual => Ok(Self::Ge),
            TackyBinOp::LessThan => Ok(Self::L),
            TackyBinOp::LessThanOrEqual => Ok(Self::Le),
            _ => {
                let message = format!("invalid tacky bin op for cond code conversion: {value}");
                panic!("{message}")
            }
        }
    }
}

impl Display for CondCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            CondCode::E => write!(f, "e"),
            CondCode::Ne => write!(f, "ne"),
            CondCode::G => write!(f, "g"),
            CondCode::Ge => write!(f, "ge"),
            CondCode::L => write!(f, "l"),
            CondCode::Le => write!(f, "le"),
        }
    }
}
