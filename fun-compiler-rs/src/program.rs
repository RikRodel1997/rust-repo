use std::fmt;

use crate::node::Node;

#[derive(PartialEq)]
pub struct Program {
    pub nodes: Vec<Node>,
}

impl Program {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.nodes {
            write!(f, "{:?}\n", node)?;
        }
        Ok(())
    }
}
