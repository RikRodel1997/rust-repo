pub mod expressions;
pub mod node;
pub mod statements;

use std::fmt;

use crate::ast::node::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Node>,
}

impl Program {
    pub fn new(statements: Option<Vec<Node>>) -> Self {
        match statements {
            Some(nodes) => Self { statements: nodes },
            None => Self { statements: vec![] },
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program ")?;
        for statement in self.statements.iter() {
            write!(f, "Stmt {}", statement)?;
        }
        Ok(())
    }
}
