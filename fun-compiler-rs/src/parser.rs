use crate::environment::Environment;
use crate::errors::{Error, ErrorKind, ParserError};
use crate::lexer::lex;
use crate::node::{Node, NodeKind, NodeValue};
use crate::tokens::TokenKind;

pub fn parse(source: String) -> Result<Node, Error> {
    if source.is_empty() {
        return Err(Error {
            kind: ErrorKind::Parser(ParserError::EmptyInput),
            message: "Can't parse empty input".into(),
        });
    }

    let main_environment = Environment::new(None);
    let mut chars = source.chars().peekable();

    let mut program = Node {
        kind: NodeKind::Program,
        children: Box::new(Vec::new()),
        next: None,
    };

    let mut position = 0;
    let mut line = 1;

    while let Some(tok) = lex(&mut chars, &mut position, &mut line) {
        let token = tok?;
        let literal = &token.literal;
        let expression = match token.kind {
            TokenKind::Value => {
                let value = literal.parse::<i64>().map_err(|_| Error {
                    kind: ErrorKind::Parser(ParserError::InvalidInteger(literal.into())),
                    message: format!("Invalid integer literal: {literal}"),
                })?;
                Node::new(
                    NodeKind::Value(NodeValue::Integer(value)),
                    Box::new(Vec::new()),
                    None,
                )
            }
            TokenKind::Identifier => {
                let symbol =
                    Node::new(NodeKind::Symbol(literal.into()), Box::new(Vec::new()), None);

                if let Some(tok) = lex(&mut chars, &mut position, &mut line) {
                    let token = tok?;
                    let literal = token.literal.as_str();
                    match literal {
                        ":" => {
                            if let Some(tok) = lex(&mut chars, &mut position, &mut line) {
                                let token = tok?;
                                let literal = token.literal.as_str();
                                match literal {
                                    "integer" => {
                                        let mut child = Node::new(
                                            NodeKind::VariableDeclaration,
                                            Box::new(Vec::new()),
                                            None,
                                        );
                                        child.add_child(symbol.clone());
                                        child.add_child(Node::new(
                                            NodeKind::TypeInteger,
                                            Box::new(Vec::new()),
                                            None,
                                        ));
                                        return Ok(child);
                                    }
                                    _ => {
                                        panic!(
                                            "Unexpected token {:?} at position {}, line {}",
                                            token, position, line
                                        );
                                    }
                                }
                            } else {
                                panic!("Unexpected end of input. Expected type.");
                            }
                        }
                        _ => {
                            panic!(
                                "Unexpected token {:?} at position {}, line {}",
                                token, position, line
                            );
                        }
                    }
                } else {
                    panic!("Unexpected end of input. Expected ':'");
                }
            }
            _ => {
                // TODO: Check for unary prefix operator
                // TODO: Check that it isn't a binary operator
                // TODO: Check if valid symbol for variable environment

                println!(
                    "unrecognized token {:?} at position {}, line {}",
                    token, position, line
                );
                continue;
            }
        };
        program.children.push(expression);
    }
    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let source = "1\n123\n1234";
        let program = parse(source.into());
        assert!(program.is_ok());

        let ast = program.unwrap();
        assert_eq!(ast.children.len(), 3);
        assert_eq!(
            ast,
            Node {
                kind: NodeKind::Program,
                children: Box::new(vec![
                    Node {
                        kind: NodeKind::Value(NodeValue::Integer(1)),
                        children: Box::new(vec![]),
                        next: None,
                    },
                    Node {
                        kind: NodeKind::Value(NodeValue::Integer(123)),
                        children: Box::new(vec![]),
                        next: None,
                    },
                    Node {
                        kind: NodeKind::Value(NodeValue::Integer(1234)),
                        children: Box::new(vec![]),
                        next: None,
                    },
                ]),
                next: None,
            }
        );
    }

    #[test]
    fn test_integer_error() {
        let source = "1\n1q";
        let program = parse(source.into());
        assert!(program.is_err());

        let error = program.unwrap_err();
        assert_eq!(
            error,
            Error {
                kind: ErrorKind::Parser(ParserError::InvalidInteger("1q".into())),
                message: "Invalid integer literal: 1q".into(),
            }
        );
    }

    #[test]
    fn test_variable_declaration() {
        let source = "a : integer = 123";
        let program = parse(source.into()).unwrap();
        println!("{:?}", program);
    }
}
