use std::{cell::RefCell, fmt};

use exprs::*;
use stmts::*;

use crate::tokens::{Builtins, IdentKinds, Keywords, Symbols, Token, TokenKind, TokenValue};

pub mod exprs;
pub mod stmts;

pub struct Parser {
    tokens: Vec<Token>,
    pos: RefCell<usize>,
    debug: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, debug: bool) -> Self {
        Self {
            tokens,
            pos: RefCell::new(0),
            debug,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmts>, ParserError> {
        let mut result: Vec<Stmts> = Vec::new();
        while let Some(t) = self.get_next() {
            match &t.kind {
                TokenKind::Builtin(builtin) => result.push(self.parse_builtin(builtin)?),
                TokenKind::Keyword(keyword) => result.push(self.parse_keyword(keyword)?),
                _ => {
                    return Err(ParserError {
                        msg: format!("Unsupported TokenKind {:?}", t.kind),
                    });
                }
            }
        }
        if self.debug {
            Parser::debug_ast(&result);
        }
        Ok(result)
    }

    fn parse_builtin(&self, builtin: &Builtins) -> Result<Stmts, ParserError> {
        match builtin {
            Builtins::Exit => {
                if let Some(next_t) = self.get_next() {
                    if next_t.kind != TokenKind::Symbol(Symbols::OpenParen) {
                        return Err(ParserError {
                            msg: "No open paren after exit token".to_string(),
                        });
                    }
                    let node_exit = Stmts::Exit(NodeStmtExit {
                        expr: self.parse_expr()?,
                    });
                    if let true = self.func_close() {
                        Ok(node_exit)
                    } else {
                        Err(ParserError {
                            msg: "Incorrectly closed exit(); builtin".to_string(),
                        })
                    }
                } else {
                    return Err(ParserError {
                        msg: "Unexpected end of exit(); builtin".to_string(),
                    });
                }
            }
        }
    }

    fn parse_keyword(&self, keyword: &Keywords) -> Result<Stmts, ParserError> {
        match keyword {
            Keywords::Let => {
                if let Some(next_t) = self.get_next() {
                    if next_t.kind != TokenKind::Ident(IdentKinds::Variable) {
                        return Err(ParserError {
                            msg: "No identifier after let token".to_string(),
                        });
                    }
                    self.get_next(); // skip equals
                    let let_node = Stmts::Let(NodeStmtLet {
                        ident: next_t.lit.clone(),
                        expr: self.parse_expr()?,
                    });
                    if let true = self.let_close() {
                        Ok(let_node)
                    } else {
                        Err(ParserError {
                            msg: "Missing semicolon at let statement".to_string(),
                        })
                    }
                } else {
                    return Err(ParserError {
                        msg: "Unexpected end of 'let' declaration".to_string(),
                    });
                }
            }
        }
    }

    fn parse_expr(&self) -> Result<Exprs, ParserError> {
        if let Some(token) = self.get_next() {
            return match &token.kind {
                TokenKind::Value(value) => {
                    if let Some(symbol_token) = self.get_next() {
                        match &symbol_token.kind {
                            TokenKind::Symbol(symbol) => match symbol {
                                Symbols::Semicolon => Ok(Exprs::Lit(ExprLit {
                                    value: value.clone(),
                                })),
                                Symbols::Plus => Ok(Exprs::Bin(ExprBin {
                                    lhs: token.clone(),
                                    rhs: self
                                        .get_next()
                                        .expect("Unexpected end of binary expression")
                                        .clone(),
                                    operator: BinOps::Addition,
                                })),
                                Symbols::Star => Ok(Exprs::Bin(ExprBin {
                                    lhs: token.clone(),
                                    rhs: self
                                        .get_next()
                                        .expect("Unexpected end of binary expression")
                                        .clone(),
                                    operator: BinOps::Multiplication,
                                })),
                                _ => Err(ParserError {
                                    msg: format!(
                                        "Unexpected symbol in expression {:?}",
                                        symbol_token
                                    ),
                                }),
                            },
                            _ => Err(ParserError {
                                msg: format!(
                                    "Unexpected token kind in expression {:?}",
                                    symbol_token
                                ),
                            }),
                        }
                    } else {
                        return Err(ParserError {
                            msg: String::from("Unexpected end of expression"),
                        });
                    }
                }
                TokenKind::Ident(IdentKinds::Variable) => {
                    let next = self.get_next();
                    self.parse_ident_expr(token)
                }
                _ => Err(ParserError {
                    msg: format!("Unexpected expression {:?}", token.kind),
                }),
            };
        } else {
            Err(ParserError {
                msg: format!("Unable to find token"),
            })
        }
    }

    fn parse_bin_expr(
        &self,
        lhs: &Token,
        operator: BinOps,
        rhs: &Token,
    ) -> Result<Exprs, ParserError> {
        Ok(Exprs::Bin(ExprBin {
            lhs: lhs.clone(),
            rhs: rhs.clone(),
            operator,
        }))
    }

    fn parse_ident_expr(&self, token: &Token) -> Result<Exprs, ParserError> {
        match &token.kind {
            TokenKind::Value(value) => Ok(Exprs::Lit(ExprLit {
                value: value.clone(),
            })),
            TokenKind::Ident(_) => Ok(Exprs::Ident(ExprIdent {
                token: token.clone(),
            })),
            _ => Err(ParserError {
                msg: format!("Unexpected token while parsing identifer {:?}", token),
            }),
        }
    }

    fn get_next(&self) -> Option<&Token> {
        let mut pos = self.pos.borrow_mut();
        if *pos < self.tokens.len() {
            let token = &self.tokens[*pos];
            *pos += 1;
            Some(token)
        } else {
            None
        }
    }

    // Checks whether a function or builtin is closed correctly
    // i.e. closed with a TokenKind::CloseParen and TokenKind::SemiColon
    fn func_close(&self) -> bool {
        let close_paren = self.get_next();
        let semi_colon = self.get_next();

        if let (Some(close), Some(semi)) = (close_paren, semi_colon) {
            if close.kind == TokenKind::Symbol(Symbols::CloseParen)
                && semi.kind == TokenKind::Symbol(Symbols::Semicolon)
            {
                return true;
            }
            return false;
        } else {
            return false;
        }
    }

    // Checks whether a let statement is closed correctly
    // i.e. closed with a TokenKind::SemiColon
    fn let_close(&self) -> bool {
        if let Some(semi) = self.get_next() {
            if semi.kind == TokenKind::Symbol(Symbols::Semicolon) {
                return true;
            }
            return false;
        } else {
            return false;
        }
    }

    fn debug_ast(stmts: &Vec<Stmts>) {
        println!("{:?}", stmts);
    }
}

pub struct ParserError {
    msg: String,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_parse() {
        let tokens: Vec<Token> = vec![Token {
            kind: TokenKind::Symbol(Symbols::Equal),
            lit: "=".to_string(),
        }];

        let mut pars = Parser::new(tokens, false);
        let out = pars.parse();
        assert_eq!(out.is_err(), true);
        assert_eq!(
            out.err().unwrap().msg,
            String::from("Unsupported TokenKind Symbol(Equal)")
        );
    }

    #[test]
    fn let_should_parse() {
        let tokens: Vec<Token> = vec![
            Token {
                kind: TokenKind::Keyword(Keywords::Let),
                lit: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident(IdentKinds::Variable),
                lit: "test".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Equal),
                lit: "=".to_string(),
            },
            Token {
                kind: TokenKind::Value(TokenValue::Float(4.0)),
                lit: "4.0".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),
                lit: ";".to_string(),
            },
        ];

        let mut pars = Parser::new(tokens, false);
        let out = pars.parse();
        assert_eq!(out.is_err(), false);

        let result = out.unwrap();
        assert_eq!(result.len(), 1);

        let stmt = result.get(0).unwrap();
        assert_eq!(
            *stmt,
            Stmts::Let(NodeStmtLet {
                ident: "test".to_string(),
                expr: Exprs::Lit(ExprLit {
                    value: TokenValue::Float(4.0),
                })
            })
        )
    }

    #[test]
    fn let_errors() {
        let tokens: Vec<Vec<Token>> = vec![
            vec![
                Token {
                    kind: TokenKind::Keyword(Keywords::Let),
                    lit: "let".to_string(),
                },
                Token {
                    kind: TokenKind::Ident(IdentKinds::Variable),
                    lit: "test".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::Equal),
                    lit: "=".to_string(),
                },
                Token {
                    kind: TokenKind::Value(TokenValue::Float(4.0)),
                    lit: "4.0".to_string(),
                },
            ],
            vec![
                Token {
                    kind: TokenKind::Keyword(Keywords::Let),
                    lit: "let".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::Equal),
                    lit: "=".to_string(),
                },
                Token {
                    kind: TokenKind::Value(TokenValue::Float(4.0)),
                    lit: "4.0".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::Semicolon),
                    lit: ";".to_string(),
                },
            ],
            vec![
                Token {
                    kind: TokenKind::Keyword(Keywords::Let),
                    lit: "let".to_string(),
                },
                Token {
                    kind: TokenKind::Ident(IdentKinds::Variable),
                    lit: "test".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::Equal),
                    lit: "=".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::Semicolon),
                    lit: ";".to_string(),
                },
            ],
            vec![Token {
                kind: TokenKind::Keyword(Keywords::Let),
                lit: "let".to_string(),
            }],
        ];

        let expected_errors = vec![
            String::from("Missing semicolon at let statement"),
            String::from("No identifier after let token"),
            String::from("Unexpected expression Symbol(Semicolon)"),
            String::from("Unexpected end of 'let' declaration"),
        ];

        for (i, token) in tokens.iter().enumerate() {
            let mut pars = Parser::new(token.clone(), false);
            let out = pars.parse();
            assert_eq!(out.is_err(), true);
            assert_eq!(out.err().unwrap().msg, *expected_errors.get(i).unwrap());
        }
    }

    #[test]
    fn exit_should_parse() {
        let tokens: Vec<Token> = vec![
            Token {
                kind: TokenKind::Builtin(Builtins::Exit),
                lit: "exit".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::OpenParen),
                lit: "(".to_string(),
            },
            Token {
                kind: TokenKind::Value(TokenValue::Integer(4)),
                lit: "4".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::CloseParen),
                lit: ")".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),
                lit: ";".to_string(),
            },
        ];

        let mut pars = Parser::new(tokens, false);
        let out = pars.parse();
        assert_eq!(out.is_err(), false);

        let result = out.unwrap();
        assert_eq!(result.len(), 1);

        let stmt = result.get(0).unwrap();
        assert_eq!(
            *stmt,
            Stmts::Exit(NodeStmtExit {
                expr: Exprs::Lit(ExprLit {
                    value: TokenValue::Integer(4)
                })
            })
        )
    }

    #[test]
    fn exit_errors() {
        let tokens: Vec<Vec<Token>> = vec![
            vec![
                Token {
                    kind: TokenKind::Builtin(Builtins::Exit),
                    lit: "exit".to_string(),
                },
                Token {
                    kind: TokenKind::Value(TokenValue::Integer(4)),
                    lit: "4".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::CloseParen),
                    lit: ")".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::Semicolon),
                    lit: ";".to_string(),
                },
            ],
            vec![
                Token {
                    kind: TokenKind::Builtin(Builtins::Exit),
                    lit: "exit".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::OpenParen),
                    lit: "(".to_string(),
                },
                Token {
                    kind: TokenKind::Value(TokenValue::Integer(4)),
                    lit: "4".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::Semicolon),
                    lit: ";".to_string(),
                },
            ],
            vec![
                Token {
                    kind: TokenKind::Builtin(Builtins::Exit),
                    lit: "exit".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::OpenParen),
                    lit: "(".to_string(),
                },
                Token {
                    kind: TokenKind::Value(TokenValue::Integer(4)),
                    lit: "4".to_string(),
                },
                Token {
                    kind: TokenKind::Symbol(Symbols::CloseParen),
                    lit: ")".to_string(),
                },
            ],
            vec![Token {
                kind: TokenKind::Builtin(Builtins::Exit),
                lit: "exit".to_string(),
            }],
        ];

        let expected_errors = vec![
            String::from("No open paren after exit token"),
            String::from("Incorrectly closed exit(); builtin"),
            String::from("Incorrectly closed exit(); builtin"),
            String::from("Unexpected end of exit(); builtin"),
        ];

        for (i, token) in tokens.iter().enumerate() {
            let mut pars = Parser::new(token.clone(), false);
            let out = pars.parse();
            assert_eq!(out.is_err(), true);
            assert_eq!(out.err().unwrap().msg, *expected_errors.get(i).unwrap());
        }
    }

    #[test]
    fn exit_with_let_should_parse() {
        let tokens: Vec<Token> = vec![
            Token {
                kind: TokenKind::Keyword(Keywords::Let),
                lit: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident(IdentKinds::Variable),
                lit: "test".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Equal),
                lit: "=".to_string(),
            },
            Token {
                kind: TokenKind::Value(TokenValue::Integer(7)),
                lit: "7".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),
                lit: ";".to_string(),
            },
            Token {
                kind: TokenKind::Builtin(Builtins::Exit),
                lit: "exit".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::OpenParen),
                lit: "(".to_string(),
            },
            Token {
                kind: TokenKind::Ident(IdentKinds::Variable),
                lit: "test".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::CloseParen),
                lit: ")".to_string(),
            },
            Token {
                kind: TokenKind::Symbol(Symbols::Semicolon),
                lit: ";".to_string(),
            },
        ];

        let mut pars = Parser::new(tokens, false);
        let out = pars.parse();
        assert_eq!(out.is_err(), false);

        let result = out.unwrap();
        assert_eq!(result.len(), 2);

        let let_stmt = result.get(0).unwrap();
        assert_eq!(
            *let_stmt,
            Stmts::Let(NodeStmtLet {
                ident: "test".to_string(),
                expr: Exprs::Lit(ExprLit {
                    value: TokenValue::Integer(7)
                })
            })
        );

        let exit_stmt = result.get(1).unwrap();
        assert_eq!(
            *exit_stmt,
            Stmts::Exit(NodeStmtExit {
                expr: Exprs::Ident(ExprIdent {
                    token: Token {
                        kind: TokenKind::Ident(IdentKinds::Variable),
                        lit: "test".to_string(),
                    },
                })
            })
        )
    }
}
