use std::collections::BTreeMap;
use std::fmt;

use crate::ast::node::Node;
use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Identifier { token, value }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    pub fn new(token: Token, value: i64) -> Self {
        IntegerLiteral { token, value }
    }
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "int {}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Node,
}

impl PrefixExpression {
    pub fn new(token: Token, operator: String, right: Node) -> Self {
        PrefixExpression {
            token,
            operator,
            right,
        }
    }
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Node,
    pub operator: String,
    pub right: Node,
}

impl InfixExpression {
    pub fn new(token: Token, left: Node, operator: String, right: Node) -> Self {
        InfixExpression {
            token,
            left,
            operator,
            right,
        }
    }
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Boolean {
    pub fn new(token: Token, value: bool) -> Self {
        Boolean { token, value }
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bool {}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Node>,
    pub consequence: Box<Node>,
    pub alternative: Option<Box<Node>>,
}

impl IfExpression {
    pub fn new(
        token: Token,
        condition: Box<Node>,
        consequence: Box<Node>,
        alternative: Option<Box<Node>>,
    ) -> Self {
        IfExpression {
            token,
            condition,
            consequence,
            alternative,
        }
    }
}

impl fmt::Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.alternative {
            Some(alternative) => write!(
                f,
                "if {} then {} else {}",
                self.condition, self.consequence, alternative
            ),
            None => write!(
                f,
                "if {} then {} else None",
                self.condition, self.consequence
            ),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Box<Node>,
}

impl FunctionLiteral {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: Box<Node>) -> Self {
        FunctionLiteral {
            token,
            parameters,
            body,
        }
    }
}

impl fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.token.literal)?;
        let mut parameters = self.parameters.iter().peekable();
        while let Some(parameter) = parameters.next() {
            write!(f, "{} ", parameter)?;
        }
        write!(f, "{}", self.body)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Node,
    pub arguments: Vec<Node>,
}

impl CallExpression {
    pub fn new(token: Token, function: Node, arguments: Vec<Node>) -> Self {
        CallExpression {
            token,
            function,
            arguments,
        }
    }
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.function)?;
        let mut arguments = self.arguments.iter().peekable();
        while let Some(argument) = arguments.next() {
            if arguments.peek().is_none() {
                write!(f, "{}", argument)?;
            } else {
                write!(f, "{}, ", argument)?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl StringLiteral {
    pub fn new(token: Token, value: String) -> Self {
        StringLiteral { token, value }
    }
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub token: Token,
    pub elements: Vec<Node>,
}

impl Array {
    pub fn new(token: Token, elements: Vec<Node>) -> Self {
        Array { token, elements }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut it = self.elements.iter().peekable();
        while let Some(element) = it.next() {
            if it.peek().is_none() {
                write!(f, "{}", element)?
            } else {
                write!(f, "{}, ", element)?
            }
        }
        write!(f, "]")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Box<Node>,
    pub index: Box<Node>,
}

impl IndexExpression {
    pub fn new(token: Token, left: Box<Node>, index: Box<Node>) -> Self {
        IndexExpression { token, left, index }
    }
}

impl fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.left)
    }
}

#[derive(Debug, Clone)]
pub struct Hash {
    pub token: Token,
    pub pairs: BTreeMap<Node, Node>,
}

impl Hash {
    pub fn new(token: Token, pairs: BTreeMap<Node, Node>) -> Self {
        Hash { token, pairs }
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.pairs.iter().peekable();
        while let Some(pair) = it.next() {
            if it.peek().is_none() {
                write!(f, "{} : {}", pair.0, pair.1)?;
            } else {
                write!(f, "{} : {} ", pair.0, pair.1)?;
            }
        }
        Ok(())
    }
}

impl PartialEq for Hash {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}
