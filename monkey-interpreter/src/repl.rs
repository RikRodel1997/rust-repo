use std::{
    cell::RefCell,
    io::{Write, stdin, stdout},
    rc::Rc,
};

use crate::{evaluator::eval, lexer::Lexer, object::Environment, parser::Parser};

pub fn eval_input(input: &str) -> Result<crate::object::Object, String> {
    let environment = Rc::new(RefCell::new(Environment::new(None)));
    let mut lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse_program()?;
    eval(program, environment)
}

pub fn start() -> Result<(), String> {
    let environment = Rc::new(RefCell::new(Environment::new(None)));
    loop {
        print!(">> ");
        stdout().flush().unwrap();

        let mut buffer = String::new();
        let bytes_read = stdin().read_line(&mut buffer).unwrap();

        if bytes_read == 0 {
            return Ok(());
        }

        let input = buffer.trim().to_string();

        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(&mut lexer);

        let program = parser.parse_program()?;
        eval(program, Rc::clone(&environment))?;
    }
}
