#![allow(dead_code, unused_variables)]
use crate::errors::ProgramError;
use crate::program::Program;

#[derive(Debug, PartialEq)]
pub enum Target {
    X86_64,
}

pub fn codegen(target: Target, program: &Program) -> Result<Vec<u8>, ProgramError> {
    match target {
        Target::X86_64 => codegen_x86_64(program),
    }
}

fn codegen_x86_64(program: &Program) -> Result<Vec<u8>, ProgramError> {
    todo!()
}
