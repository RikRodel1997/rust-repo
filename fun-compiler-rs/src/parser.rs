use std::collections::HashMap;

use crate::SourceChars;
use crate::errors::{Error, ErrorKind, ParserError};
use crate::lexer::lex;
use crate::node::{Node, NodeKind, Symbol, Type};
use crate::program::Program;
use crate::tokens::{Token, TokenKind};

pub struct ParsingContext {
    pub types: HashMap<String, Type>,
    pub variables: HashMap<Symbol, Node>, // TODO: we really just need a NodeValue
    pub position: usize,
    pub line: usize,
}

impl ParsingContext {
    pub fn new() -> Self {
        let mut types = HashMap::new();
        types.insert("integer".into(), Type::Integer(0));

        let variables = HashMap::new();
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
        return Err(Error::new(
            ErrorKind::Parser(ParserError::EmptyInput),
            "Can't parse empty input".into(),
        ));
    }

    let mut program = Program::new();

    let mut chars = source.chars().peekable();

    while let Some(tok) = lex(&mut chars, &mut context.position, &mut context.line) {
        let token = tok?;
        let literal = token.literal.clone();
        match token.kind {
            TokenKind::Value => program.add_node(value(&literal)?),
            TokenKind::Identifier => {
                next_literal(&mut chars, &mut context.position, &mut context.line, ":")?;
                let peeked = peek_token(&mut chars, &mut context.position, &mut context.line);
                match peeked.kind {
                    TokenKind::Type => {
                        let var_decl = variable_declaration(&mut chars, context, literal.clone())?;
                        context.variables.insert(literal.clone(), var_decl.clone());
                        program.add_node(var_decl);
                    }
                    TokenKind::Equal => {
                        if let Some(node) = program.nodes.iter_mut().find(|node| {
                            node.kind == NodeKind::VariableDeclaration(literal.clone())
                        }) {
                            if !context.variables.contains_key(&literal) {
                                return Err(Error::new(
                                    ErrorKind::Parser(ParserError::UnknownIdentifier),
                                    format!("Unknown identifier '{literal}'"),
                                ));
                            }
                            node.children[0] = variable_redefinition(&mut chars, context)?;
                        } else {
                            return Err(Error::new(
                                ErrorKind::Parser(ParserError::UnknownIdentifier),
                                format!("Unknown identifier '{literal}'"),
                            ));
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::Parser(ParserError::UnexpectedToken),
                            format!("Expected '=' or type. Got literal '{}'", peeked.literal),
                        ));
                    }
                };
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

fn peek_token(chars: &mut SourceChars, position: &mut usize, line: &mut usize) -> Token {
    let mut chars_clone = chars.clone();
    lex(&mut chars_clone, position, line).unwrap().unwrap() // TODO: lol
}

fn variable_declaration(
    chars: &mut SourceChars,
    context: &mut ParsingContext,
    literal: String,
) -> Result<Node, Error> {
    let mut var_decl = Node::new(NodeKind::VariableDeclaration(literal), vec![], None);
    let type_node = match next_type(chars, context) {
        Ok(node) => node,
        Err(e) => return Err(e),
    };

    next_literal(chars, &mut context.position, &mut context.line, "=")?;

    let value = next_value(chars, &mut context.position, &mut context.line)?;
    var_decl.add_child(Node::new(
        NodeKind::Value(Type::Integer(value)),
        vec![],
        None,
    ));

    Ok(var_decl)
}

fn variable_redefinition(
    chars: &mut SourceChars,
    context: &mut ParsingContext,
) -> Result<Node, Error> {
    next_literal(chars, &mut context.position, &mut context.line, "=")?;

    let value = next_value(chars, &mut context.position, &mut context.line)?;
    let node = Node::new(NodeKind::Value(Type::Integer(value)), vec![], None);

    Ok(node)
}

fn next_type(chars: &mut SourceChars, context: &mut ParsingContext) -> Result<Type, Error> {
    let position = &mut context.position;
    let line = &mut context.line;

    let token = match lex(chars, position, line) {
        Some(token) => token?,
        None => {
            return Err(Error::new(
                ErrorKind::Parser(ParserError::UnexpectedEndOfInput),
                "Unexpected end of input".into(),
            ));
        }
    };

    let literal = token.literal;
    let is_valid = context.types.get(&literal);

    if is_valid.is_none() {
        return Err(Error::new(
            ErrorKind::Parser(ParserError::InvalidType),
            format!("Unexpected type '{literal}'"),
        ));
    }

    Ok(is_valid.unwrap().clone())
}

fn next_value(
    chars: &mut SourceChars,
    position: &mut usize,
    line: &mut usize,
) -> Result<i64, Error> {
    let token = match lex(chars, position, line) {
        Some(token) => token?,
        None => {
            return Err(Error::new(
                ErrorKind::Parser(ParserError::UnexpectedEndOfInput),
                "Unexpected end of input".into(),
            ));
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
            return Err(Error::new(
                ErrorKind::Parser(ParserError::UnexpectedToken),
                format!("Unexpected token value. Got {kind:?}"),
            ));
        }
    };

    Ok(value)
}

fn value(literal: &str) -> Result<Node, Error> {
    let value = literal.parse::<i64>().map_err(|_| {
        Error::new(
            ErrorKind::Parser(ParserError::InvalidInteger),
            format!("Invalid integer literal '{literal}'"),
        )
    })?;
    Ok(Node::new(
        NodeKind::Value(Type::Integer(value)),
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
            return Err(Error::new(
                ErrorKind::Parser(ParserError::UnexpectedEndOfInput),
                "Unexpected end of input".into(),
            ));
        }
    };
    let literal = &token.literal;
    if literal != expected {
        return Err(Error::new(
            ErrorKind::Parser(ParserError::UnexpectedToken),
            format!("Expected '{expected}' found '{literal}'"),
        ));
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
                Node::new(NodeKind::Value(Type::Integer(1)), vec![], None),
                Node::new(NodeKind::Value(Type::Integer(123)), vec![], None),
                Node::new(NodeKind::Value(Type::Integer(1234)), vec![], None),
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
                NodeKind::VariableDeclaration("a".into()),
                vec![Node::new(NodeKind::Value(Type::Integer(123)), vec![], None),],
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
                kind: ErrorKind::Parser(ParserError::UnexpectedToken),
                message: "Expected '=' or type. Got literal 'int'".into(),
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
                NodeKind::VariableDeclaration("a".into()),
                vec![Node::new(NodeKind::Value(Type::Integer(456)), vec![], None),],
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
                message: "Unknown identifier 'b'".into(),
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
