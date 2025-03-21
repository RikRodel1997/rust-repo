mod args;
mod asm;
mod tokens;

use args::{ArgType, Args};

use std::{env, fs};

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = Args::new(env_args).expect("Incorrect usage.");
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

    // let ast = parser::parse(&mut tokens.iter().peekable());
    // if debug {
    //     parser::debug_ast(&ast);
    // }

    // let program = asm::generate(ast);
    // if debug {
    //     asm::debug_asm(&program);
    // }

    // match asm::write_asm(program) {
    //     Ok(_) => (),
    //     Err(e) => eprintln!("ASM generated and written unsuccessfully {e}."),
    // };
}
