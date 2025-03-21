use std::{collections::HashMap, fs::File, io::Write};

use crate::{
    parser::{Ast, BinaryKinds, Exprs, NodeExpr, Stmts},
    tokens::{TokenKind, TokenValue},
};

struct Program {
    stack_size: i32,
    output: String,
    vars: HashMap<String, Var>,
}

impl Program {
    fn new() -> Self {
        Self {
            stack_size: 0,
            output: String::new(),
            vars: HashMap::new(),
        }
    }

    fn push(&mut self, register: &str) {
        let _ = &self.stack_size + 1;
        let _ = &self.output.push_str(&format!("    push {register}\n"));
    }

    fn pop(&mut self, register: &str) {
        let _ = &self.stack_size - 1;
        let _ = &self.output.push_str(&format!("    pop {register}\n"));
    }
}

struct Var {
    stack_pos: i32,
}

pub fn generate(ast: Ast) -> String {
    let mut prog = Program::new();
    prog.output.push_str("global _start\n_start:\n");

    for stmt in ast.stmts {
        generate_stmt(stmt, &mut prog);
    }
    prog.output.push_str("    mov rax, 60\n    syscall\n");
    prog.output
}

fn generate_stmt(stmt: Stmts, prog: &mut Program) {
    match stmt {
        Stmts::Exit(stmt) => match stmt.expr {
            Ok(expr) => {
                generate_expr(expr, prog);
                prog.output.push_str("    mov rax, 60\n");
                prog.pop("rdi");
                prog.output.push_str("    syscall\n");
            }
            Err(e) => panic!("{:?}", e),
        },
        Stmts::Let(stmt) => match stmt.expr {
            Ok(expr) => {
                if prog.vars.contains_key(&stmt.ident) {
                    eprintln!("{} already used.", stmt.ident);
                    panic!();
                }
                prog.vars.insert(
                    stmt.ident,
                    Var {
                        stack_pos: prog.stack_size,
                    },
                );
                generate_expr(expr, prog);
            }
            Err(e) => panic!("{:?}", e),
        },
    }
}

fn generate_expr(expr: NodeExpr, prog: &mut Program) {
    match expr.kind {
        Exprs::Literal => {
            let value = match &expr.token.kind {
                TokenKind::Value(val) => match val {
                    TokenValue::Integer(int) => int.to_string(),
                    TokenValue::Float(flt) => flt.to_string(),
                },
                _ => {
                    eprintln!("Unexpected token kind for literal expression");
                    return;
                }
            };
            prog.output.push_str(&format!("    mov rax, {value}\n"));
            prog.push("rax");
        }
        Exprs::Ident => {
            if !prog.vars.contains_key(&expr.token.lit) {
                eprintln!("Undeclared identifier {}", &expr.token.lit);
            }
            let var = prog.vars.get(&expr.token.lit).unwrap();
            let asm = format!("QWORD [rsp + {}]", prog.stack_size - var.stack_pos);
            prog.push(&asm);
        }
        Exprs::Binary(kind) => match kind {
            BinaryKinds::Addition => {
                todo!()
            }
            BinaryKinds::Multiplication => {
                todo!()
            }
        },
    }
}

pub fn write_asm(asm: String) -> Result<(), std::io::Error> {
    let mut file = File::create("out.asm")?;
    file.write_all(asm.as_bytes())?;
    Ok(())
}

pub fn debug_asm(asm: &str) {
    println!("asm\n\n{asm}\n");
}
