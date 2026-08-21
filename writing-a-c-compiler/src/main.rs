mod asm;
mod lexer;
mod parser;
mod tacky;

use std::fs::File;
use std::io::Write;
use std::{env, fs};

use crate::asm::generator::AsmGenerator;
use crate::asm::parser::AsmParser;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::tacky::TackyParser;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();

    let output_file = output_arg(&args);

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

        let asm_code = AsmGenerator::new(&asm_ast)
            .generate()
            .expect("asm generation failed");

        match output_file {
            Some(output_path) => {
                let mut file = File::create(output_path).unwrap();
                file.write_all(asm_code.as_bytes()).unwrap();
            }
            None => {}
        }
    }

    Ok(())
}

fn output_arg(args: &[String]) -> Option<String> {
    if args.len() == 3 {
        let output_path = &args[2];
        return Some(output_path.clone());
    }

    None
}
