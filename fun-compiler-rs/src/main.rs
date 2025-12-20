mod environment;
mod errors;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::process::exit;

use crate::lexer::lex;
use crate::parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = parse_args(&args);
    let source = fs::read_to_string(file).expect("Should have been able to read the file");

    let tokens = match lex(source.as_str()) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("Lexer error: {:?}", e);
            exit(1)
        }
    };

    let ast = match parse(tokens) {
        Ok(ast) => ast,
        Err(e) => {
            println!("Parser error: {:?}", e);
            exit(1)
        }
    };
}

fn parse_args(args: &[String]) -> &str {
    if args.len() == 1 {
        println!("usage: [file-path]");
        exit(2);
    }

    let file = args.get(1).unwrap();
    file
}
