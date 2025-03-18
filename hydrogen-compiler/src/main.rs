mod args;
mod asm;
mod parser;
mod tokens;

use args::{ArgType, Args};

use std::{collections::HashMap, fs};

pub struct Program {
    pub stack_size: i32,
    pub output: String,
    pub vars: HashMap<String, Var>,
}

impl Program {
    fn new() -> Self {
        Self {
            stack_size: 0,
            output: String::new(),
            vars: HashMap::new(),
        }
    }

    fn push(&mut self, register: &str) {
        let _ = &self.stack_size + 1;
        let _ = &self.output.push_str(&format!("    push {register}\n"));
    }

    fn pop(&mut self, register: &str) {
        let _ = &self.stack_size - 1;
        let _ = &self.output.push_str(&format!("    pop {register}\n"));
    }
}

pub struct Var {
    pub stack_pos: i32,
}

fn main() {
    // let prog = Program::new();
    let args: Vec<Args> = Args::new();
    let file_arg = args.iter().find(|arg| arg.arg_type == ArgType::File);
    let file_path = file_arg.unwrap().arg_value.as_ref().unwrap();
    let debug = match args.iter().find(|arg| arg.arg_type == ArgType::DebugMode) {
        Some(_) => true,
        None => false,
    };

    let data =
        fs::read_to_string(file_path).expect(&format!("Was unable to read file {file_path}"));

    let tokens = tokens::tokenize(data);
    if debug {
        tokens::debug_tokens(&tokens);
    }

    let ast = parser::parse(&mut tokens.iter().peekable());
    if debug {
        parser::debug_ast(&ast);
    }

    let asm = asm::generate(ast);
    if debug {
        asm::debug_asm(&asm);
    }

    match asm::write_asm(asm) {
        Ok(_) => (),
        Err(e) => eprintln!("ASM generated and written unsuccessfully {e}."),
    };
}
