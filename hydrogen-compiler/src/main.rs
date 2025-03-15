mod args;
mod asm;
mod parser;
mod tokens;

use args::{ArgType, Args};

use std::fs;

fn main() {
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
    let ast = parser::parse(&mut tokens.iter().peekable());
    if debug {
        parser::print_ast(&ast);
    }

    let asm = asm::generate(ast);
    if debug {
        asm::print_asm(&asm);
    }

    match asm::write_asm(asm) {
        Ok(_) => (),
        Err(e) => eprintln!("ASM generated and written unsuccessfully {e}."),
    };
}
