use std::{env, fs};

mod ast;
mod builtins;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        let contents = fs::read_to_string(file_path)
            .map_err(|e| format!("Failed to read file '{}': {}", file_path, e))?;

        match repl::eval_input(&contents) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        repl::start()
    }
}
