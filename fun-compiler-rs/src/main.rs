mod environment;
mod errors;
mod lexer;
mod node;
mod parser;
mod program;
mod tokens;

use std::env;
use std::fs;
use std::iter::Peekable;
use std::process::exit;
use std::str::Chars;

use crate::parser::{ParsingContext, parse};

pub type SourceChars<'a> = Peekable<Chars<'a>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = parse_args(&args);
    let source = fs::read_to_string(file).expect(&format!("Unable to read file {}", file));

    let mut context = ParsingContext::new();

    let _ = match parse(source, &mut context) {
        Ok(program) => program,
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
