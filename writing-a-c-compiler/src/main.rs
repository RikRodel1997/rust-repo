mod asm;
mod lexer;
mod parser;

use std::{env, fs};

use crate::asm::generator::AsmGenerator;
use crate::asm::parser::AsmParser;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        let file_path = &args[1];
        let input = fs::read_to_string(file_path).unwrap();
        let tokens = Lexer::new(&input).lex().expect("lexing failed");
        let ast = Parser::new(&tokens).parse().expect("parsing failed");
        let asm_ast = AsmParser::new(&ast).parse().expect("asm parsing failed");
        let _asm_code = AsmGenerator::new(&asm_ast)
            .generate()
            .expect("asm generation failed");
        println!("{ast}");
    }
}
