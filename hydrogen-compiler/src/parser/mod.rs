mod exprs;
mod stmts;

use core::panic;
use std::{fmt, iter::Peekable, slice::Iter};

use crate::tokens::{Builtins, IdentifierKinds, Keywords, Symbols, Token, TokenKind};
pub use exprs::*;
pub use stmts::*;

#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<Stmts>,
}

pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub fn parse(tokens: &mut Peekable<Iter<'_, Token>>) -> Ast {
    let mut ast = Ast { stmts: Vec::new() };
    while let Some(tk) = tokens.peek() {
        let curr_tk = *tk;
        if matches!(curr_tk.kind, TokenKind::Symbol(_)) {
            tokens.next();
        }
        match parse_stmt(&curr_tk, tokens) {
            Some(stmt) => {
                ast.stmts.push(stmt);
            }
            None => (),
        }
    }
    ast
}

fn parse_stmt(tk: &Token, tokens: &mut Peekable<Iter<'_, Token>>) -> Option<Stmts> {
    return match &tk.kind {
        TokenKind::Builtin(kind) => match kind {
            Builtins::Exit => match tokens.next() {
                Some(_) => {
                    let next = tokens.next().unwrap();
                    if next.kind != TokenKind::Symbol(Symbols::OpenParen) {
                        panic!("NO OPEN PAREN AFTER EXIT");
                    }
                    let exit_node = Stmts::Exit(NodeStmtExit {
                        expr: parse_expr(tokens),
                    });
                    tokens.next(); // TODO: This should be a proper close_paren check
                    match ends_with_semicolon(tokens) {
                        Ok(_) => Some(exit_node),
                        Err(e) => {
                            eprintln!("{} after {:?}", e.message, exit_node);
                            None
                        }
                    }
                }
                None => None,
            },
        },
        TokenKind::Keyword(kind) => match kind {
            Keywords::Let => match tokens.next() {
                Some(_) => {
                    let next = tokens.next().unwrap();
                    if next.kind != TokenKind::Identifier(IdentifierKinds::Variable) {
                        panic!("NO IDENTIFIER AFTER LET STATEMENT");
                    }
                    tokens.next();
                    let let_node = Stmts::Let(NodeStmtLet {
                        ident: next.lit.clone(),
                        expr: parse_expr(tokens),
                    });
                    match ends_with_semicolon(tokens) {
                        Ok(_) => Some(let_node),
                        Err(e) => {
                            eprintln!("{} after {:?}", e.message, let_node);
                            None
                        }
                    }
                }
                None => None,
            },
        },
        _ => {
            panic!("Don't know what to do with tk {:?}", tk);
        }
    };
}

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<NodeExpr> {
    let next_token = tokens.next().unwrap();
    match &next_token.kind {
        TokenKind::Value(_) => Some(NodeExpr {
            kind: Exprs::Literal,
            token: next_token.clone(),
        }),
        TokenKind::Symbol(symbol) => match symbol {
            Symbols::Equal => Some(NodeExpr {
                kind: Exprs::Literal,
                token: next_token.clone(),
            }),
            _ => {
                eprintln!("Remaining tokens {:?}", tokens);
                panic!("Don't know what to do with symbol {:?}", symbol);
            }
        },
        TokenKind::Identifier(ident) => match ident {
            IdentifierKinds::Variable => Some(NodeExpr {
                kind: Exprs::Identifier,
                token: next_token.clone(),
            }),
        },
        _ => {
            eprintln!("Remaining tokens {:?}", tokens);
            panic!("Don't know what to do with next_token {:?}", next_token);
        }
    }
}

fn ends_with_semicolon(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<bool, ParseError> {
    match &tokens.next() {
        Some(tk) => {
            if tk.kind != TokenKind::Symbol(Symbols::Semicolon) {
                return Err(ParseError {
                    message: "ParserError, no semicolon found".to_string(),
                });
            }
        }
        None => {
            return Err(ParseError {
                message: "ParserError, no semicolon found".to_string(),
            });
        }
    }

    Ok(true)
}

pub fn debug_ast(ast: &Ast) {
    println!("ast\n{:?}\n", ast);
}
