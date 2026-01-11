use std::collections::HashMap;

use crate::node::{Node, NodeKind, NodeValue};

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub bindings: HashMap<String, Node>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Environment {
            parent,
            bindings: HashMap::new(),
        }
    }

    pub fn set_binding(&mut self, identifier: String, value: Node) -> () {
        self.bindings.insert(identifier, value);
    }

    pub fn get_binding(&self, identifier: String) -> Option<&Node> {
        self.bindings.get(&identifier)
    }

    pub fn set_type_bindings(&mut self) -> () {
        self.set_binding(
            "integer".into(),
            Node::new(NodeKind::Value(NodeValue::Integer(0)), vec![], None),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{NodeKind, NodeValue};

    #[test]
    fn test_bindings() {
        let mut env = Environment::new(None);
        env.set_binding(
            "test value".into(),
            Node::new(NodeKind::Value(NodeValue::Integer(1)), vec![], None),
        );
        let node = env.get_binding("test value".into());
        assert!(node.is_some());
        assert_eq!(node.unwrap().kind, NodeKind::Value(NodeValue::Integer(1)));
    }
}
