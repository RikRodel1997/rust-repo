mod exprs;
mod stmts;

use core::panic;
use std::{iter::Peekable, slice::Iter};

use crate::tokens::{Builtins, Symbols, Token, TokenKind};
pub use exprs::*;
pub use stmts::*;

#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmts>,
}

pub fn parse(tokens: &mut Peekable<Iter<'_, Token>>) -> Ast {
    let mut ast = Ast { stmts: Vec::new() };
    while let Some(tk) = tokens.peek() {
        let curr_tk = tk.clone().clone();
        if matches!(curr_tk.kind, TokenKind::Symbol(_)) {
            tokens.next();
        }
        match parse_stmt(&curr_tk, tokens) {
            Some(stmt) => ast.stmts.push(stmt),
            None => (),
        }
    }
    ast
}

pub fn debug_ast(ast: &Ast) {
    println!("ast\n\n{:?}\n", ast);
}

fn parse_stmt(tk: &Token, tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Stmts> {
    return match &tk.kind {
        TokenKind::Builtin(kind) => match kind {
            Builtins::Exit => match tokens.next() {
                Some(_) => {
                    let next = tokens.next().unwrap();
                    if next.kind != TokenKind::Symbol(Symbols::OpenParen) {
                        eprintln!("next {:?}", next);
                        panic!("NO OPEN PAREN AFTER EXIT");
                    } else {
                        Some(Stmts::Exit(NodeStmtExit {
                            expr: parse_expr(tokens),
                        }))
                    }
                }
                None => None,
            },
        },
        _ => None,
    };
}

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<NodeExpr> {
    let next_token = tokens.next().unwrap();
    match next_token.kind {
        TokenKind::Value => Some(NodeExpr {
            kind: Exprs::Literal,
            token: next_token.clone().clone(),
        }),

        _ => None,
    }
}
