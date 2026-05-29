mod asm;
mod lexer;
mod parser;
mod tacky;

use std::{env, fs};

use crate::asm::generator::AsmGenerator;
use crate::asm::parser::AsmParser;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::tacky::TackyParser;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        let file_path = &args[1];
        let input = fs::read_to_string(file_path).unwrap();
        let tokens = Lexer::new(&input).lex().expect("token lexing failed");

        let ast = Parser::new(&tokens).parse().expect("ast parsing failed");

        let tacky_ir = TackyParser::new(&ast)
            .parse()
            .expect("tacky ir parsing failed");

        let mut asm_parser = AsmParser::new(&tacky_ir);
        asm_parser.parse().expect("asm parsing failed");
        asm_parser.parse_pseudo().expect("pseudo parsing failed");
        asm_parser
            .fix_instructions()
            .expect("fixing instructions failed");

        let asm_ast = asm_parser.asm_ast.expect("expected asm_ast to be present");

        let _asm_code = AsmGenerator::new(&asm_ast)
            .generate()
            .expect("asm generation failed");
    }
}
