use std::{collections::BTreeMap, vec};

use crate::{
    ast::{Program, expressions::*, node::Node, statements::*},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}

impl From<TokenType> for Precedence {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::Equal | TokenType::NotEqual => Self::Equals,
            TokenType::LessThan | TokenType::GreaterThan => Self::LessGreater,
            TokenType::Plus | TokenType::Minus => Self::Sum,
            TokenType::Slash | TokenType::Asterisk => Self::Product,
            TokenType::LParen => Self::Call,
            TokenType::LBracket => Self::Index,
            _ => Self::Lowest,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Parser<'a> {
    pub lexer: &'a mut Lexer,
    pub errors: Vec<String>,
    pub current_token: Token,
    pub peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            errors: vec![],
            current_token: Token {
                token_type: TokenType::Eof,
                literal: "".into(),
            },
            peek_token: Token {
                token_type: TokenType::Eof,
                literal: "".into(),
            },
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn parse_program(&mut self) -> Result<Node, String> {
        let mut program = Program::new(None);

        while !self.current_token_is(TokenType::Eof) {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token();
        }

        Ok(Node::Program(program))
    }

    fn parse_statement(&mut self) -> Result<Node, String> {
        match self.current_token.token_type {
            TokenType::Let => Ok(Node::LetStatement(self.parse_let_statement()?)),
            TokenType::Return => Ok(Node::ReturnStatement(self.parse_return_statement()?)),
            _ => Ok(Node::ExpressionStatement(
                self.parse_expression_statement()?,
            )),
        }
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, String> {
        let current_token = self.current_token.clone();

        self.next_token();

        if self.current_token_is(TokenType::SemiColon) {
            return Ok(ReturnStatement::new(current_token, None));
        }

        let return_value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::SemiColon) {
            self.next_token();
        }

        Ok(ReturnStatement::new(
            current_token,
            Some(Box::new(return_value)),
        ))
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, String> {
        let current_token = self.current_token.clone();

        self.expect_peek(TokenType::Ident)?;

        let name = Identifier::new(
            self.current_token.clone(),
            self.current_token.literal.clone(),
        );

        self.expect_peek(TokenType::Assign)?;

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenType::SemiColon) {
            self.next_token();
        }

        let let_stmt = LetStatement::new(current_token, name, Box::new(value));

        Ok(let_stmt)
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, String> {
        let expression_statement = ExpressionStatement::new(
            self.current_token.clone(),
            Some(Box::new(self.parse_expression(Precedence::Lowest)?)),
        );

        if self.peek_token_is(TokenType::SemiColon) {
            self.next_token();
        }

        Ok(expression_statement)
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, String> {
        let token = self.current_token.clone();
        let mut statements: Vec<Node> = Vec::new();

        self.next_token();

        while !self.current_token_is(TokenType::RBrace) && !self.current_token_is(TokenType::Eof) {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }
        Ok(BlockStatement::new(token, statements))
    }

    fn prefix(&mut self) -> Result<Node, String> {
        match self.current_token.token_type {
            TokenType::Ident => Ok(self.parse_identifier()),
            TokenType::Integer => Ok(self.parse_integer_literal()?),
            TokenType::True | TokenType::False => Ok(self.parse_boolean()?),
            TokenType::Bang | TokenType::Minus => Ok(self.parse_prefix_expression()?),
            TokenType::LParen => Ok(self.parse_grouped_expression()?),
            TokenType::LBracket => Ok(self.parse_array()?),
            TokenType::LBrace => Ok(self.parse_hash()?),
            TokenType::If => Ok(self.parse_if_expression()?),
            TokenType::Function => Ok(self.parse_function_literal()?),
            TokenType::String => Ok(self.parse_string_literal()),
            _ => {
                return Err(format!(
                    "Unknown prefix token '{}' of type '{:?}'",
                    self.current_token.literal, self.current_token.token_type
                ));
            }
        }
    }

    fn infix(&mut self, left: Node) -> Result<Node, String> {
        match self.current_token.token_type {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Slash
            | TokenType::Asterisk
            | TokenType::Equal
            | TokenType::NotEqual
            | TokenType::LessThan
            | TokenType::Assign
            | TokenType::GreaterThan => Ok(self.parse_infix_expression(left)?),
            TokenType::LParen => Ok(self.parse_call_expression(left)?),
            TokenType::LBracket => Ok(self.parse_index_expression(left)?),
            _ => {
                return Err(format!("Unknown token in infix {:?}", self.current_token));
            }
        }
    }

    fn parse_index_expression(&mut self, left: Node) -> Result<Node, String> {
        let token = self.current_token.clone();
        self.next_token();
        let index = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenType::RBracket)?;

        Ok(Node::IndexExpression(IndexExpression::new(
            token,
            Box::new(left),
            Box::new(index),
        )))
    }

    fn parse_hash(&mut self) -> Result<Node, String> {
        let token = self.current_token.clone();
        let mut pairs = BTreeMap::new();

        while !self.peek_token_is(TokenType::RBrace) {
            self.next_token();

            let key = self.parse_expression(Precedence::Lowest)?;

            self.expect_peek(TokenType::Colon)?;
            self.next_token();

            let value = self.parse_expression(Precedence::Lowest)?;

            pairs.insert(key, value);

            if self.peek_token_is(TokenType::RBrace) {
                break;
            }

            self.expect_peek(TokenType::Comma)?;
        }

        self.expect_peek(TokenType::RBrace)?;
        Ok(Node::Hash(Hash::new(token, pairs)))
    }

    fn parse_array(&mut self) -> Result<Node, String> {
        let token = self.current_token.clone();
        let elements = self.parse_expression_list(TokenType::RBracket)?;

        Ok(Node::Array(Array::new(token, elements)))
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Result<Vec<Node>, String> {
        let mut list = Vec::new();
        if self.peek_token_is(end) {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(TokenType::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }

        self.expect_peek(end)?;

        Ok(list)
    }

    fn parse_string_literal(&mut self) -> Node {
        let token = self.current_token.clone();
        Node::String(StringLiteral::new(token.clone(), token.literal))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Node, String> {
        let mut left_expression = self.prefix()?;

        while !self.peek_token_is(TokenType::SemiColon) && precedence < self.peek_precedence() {
            self.next_token();
            left_expression = self.infix(left_expression)?;
        }

        Ok(left_expression)
    }

    fn parse_prefix_expression(&mut self) -> Result<Node, String> {
        let token = self.current_token.clone();
        let operator = self.current_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;
        Ok(Node::PrefixExpression(Box::new(PrefixExpression::new(
            token, operator, right,
        ))))
    }

    fn parse_infix_expression(&mut self, left: Node) -> Result<Node, String> {
        let token = self.current_token.clone();
        let operator = self.current_token.clone().literal;
        let left = left;

        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        Ok(Node::InfixExpression(Box::new(InfixExpression::new(
            token, left, operator, right,
        ))))
    }

    fn parse_call_expression(&mut self, function: Node) -> Result<Node, String> {
        let token = self.current_token.clone();
        let arguments = self.parse_expression_list(TokenType::RParen)?;
        Ok(Node::CallExpression(Box::new(CallExpression::new(
            token, function, arguments,
        ))))
    }

    fn parse_if_expression(&mut self) -> Result<Node, String> {
        let token = self.current_token.clone();
        self.expect_peek(TokenType::LParen)?;
        self.next_token();

        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_peek(TokenType::RParen)?;
        self.expect_peek(TokenType::LBrace)?;

        let consequence = self.parse_block_statement()?;

        if self.peek_token_is(TokenType::Else) {
            self.next_token();
            self.expect_peek(TokenType::LBrace)?;

            let alternative = self.parse_block_statement()?;

            Ok(Node::IfExpression(IfExpression::new(
                token,
                Box::new(condition),
                Box::new(Node::BlockStatement(consequence)),
                Some(Box::new(Node::BlockStatement(alternative))),
            )))
        } else {
            Ok(Node::IfExpression(IfExpression::new(
                token,
                Box::new(condition),
                Box::new(Node::BlockStatement(consequence)),
                None,
            )))
        }
    }

    fn parse_function_literal(&mut self) -> Result<Node, String> {
        let token = self.current_token.clone();

        self.expect_peek(TokenType::LParen)?;
        let parameters = self.parse_function_parameters()?;

        self.expect_peek(TokenType::LBrace)?;
        let body = self.parse_block_statement()?;

        Ok(Node::FunctionLiteral(FunctionLiteral::new(
            token,
            parameters,
            Box::new(Node::BlockStatement(body)),
        )))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, String> {
        let mut parameters = vec![];

        if self.peek_token_is(TokenType::RParen) {
            self.next_token();
            return Ok(parameters);
        }

        self.next_token();
        let token = self.current_token.clone();
        parameters.push(Identifier::new(token.clone(), token.literal));

        loop {
            if self.peek_token_is(TokenType::Comma) {
                self.next_token();
                self.next_token();
                let token = self.current_token.clone();
                parameters.push(Identifier::new(token.clone(), token.literal));
            } else {
                break;
            }
        }

        self.expect_peek(TokenType::RParen)?;
        Ok(parameters)
    }

    fn parse_grouped_expression(&mut self) -> Result<Node, String> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest);
        self.expect_peek(TokenType::RParen)?;
        expr
    }

    fn parse_identifier(&mut self) -> Node {
        Node::Identifier(Identifier::new(
            self.current_token.clone(),
            self.current_token.clone().literal,
        ))
    }

    fn parse_boolean(&mut self) -> Result<Node, String> {
        let value = match self.current_token.literal.parse::<bool>() {
            Ok(value) => value,
            Err(_) => {
                return Err(format!(
                    "Current token {:?} is an invalid boolean",
                    self.current_token
                ));
            }
        };
        Ok(Node::Boolean(Boolean::new(
            self.current_token.clone(),
            value,
        )))
    }

    fn parse_integer_literal(&mut self) -> Result<Node, String> {
        let value = match self.current_token.literal.parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                return Err(format!(
                    "Current token {:?} is an invalid integer",
                    self.current_token
                ));
            }
        };
        Ok(Node::IntegerLiteral(IntegerLiteral::new(
            self.current_token.clone(),
            value,
        )))
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, token_type: TokenType) -> Result<(), String> {
        if self.peek_token.token_type == token_type {
            self.next_token();
            Ok(())
        } else {
            Err(format!(
                "Peek token type '{:?}' is not as expected '{:?}'",
                self.peek_token.token_type, token_type
            ))
        }
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn peek_precedence(&self) -> Precedence {
        Precedence::from(self.peek_token.token_type)
    }

    fn current_precedence(&self) -> Precedence {
        Precedence::from(self.current_token.token_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stmt_string(input: &&str) -> String {
        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        let stmts = match program {
            Node::Program(program) => program
                .statements
                .iter()
                .map(|s| s.to_string())
                .collect::<String>(),
            _ => panic!("Unexpected node type in test!"),
        };
        stmts
    }

    #[test]
    fn test_new_without_tokens() {
        let mut lexer = Lexer::new("".into());
        let parser = Parser::new(&mut lexer);
        assert_eq!(
            parser,
            Parser {
                lexer: &mut Lexer {
                    input: "".into(),
                    pos: 2,
                    read_pos: 3,
                    ch: '\0'
                },
                errors: vec![],
                current_token: Token {
                    token_type: TokenType::Eof,
                    literal: "".into()
                },
                peek_token: Token {
                    token_type: TokenType::Eof,
                    literal: "".into()
                }
            }
        )
    }

    #[test]
    fn test_new_with_variable_declaration() {
        let mut lexer = Lexer::new("let 5 = x;".into());
        let parser = Parser::new(&mut lexer);
        assert_eq!(
            parser,
            Parser {
                lexer: &mut Lexer {
                    input: "let 5 = x;".into(),
                    pos: 5,
                    read_pos: 6,
                    ch: ' '
                },
                errors: vec![],
                current_token: Token {
                    token_type: TokenType::Let,
                    literal: "let".into()
                },
                peek_token: Token {
                    token_type: TokenType::Integer,
                    literal: "5".into()
                }
            }
        )
    }

    #[test]
    fn test_let_statements() {
        let tests = [
            ("let x = 5;", "Let x int 5"),
            ("let y = true;", "Let y bool true"),
            ("let foobar = y;", "Let foobar y"),
        ];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = [
            ("return 5;", "return int 5"),
            ("return 10;", "return int 10"),
            ("return 993322;", "return int 993322"),
            ("return;", "return None"),
        ];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";

        assert_eq!(make_stmt_string(&input), "foobar".to_string());
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        assert_eq!(make_stmt_string(&input), "int 5".to_string());
    }

    #[test]
    fn test_prefix_expression() {
        let tests = vec![
            ("!5;", "(!int 5)"),
            ("-15;", "(-int 15)"),
            ("!true;", "(!bool true)"),
            ("!false;", "(!bool false)"),
        ];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_infix_expression() {
        let tests = vec![
            ("6 + 5;", "(int 6 + int 5)"),
            ("6 - 5;", "(int 6 - int 5)"),
            ("6 * 5;", "(int 6 * int 5)"),
            ("6 / 5;", "(int 6 / int 5)"),
            ("6 > 5;", "(int 6 > int 5)"),
            ("6 < 5;", "(int 6 < int 5)"),
            ("6 == 5;", "(int 6 == int 5)"),
            ("6 != 5;", "(int 6 != int 5)"),
            ("true == true;", "(bool true == bool true)"),
            ("true != true;", "(bool true != bool true)"),
            ("false == false;", "(bool false == bool false)"),
        ];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_operator_precedence() {
        let tests = vec![
            ("-a * b;", "((-a) * b)"),
            ("!-a;", "(!(-a))"),
            ("a + b + c;", "((a + b) + c)"),
            ("a + b - c;", "((a + b) - c)"),
            ("a * b * c;", "((a * b) * c)"),
            ("a * b / c;", "((a * b) / c)"),
            ("a + b / c;", "(a + (b / c))"),
            ("a + b * c + d / e - f;", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5;", "(int 3 + int 4)((-int 5) * int 5)"),
            ("5 > 4 == 3 < 4;", "((int 5 > int 4) == (int 3 < int 4))"),
            ("5 < 4 != 3 > 4;", "((int 5 < int 4) != (int 3 > int 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5;",
                "((int 3 + (int 4 * int 5)) == ((int 3 * int 1) + (int 4 * int 5)))",
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5;",
                "((int 3 + (int 4 * int 5)) == ((int 3 * int 1) + (int 4 * int 5)))",
            ),
            ("true;", "bool true"),
            ("false;", "bool false"),
            ("3 > 5 == false;", "((int 3 > int 5) == bool false)"),
            ("3 < 5 == true;", "((int 3 < int 5) == bool true)"),
            ("1 + (2 + 3) + 4;", "((int 1 + (int 2 + int 3)) + int 4)"),
            ("(5 + 5) * 2;", "((int 5 + int 5) * int 2)"),
            ("2 / (5 + 5);", "(int 2 / (int 5 + int 5))"),
            ("-(5 + 5);", "(-(int 5 + int 5))"),
            ("!(true == true)", "(!(bool true == bool true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, int 1, (int 2 * int 3), (int 4 + int 5), add(int 6, (int 7 * int 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            (
                "a * [1, 2, 3, 4][b * c] * d",
                "((a * [int 1, int 2, int 3, int 4] ) * d)",
            ),
            (
                "add(a * b[2], b[1], 2 * [1, 2][1])",
                "add((a * b ), b , (int 2 * [int 1, int 2] ))",
            ),
        ];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_if_expression() {
        let tests = vec![(
            "if (x < y) { x };",
            "if (x < y) then Block Stmt x else None",
        )];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = vec![(
            "if (x < y) { x } else { y };",
            "if (x < y) then Block Stmt x else Block Stmt y",
        )];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let tests = vec![("fn(x, y) { x + y; }", "fn x y Block Stmt (x + y)")];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_function_parameter_parsing() {
        let tests = vec![
            ("fn() {};", "fn Block "),
            ("fn(x) {};", "fn x Block "),
            ("fn(x, y, z) {};", "fn x y z Block "),
        ];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_call_expression_parsing() {
        let tests = vec![(
            "add(1, 2 * 3, 4 + 5)",
            "add(int 1, (int 2 * int 3), (int 4 + int 5))",
        )];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_string_literal() {
        let tests = vec![("\"hello world\";", "hello world")];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_array() {
        let tests = vec![(
            "[1, 2 * 2, 3 + 3]",
            "[int 1, (int 2 * int 2), (int 3 + int 3)]",
        )];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_index_expression() {
        let tests = vec![("myArray[1 + 1]", "myArray ")];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }

    #[test]
    fn test_hash() {
        let tests = vec![
            ("{\"one\": 1}", "one : int 1"),
            (
                "{\"one\": 1, \"two\": 2, \"three\": 3}",
                "one : int 1 two : int 2 three : int 3",
            ),
            ("{}", ""),
            (
                "{\"one\": 0 + 1, \"two\": 10 - 8, \"three\": 15 / 5}",
                "one : (int 0 + int 1) two : (int 10 - int 8) three : (int 15 / int 5)",
            ),
        ];

        for (input, output) in tests.iter() {
            assert_eq!(make_stmt_string(input), *output);
        }
    }
}
