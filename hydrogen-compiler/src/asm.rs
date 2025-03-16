use std::{fs::File, io::Write};

use crate::{
    parser::{Ast, Exprs, NodeExpr, Stmts},
    tokens::TokenValue,
};

pub fn generate(ast: Ast) -> String {
    let mut asm = String::from("global _start\n_start:\n");

    for stmt in ast.stmts {
        asm.push_str(&generate_stmt(stmt));
    }

    asm
}

fn generate_stmt(stmt: Stmts) -> String {
    let mut asm = String::new();
    match stmt {
        Stmts::Exit(stmt) => match stmt.expr {
            Some(expr) => {
                asm.push_str(&generate_expr(expr));
                asm.push_str("    mov rax, 60\n    syscall");
            }
            None => {
                asm.push_str("    mov rax, 60\n    syscall");
            }
        },
    }
    asm
}

fn generate_expr(expr: NodeExpr) -> String {
    match expr.kind {
        Exprs::Literal => {
            let value = match &expr.token.value.as_ref().unwrap() {
                TokenValue::Integer(int) => int.to_string(),
                TokenValue::Float(flt) => flt.to_string(),
                TokenValue::String(str) => str.to_string(),
            };
            String::from(format!("    mov rdi, {value}\n"))
        }
        Exprs::Identifier => String::from("TODO"),
    }
}

pub fn debug_asm(asm: &str) {
    println!("asm\n\n{asm}\n");
}

pub fn write_asm(asm: String) -> Result<(), std::io::Error> {
    let mut file = File::create("out.asm")?;
    file.write_all(asm.as_bytes())?;
    Ok(())
}
