mod exprs;
mod stmts;

use core::panic;
use std::{fmt, iter::Peekable, slice::Iter};

use crate::tokens::{Builtins, IdentKinds, Keywords, Symbols, Token, TokenKind, TokenValue};
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
                    if next.kind != TokenKind::Ident(IdentKinds::Variable) {
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

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<Exprs, ParseError> {
    let next_token = tokens.next().unwrap();
    match &next_token.kind {
        TokenKind::Value(val) => match tokens.next() {
            Some(tk) => match &tk.kind {
                TokenKind::Symbol(symbol) => match symbol {
                    Symbols::Semicolon => Ok(NodeExpr {
                        kind: Exprs::Literal,
                        token: next_token.clone(),
                    }),
                    Symbols::Plus => parse_binary_expr(val, BinaryKinds::Addition, tokens),
                    Symbols::Star => parse_binary_expr(val, BinaryKinds::Multiplication, tokens),
                    _ => Err(ParseError {
                        message: format!(
                            "Unexpected symbol {:?}.\nRemaining tokens {:?}",
                            symbol, tokens
                        ),
                    }),
                },
                TokenKind::Value(_) => Ok(NodeExpr {
                    kind: Exprs::Literal,
                    token: next_token.clone(),
                }),
                _ => Err(ParseError {
                    message: format!("Unexpected token {:?}.", tk),
                }),
            },
            None => Err(ParseError {
                message: format!("Unexpected end of input.\nRemaining tokens {:?}", tokens),
            }),
        },

        TokenKind::Symbol(symbol) => match symbol {
            Symbols::Equal => Ok(NodeExpr {
                kind: Exprs::Literal,
                token: next_token.clone(),
            }),
            _ => Err(ParseError {
                message: format!("Invalid symbol {:?}\nRemaining tokens {:?}", symbol, tokens),
            }),
        },
        TokenKind::Ident(ident) => match ident {
            IdentKinds::Variable => Ok(NodeExpr {
                kind: Exprs::Ident,
                token: next_token.clone(),
            }),
        },
        _ => Err(ParseError {
            message: format!(
                "Unexpected token {:?}\nRemaining tokens {:?}",
                next_token, tokens
            ),
        }),
    }
}

fn parse_binary_expr(
    lhs: &TokenValue,
    operator: BinaryKinds,
    tokens: &mut Peekable<Iter<'_, Token>>,
) -> Result<NodeExpr, ParseError> {
    println!("tokens: {:?}", tokens);
    println!("lhs: {:?}", lhs);
    parse_expr(tokens).unwrap();

    Ok(NodeExpr {
        kind: Exprs::Binary(operator),
        token: tokens.next().unwrap().clone(),
    })
}

fn ends_with_semicolon(tokens: &mut Peekable<Iter<'_, Token>>) -> Result<bool, ParseError> {
    match &tokens.next() {
        Some(tk) => {
            if tk.kind != TokenKind::Symbol(Symbols::Semicolon) {
                return Err(ParseError {
                    message: "No semicolon found".to_string(),
                });
            }
        }
        None => {
            return Err(ParseError {
                message: "No semicolon found".to_string(),
            });
        }
    }
    Ok(true)
}

pub fn debug_ast(ast: &Ast) {
    println!("ast\n{:?}\n", ast);
}
