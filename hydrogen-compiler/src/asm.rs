use std::fmt;
use std::{collections::HashMap, fs::File, io::Write};

use crate::parser::exprs::{ExprIdent, ExprLit, Exprs};
use crate::parser::stmts::{NodeStmtLet, Stmts};
use crate::tokens::TokenValue;

pub struct Program<'a> {
    stmts: &'a Vec<Stmts>,
    output: String,
    output_file: &'a str,
    stack_size: i32,
    vars: HashMap<String, Var>,
    debug: bool,
}

struct Var {
    stack_pos: i32,
}

impl<'a> Program<'a> {
    pub fn new(stmts: &'a Vec<Stmts>, output_file: &'a str, debug: bool) -> Self {
        Self {
            stack_size: 0,
            output: String::new(),
            output_file,
            vars: HashMap::new(),
            stmts,
            debug,
        }
    }

    pub fn generate(&mut self) {
        self.output.push_str("global _start\n_start:\n");

        for stmt in self.stmts.iter() {
            self.generate_stmt(stmt);
        }

        self.output.push_str("    mov rax, 60\n    syscall\n");
    }

    pub fn write(&self) -> Result<(), std::io::Error> {
        if self.debug {
            println!("asm\n\n{}\n", self.output);
        }
        let mut file = File::create(self.output_file)?;
        file.write_all(self.output.as_bytes())?;
        Ok(())
    }

    fn generate_stmt(&mut self, stmt: &Stmts) {
        match stmt {
            Stmts::Exit(stmt) => match &stmt.expr {
                Exprs::Lit(expr) => {
                    self.generate_expr_lit(expr);
                    self.output.push_str("    mov rax, 60\n");
                    self.pop("rdi");
                    self.output.push_str("    syscall\n")
                }
                Exprs::Ident(ident) => {
                    self.generate_expr_ident(ident);
                    self.output.push_str("    mov rax, 60\n");
                    self.pop("rdi");
                    self.output.push_str("    syscall\n")
                }
                _ => panic!(
                    "Unsupported expression for exit(); statement {:?}",
                    stmt.expr
                ),
            },
            Stmts::Let(stmt) => self.generate_stmt_let(stmt),
        }
    }

    fn generate_stmt_let(&mut self, stmt: &NodeStmtLet) {
        if self.vars.contains_key(&stmt.ident) {
            eprintln!("Identifier {} already used", stmt.ident);
            panic!();
        }

        self.vars.insert(
            stmt.ident.clone(),
            Var {
                stack_pos: self.stack_size,
            },
        );

        match &stmt.expr {
            Exprs::Lit(lit) => self.generate_expr_lit(lit),
            Exprs::Ident(ident) => self.generate_expr_ident(ident),
            _ => panic!("Unsupported expr {:?}", stmt.expr),
        };
    }

    fn generate_expr_lit(&mut self, lit: &ExprLit) {
        let val = match lit.value {
            TokenValue::Integer(int) => int.to_string(),
            TokenValue::Float(flt) => flt.to_string(),
        };
        self.output.push_str(&format!("    mov rax, {val}\n"));
        self.push("rax");
    }

    fn generate_expr_ident(&mut self, ident: &ExprIdent) {
        let ident = &ident.token.lit;
        if let Some(var) = self.vars.get(ident) {
            let out = format!(
                "    push QWORD [rsp + {}]\n",
                (self.stack_size - var.stack_pos - 1) * 8
            );
            self.output.push_str(out.as_str());
        } else {
            panic!("Undeclared identifier {}", ident);
        }
    }

    fn push(&mut self, register: &str) {
        self.stack_size += 1;
        self.output.push_str(&format!("    push {register}\n"));
    }

    fn pop(&mut self, register: &str) {
        self.stack_size -= 1;
        self.output.push_str(&format!("    pop {register}\n"));
    }
}

pub struct GeneratorError {
    msg: String,
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl fmt::Debug for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        parser::stmts::{NodeStmtExit, NodeStmtLet},
        tokens::{IdentKinds, Token, TokenKind},
    };

    use super::*;

    #[test]
    fn test_push_pop() {
        let stmts = Vec::new();
        let mut prog = Program::new(&stmts, "test.asm", false);
        prog.push("rax");
        assert_eq!(prog.stack_size, 1);
        assert_eq!(prog.output, "    push rax\n");

        prog.pop("rax");
        assert_eq!(prog.stack_size, 0);
        assert_eq!(prog.output, "    push rax\n    pop rax\n");

        prog.push("rax");
        assert_eq!(prog.stack_size, 1);
        assert_eq!(prog.output, "    push rax\n    pop rax\n    push rax\n");
    }

    #[test]
    fn test_exit_with_lit() {
        let mut stmts = Vec::new();
        stmts.push(Stmts::Exit(NodeStmtExit {
            expr: Exprs::Lit(ExprLit {
                value: TokenValue::Integer(4),
            }),
        }));
        let mut prog = Program::new(&stmts, "test.asm", false);
        prog.generate();
        let expected = String::from(
            "global _start\n_start:\n    mov rax, 4\n    push rax\n    mov rax, 60\n    pop rdi\n    syscall\n    mov rax, 60\n    syscall\n",
        );
        assert_eq!(prog.output, expected);
    }

    #[test]
    fn test_exit_with_ident() {
        let mut stmts = Vec::new();
        stmts.push(Stmts::Let(NodeStmtLet {
            ident: "test".to_string(),
            expr: Exprs::Lit(ExprLit {
                value: TokenValue::Integer(4),
            }),
        }));
        stmts.push(Stmts::Exit(NodeStmtExit {
            expr: Exprs::Ident(ExprIdent {
                token: Token {
                    kind: TokenKind::Ident(IdentKinds::Variable),
                    lit: "test".to_string(),
                },
            }),
        }));
        let mut prog = Program::new(&stmts, "test.asm", false);
        prog.generate();
        let expected = String::from(
            "global _start\n_start:\n    mov rax, 4\n    push rax\n    push QWORD [rsp + 0]\n    mov rax, 60\n    pop rdi\n    syscall\n    mov rax, 60\n    syscall\n",
        );
        assert_eq!(prog.output, expected);
    }

    #[test]
    fn test_let() {
        let mut stmts = Vec::new();
        stmts.push(Stmts::Let(NodeStmtLet {
            ident: "test".to_string(),
            expr: Exprs::Lit(ExprLit {
                value: TokenValue::Integer(4),
            }),
        }));
        let mut prog = Program::new(&stmts, "test.asm", false);
        prog.generate();
        let expected = String::from(
            "global _start\n_start:\n    mov rax, 4\n    push rax\n    mov rax, 60\n    syscall\n",
        );
        assert_eq!(prog.output, expected);
    }
}
