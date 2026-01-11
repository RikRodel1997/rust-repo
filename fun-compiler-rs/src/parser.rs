use crate::environment::Environment;
use crate::errors::{Error, ErrorKind, ParserError};
use crate::lexer::lex;
use crate::node::{Node, NodeKind, NodeValue, Type};
use crate::program::Program;
use crate::tokens::{Token, TokenKind};
use crate::{SourceChars, node};

pub struct ParsingContext {
    pub types: Environment,
    pub variables: Environment,
    pub position: usize,
    pub line: usize,
}

impl ParsingContext {
    pub fn new() -> Self {
        let mut types = Environment::new(None);
        types.set_type_bindings();

        let variables = Environment::new(None);
        ParsingContext {
            types,
            variables,
            position: 0,
            line: 1,
        }
    }
}

pub fn parse(source: String, context: &mut ParsingContext) -> Result<Program, Error> {
    if source.is_empty() {
        return Err(Error {
            kind: ErrorKind::Parser(ParserError::EmptyInput),
            message: "Can't parse empty input".into(),
        });
    }

    let mut program = Program::new();

    let mut chars = source.chars().peekable();

    while let Some(tok) = lex(&mut chars, &mut context.position, &mut context.line) {
        let token = tok?;
        let literal = token.literal.clone();
        match token.kind {
            TokenKind::Value => program.add_node(value(&literal)?),
            TokenKind::Identifier => {
                if let Some(node) = program
                    .nodes
                    .iter_mut()
                    .find(|node| node.kind == NodeKind::VariableDeclaration)
                {
                    if !context.variables.bindings.contains_key(&literal) {
                        return Err(Error {
                            kind: ErrorKind::Parser(ParserError::UnknownIdentifier),
                            message: format!("Unknown identifier '{literal}'"),
                        });
                    }
                    node.children[2] = variable_redefinition(&mut chars, context)?;
                } else {
                    let symbol = Node::new(NodeKind::Symbol(literal.clone()), vec![], None);
                    let var_decl = variable_declaration(&mut chars, context, symbol.clone())?;
                    context.variables.bindings.insert(literal.clone(), symbol);
                    program.add_node(var_decl);
                }
            }
            _ => {
                // TODO: Check for unary prefix operator
                // TODO: Check that it isn't a binary operator
                // TODO: Check if valid symbol for variable environment

                println!(
                    "unrecognized token {:?} at position {}, line {}",
                    token, context.position, context.line
                );
                continue;
            }
        };
    }
    Ok(program)
}

