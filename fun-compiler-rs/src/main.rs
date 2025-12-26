mod binding;
mod environment;
mod errors;
mod lexer;
mod node;
mod parser;
mod tokens;

use std::env;
use std::fs;
use std::process::exit;

use crate::parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = parse_args(&args);
    let source = fs::read_to_string(file).expect(&format!("Unable to read file {}", file));

    match parse(source) {
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
