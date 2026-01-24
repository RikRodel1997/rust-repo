use std::collections::HashMap;

use crate::errors::{ParserError, ParserErrorKind};
use crate::lexer::Lexer;
use crate::node::{Node, NodeKind, Symbol, Type};
use crate::program::Program;
use crate::tokens::{Token, TokenKind};

pub struct ParsingContext {
    pub types: HashMap<String, Type>,
    pub variables: HashMap<Symbol, Node>, // TODO: we really just need a NodeValue
}

impl ParsingContext {
    pub fn new() -> Self {
        let mut types = HashMap::new();
        types.insert("integer".into(), Type::Integer(0));

        let variables = HashMap::new();
        ParsingContext { types, variables }
    }
}

pub fn parse(source: String, context: &mut ParsingContext) -> Result<Program, ParserError> {
    if source.is_empty() {
        return Err(ParserError::new(
            ParserErrorKind::EmptyInput,
            "Can't parse empty input".into(),
        ));
    }

    let mut program = Program::new();

    let chars = source.chars().peekable();

    let mut lexer = Lexer::new(chars);

    while let Some(token) = lexer.next_token() {
        let literal = token.literal.clone();
        match token.kind {
            TokenKind::Value => program.add_node(value(&literal)?),
            TokenKind::Identifier => {
                next_literal(&mut lexer, ":")?;
                let peeked = lexer
                    .peek_token()
                    .expect("Unexpected end of input during identifier parsing");
                match peeked.kind {
                    TokenKind::Type => {
                        if context.variables.contains_key(&literal) {
                            return Err(ParserError::new(
                                ParserErrorKind::VariableAlreadyDeclared,
                                format!("Variable '{literal}' is already declared"),
                            ));
                        }
                        let var_decl = variable_declaration(&mut lexer, context, literal.clone())?;
                        context.variables.insert(literal.clone(), var_decl.clone());
                        program.add_node(var_decl);
                    }
                    TokenKind::Equal => {
                        if let Some(node) = program.nodes.iter_mut().find_map(|node| {
                            node.find_kind(NodeKind::VariableDeclaration(literal.clone()))
                        }) {
                            if !context.variables.contains_key(&literal) {
                                return Err(ParserError::new(
                                    ParserErrorKind::UnknownIdentifier,
                                    format!("Unknown identifier '{literal}'"),
                                ));
                            }
                            node.kind = NodeKind::VariableReAssignment(literal.clone());
                            node.children[0] = variable_re_assignment(&mut lexer)?;
                        } else {
                            return Err(ParserError::new(
                                ParserErrorKind::UnknownIdentifier,
                                format!("Unknown identifier '{literal}'"),
                            ));
                        }
                    }
                    _ => {
                        return Err(ParserError::new(
                            ParserErrorKind::UnexpectedToken,
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
                    token, lexer.position, lexer.line
                );
                continue;
            }
        };
    }
    Ok(program)
}

fn variable_declaration(
    lexer: &mut Lexer,
    context: &mut ParsingContext,
    literal: String,
) -> Result<Node, ParserError> {
    let mut var_decl = Node::new(NodeKind::VariableDeclaration(literal), vec![], None);

    // TODO: actually use this instead of defaulting to integer
    match next_type(lexer, context) {
        Ok(node) => node,
        Err(e) => return Err(e),
    };

    match lexer.peek_token() {
        Some(token) => match token.kind {
            TokenKind::Equal => {
                next_literal(lexer, "=")?;

                let value = next_value(lexer)?;
                var_decl.add_child(Node::new(
                    NodeKind::Value(Some(Type::Integer(value))),
                    vec![],
                    None,
                ));
            }
            _ => {
                var_decl.add_child(Node::new(NodeKind::Value(None), vec![], None));
            }
        },
        None => {
            var_decl.add_child(Node::new(NodeKind::Value(None), vec![], None));
        }
    }

    Ok(var_decl)
}

fn variable_re_assignment(lexer: &mut Lexer) -> Result<Node, ParserError> {
    next_literal(lexer, "=")?;

    let value = next_value(lexer)?;
    let node = Node::new(NodeKind::Value(Some(Type::Integer(value))), vec![], None);

    Ok(node)
}

fn next_type(lexer: &mut Lexer, context: &mut ParsingContext) -> Result<Type, ParserError> {
    let token = lexer
        .next_token()
        .expect("Unexpected end of input during type parsing");

    let literal = token.literal;
    let is_valid = context.types.get(&literal);

    if is_valid.is_none() {
        return Err(ParserError::new(
            ParserErrorKind::InvalidType,
            format!("Unexpected type '{literal}'"),
        ));
    }

    Ok(is_valid.unwrap().clone())
}

fn next_value(lexer: &mut Lexer) -> Result<i64, ParserError> {
    let token = lexer
        .next_token()
        .expect("Unexpected end of input during value parsing");

    let literal = &token.literal;

    let value = match token.kind {
        TokenKind::Value => {
            let value = token.literal.parse::<i64>().map_err(|_| ParserError {
                kind: ParserErrorKind::InvalidInteger,
                message: format!("Invalid integer literal '{literal}'"),
            })?;
            value
        }
        _ => {
            let kind = token.kind;
            return Err(ParserError::new(
                ParserErrorKind::UnexpectedToken,
                format!("Unexpected token value. Got {kind:?}"),
            ));
        }
    };

    Ok(value)
}

fn value(literal: &str) -> Result<Node, ParserError> {
    let value = literal.parse::<i64>().map_err(|_| {
        ParserError::new(
            ParserErrorKind::InvalidInteger,
            format!("Invalid integer literal '{literal}'"),
        )
    })?;
    Ok(Node::new(
        NodeKind::Value(Some(Type::Integer(value))),
        vec![],
        None,
    ))
}

fn next_literal(lexer: &mut Lexer, expected: &str) -> Result<Token, ParserError> {
    let token = lexer
        .next_token()
        .expect("Unexpected end of input during literal parsing");

    let literal = &token.literal;

    if literal != expected {
        return Err(ParserError::new(
            ParserErrorKind::UnexpectedToken,
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
                Node::new(NodeKind::Value(Some(Type::Integer(1))), vec![], None),
                Node::new(NodeKind::Value(Some(Type::Integer(123))), vec![], None),
                Node::new(NodeKind::Value(Some(Type::Integer(1234))), vec![], None),
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
            ParserError {
                kind: ParserErrorKind::InvalidInteger,
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
                vec![Node::new(
                    NodeKind::Value(Some(Type::Integer(123))),
                    vec![],
                    None
                ),],
                None,
            )
        );
    }

    #[test]
    fn test_variable_declaration_without_value() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer";
        let program = parse(source.into(), &mut ctx).unwrap();
        let nodes = program.nodes;

        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0],
            Node::new(
                NodeKind::VariableDeclaration("a".into()),
                vec![Node::new(NodeKind::Value(None), vec![], None),],
                None,
            )
        );
    }

    #[test]
    fn test_variable_declaration_with_value_assigned_later() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer\na := 123";
        let program = parse(source.into(), &mut ctx).unwrap();
        let nodes = program.nodes;

        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0],
            Node::new(
                NodeKind::VariableReAssignment("a".into()),
                vec![Node::new(
                    NodeKind::Value(Some(Type::Integer(123))),
                    vec![],
                    None
                ),],
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
            ParserError {
                kind: ParserErrorKind::UnexpectedToken,
                message: "Expected '=' or type. Got literal 'int'".into(),
            }
        );
    }

    #[test]
    fn test_variable_declaration_twice() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer = 123\na : integer = 456";
        let program = parse(source.into(), &mut ctx);
        assert!(program.is_err());

        let error = program.unwrap_err();
        assert_eq!(
            error,
            ParserError {
                kind: ParserErrorKind::VariableAlreadyDeclared,
                message: "Variable 'a' is already declared".into(),
            }
        );
    }

    #[test]
    fn test_variable_re_assignment() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer = 123\na := 456";
        let program = parse(source.into(), &mut ctx).unwrap();
        let nodes = program.nodes;

        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0],
            Node::new(
                NodeKind::VariableReAssignment("a".into()),
                vec![Node::new(
                    NodeKind::Value(Some(Type::Integer(456))),
                    vec![],
                    None
                ),],
                None,
            )
        );
    }

    #[test]
    fn test_variable_re_assignment_unknown_identifier() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer = 123\nb := 456";
        let program = parse(source.into(), &mut ctx);
        assert!(program.is_err());

        let error = program.unwrap_err();
        assert_eq!(
            error,
            ParserError {
                kind: ParserErrorKind::UnknownIdentifier,
                message: "Unknown identifier 'b'".into(),
            }
        );
    }

    #[test]
    fn test_variable_re_assignment_wrong_symbols() {
        let mut ctx = ParsingContext::new();
        let source = "a : integer = 123\na = 456";
        let program = parse(source.into(), &mut ctx);
        assert!(program.is_err());

        let error = program.unwrap_err();
        assert_eq!(
            error,
            ParserError {
                kind: ParserErrorKind::UnexpectedToken,
                message: "Expected ':' found '='".into(),
            }
        );
    }
}