fn variable_declaration(
    chars: &mut SourceChars,
    context: &mut ParsingContext,
    symbol: Node,
) -> Result<Node, Error> {
    let mut var_decl = match next_literal(chars, &mut context.position, &mut context.line, ":") {
        Ok(_) => {
            let mut var_decl = Node::new(NodeKind::VariableDeclaration, vec![], None);
            var_decl.add_child(symbol.clone());
            var_decl
        }
        Err(e) => return Err(e),
    };

    match next_type(chars, context) {
        Ok(_) => {
            let type_node = Node::new(NodeKind::Type(Type::Integer), vec![], None);
            var_decl.add_child(type_node)
        }
        Err(e) => return Err(e),
    }

    match next_literal(chars, &mut context.position, &mut context.line, "=") {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    match next_value(chars, &mut context.position, &mut context.line) {
        Ok(value) => {
            let value_node = Node::new(NodeKind::Value(NodeValue::Integer(value)), vec![], None);
            var_decl.add_child(value_node)
        }
        Err(e) => return Err(e),
    };

    Ok(var_decl)
}

fn variable_redefinition(
    chars: &mut SourceChars,
    context: &mut ParsingContext,
) -> Result<Node, Error> {
    match next_literal(chars, &mut context.position, &mut context.line, ":") {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    match next_literal(chars, &mut context.position, &mut context.line, "=") {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    let value = match next_value(chars, &mut context.position, &mut context.line) {
        Ok(value) => Node::new(NodeKind::Value(NodeValue::Integer(value)), vec![], None),
        Err(e) => return Err(e),
    };

    Ok(value)
}

fn next_type(chars: &mut SourceChars, context: &mut ParsingContext) -> Result<Node, Error> {
    let position = &mut context.position;
    let line = &mut context.line;

    let token = match lex(chars, position, line) {
        Some(token) => token?,
        None => {
            return Err(Error {
                kind: ErrorKind::Parser(ParserError::UnexpectedEndOfInput),
                message: "Unexpected end of input".into(),
            });
        }
    };

    let literal = token.literal;
    let is_valid = context.types.bindings.get(&literal);

    if is_valid.is_none() {
        return Err(Error {
            kind: ErrorKind::Parser(ParserError::InvalidType),
            message: format!("Unexpected type {literal}",),
        });
    }

    let type_node = match token.kind {
        TokenKind::Type => {
            let type_node = Node::new(NodeKind::Type(Type::Integer), vec![], None);
            type_node
        }
        _ => {
            return Err(Error {
                kind: ErrorKind::Parser(ParserError::UnexpectedToken),
                message: format!("Unexpected token type. Got {:?}", token.kind),
            });
        }
    };

    Ok(type_node)
}

fn next_value(
    chars: &mut SourceChars,
    position: &mut usize,
    line: &mut usize,
) -> Result<i64, Error> {
    let token = match lex(chars, position, line) {
        Some(token) => token?,
        None => {
            return Err(Error {
                kind: ErrorKind::Parser(ParserError::UnexpectedEndOfInput),
                message: "Unexpected end of input".into(),
            });
        }
    };

    let literal = &token.literal;

    let value = match token.kind {
        TokenKind::Value => {
            let value = token.literal.parse::<i64>().map_err(|_| Error {
                kind: ErrorKind::Parser(ParserError::InvalidInteger),
                message: format!("Invalid integer literal '{literal}'"),
            })?;
            value
        }
        _ => {
            let kind = token.kind;
            return Err(Error {
                kind: ErrorKind::Parser(ParserError::UnexpectedToken),
                message: format!("Unexpected token value. Got {kind:?}"),
            });
        }
    };

    Ok(value)
}

fn value(literal: &str) -> Result<Node, Error> {
    let value = literal.parse::<i64>().map_err(|_| Error {
        kind: ErrorKind::Parser(ParserError::InvalidInteger),
        message: format!("Invalid integer literal '{literal}'"),
    })?;
    Ok(Node::new(
        NodeKind::Value(NodeValue::Integer(value)),
        vec![],
        None,
    ))
}

fn next_literal(
    chars: &mut SourceChars,
    position: &mut usize,
    line: &mut usize,
    expected: &str,
) -> Result<Token, Error> {
    let token = match lex(chars, position, line) {
        Some(token) => token?,
        None => {
            return Err(Error {
                kind: ErrorKind::Parser(ParserError::UnexpectedEndOfInput),
                message: "Unexpected end of input".into(),
            });
        }
    };
    let literal = &token.literal;
    if literal != expected {
        return Err(Error {
            kind: ErrorKind::Parser(ParserError::UnexpectedToken),
            message: format!("Expected '{expected}' found '{literal}'"),
        });
    }
    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let mut ctx = ParsingContext::new();
        let source = "1\n123\n1234";
        let program = parse(source.into(), &mut ctx).unwrap();
        let nodes = program.nodes;

        assert_eq!(nodes.len(), 3);
        assert_eq!(
            nodes,
            vec![
                Node::new(NodeKind::Value(NodeValue::Integer(1)), vec![], None),
                Node::new(NodeKind::Value(NodeValue::Integer(123)), vec![], None),
                Node::new(NodeKind::Value(NodeValue::Integer(1234)), vec![], None),
            ]
        );
    }

    #[test]
    fn test_integer_error() {
        let mut ctx = ParsingContext::new();
        let source = "1\n1q";
        let program = parse(source.into(), &mut ctx);
        assert!(program.is_err());

        let error = program.unwrap_err();
        assert_eq!(
            error,
            Error {
                kind: ErrorKind::Parser(ParserError::InvalidInteger),
                message: "Invalid integer literal '1q'".into(),
            }
        );
    }

    #[test]
    fn test_variable_declaration() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer = 123";
        let program = parse(source.into(), &mut ctx).unwrap();
        let nodes = program.nodes;

        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0],
            Node::new(
                NodeKind::VariableDeclaration,
                vec![
                    Node::new(NodeKind::Symbol("a".into()), vec![], None),
                    Node::new(NodeKind::Type(Type::Integer), vec![], None),
                    Node::new(NodeKind::Value(NodeValue::Integer(123)), vec![], None),
                ],
                None,
            )
        );
    }

    #[test]
    fn test_variable_declaration_invalid_type() {
        let mut ctx = ParsingContext::new();
        let source = "a : int = 123";
        let program = parse(source.into(), &mut ctx);
        assert!(program.is_err());

        let error = program.unwrap_err();
        assert_eq!(
            error,
            Error {
                kind: ErrorKind::Parser(ParserError::InvalidType),
                message: "Unexpected type int".into(),
            }
        );
    }

    #[test]
    fn test_variable_redefinition() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer = 123\na := 456";
        let program = parse(source.into(), &mut ctx).unwrap();
        let nodes = program.nodes;

        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0],
            Node::new(
                NodeKind::VariableDeclaration,
                vec![
                    Node::new(NodeKind::Symbol("a".into()), vec![], None),
                    Node::new(NodeKind::Type(Type::Integer), vec![], None),
                    Node::new(NodeKind::Value(NodeValue::Integer(456)), vec![], None),
                ],
                None,
            )
        );
    }

    #[test]
    fn test_variable_redefinition_unknown_identifier() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer = 123\nb := 456";
        let program = parse(source.into(), &mut ctx);
        assert!(program.is_err());

        let error = program.unwrap_err();
        assert_eq!(
            error,
            Error {
                kind: ErrorKind::Parser(ParserError::UnknownIdentifier),
                message: "Unknown identifier: b".into(),
            }
        );
    }

    #[test]
    fn test_variable_redefinition_wrong_symbols() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer = 123\na = 456";
        let program = parse(source.into(), &mut ctx);
        assert!(program.is_err());

        let error = program.unwrap_err();
        assert_eq!(
            error,
            Error {
                kind: ErrorKind::Parser(ParserError::UnexpectedToken),
                message: "Expected ':' found '='".into(),
            }
        );
    }
}
