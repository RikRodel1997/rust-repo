use std::{iter::Peekable, slice::Iter};

use crate::tokens::{Token, TokenKind, TokenValue};

#[derive(Debug)]
pub struct NodeExpr {
    pub kind: Exprs,
    pub value: TokenValue,
}

#[derive(Debug)]
pub enum Exprs {
    Literal,
}

#[derive(Debug)]
pub struct NodeExit {
    pub expr: Option<NodeExpr>,
}

#[derive(Debug)]
pub struct Ast {
    pub root: NodeExit,
}

pub fn parse(tokens: &mut Peekable<Iter<'_, Token>>) -> Ast {
    let mut ast: Ast = Ast {
        root: NodeExit { expr: None },
    };
    while let Some(tk) = tokens.peek() {
        match tk.kind {
            TokenKind::Value => {
                ast.root.expr = parse_expr(tokens);
                tokens.next();
            }
            _ => {
                tokens.next();
            }
        }
    }
    ast
}

pub fn print_ast(ast: &Ast) {
    println!("ast\n\n{:?}\n", ast);
}

fn parse_expr(tokens: &mut Peekable<Iter<'_, Token>>) -> Option<NodeExpr> {
    let next_token = tokens.peek().unwrap();
    match next_token.kind {
        TokenKind::Value => Some(NodeExpr {
            kind: Exprs::Literal,
            value: next_token
                .value
                .clone()
                .expect("Unable to clone tokens value"),
        }),
        _ => None,
    }
}
