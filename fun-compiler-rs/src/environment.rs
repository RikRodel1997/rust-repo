use crate::binding::Binding;
use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Environment {
    pub parent: Option<Box<Environment>>,
    pub binding: Option<Binding>,
}

impl Environment {
    pub fn new(parent: Option<Box<Environment>>) -> Self {
        Environment {
            parent,
            binding: None,
        }
    }

    pub fn set_binding(&mut self, binding: Binding) -> () {
        if let Some(binding) = self.binding.clone() {
            self.binding = Some(Binding::new(
                binding.identifier.clone(),
                binding.value.clone(),
                Some(Box::new(binding)),
            ));
        } else {
            self.binding = Some(Binding::new(binding.identifier, binding.value, None));
        }
    }

    pub fn get_binding(&self, identifier: &Node) -> Option<&Binding> {
        self.binding
            .as_ref()
            .filter(|b| b.identifier.as_ref().unwrap() == identifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{NodeKind, NodeValue};

    #[test]
    fn test_set_binding() {
        let mut env = Environment::new(None);
        let binding = Binding::new(
            Some(Node::new(NodeKind::Program, Box::new(vec![]), None)),
            Some(Node::new(
                NodeKind::Value(NodeValue::Integer(1)),
                Box::new(vec![]),
                None,
            )),
            None,
        );
        env.set_binding(binding);
        assert!(env.binding.is_some());

        let bind = env.binding.unwrap();
        assert_eq!(
            bind.identifier,
            Some(Node::new(NodeKind::Program, Box::new(vec![]), None))
        );
        assert_eq!(
            bind.value,
            Some(Node::new(
                NodeKind::Value(NodeValue::Integer(1)),
                Box::new(vec![]),
                None,
            ))
        );
        assert!(bind.next.is_none());
    }

    #[test]
    fn test_get_binding() {
        let mut env = Environment::new(None);
        let binding = Binding::new(
            Some(Node::new(NodeKind::Program, Box::new(vec![]), None)),
            Some(Node::new(
                NodeKind::Value(NodeValue::Integer(1)),
                Box::new(vec![]),
                None,
            )),
            None,
        );
        env.set_binding(binding);

        let result = env.get_binding(&Node::new(NodeKind::Program, Box::new(vec![]), None));
        assert!(result.is_some());

        let bind = result.unwrap();
        assert_eq!(
            bind.identifier,
            Some(Node::new(NodeKind::Program, Box::new(vec![]), None))
        );
        assert_eq!(
            bind.value,
            Some(Node::new(
                NodeKind::Value(NodeValue::Integer(1)),
                Box::new(vec![]),
                None,
            ))
        );
        assert!(bind.next.is_none());
    }

    #[test]
    fn test_get_binding_nested() {
        let mut env = Environment::new(None);
        let binding = Binding::new(
            Some(Node::new(NodeKind::Program, Box::new(vec![]), None)),
            Some(Node::new(
                NodeKind::Value(NodeValue::Integer(1)),
                Box::new(vec![]),
                None,
            )),
            None,
        );
        env.set_binding(binding);

        let result = env.get_binding(&Node::new(NodeKind::Program, Box::new(vec![]), None));
        assert!(result.is_some());

        let bind = result.unwrap();
        assert_eq!(
            bind.identifier,
            Some(Node::new(NodeKind::Program, Box::new(vec![]), None))
        );
        assert_eq!(
            bind.value,
            Some(Node::new(
                NodeKind::Value(NodeValue::Integer(1)),
                Box::new(vec![]),
                None,
            ))
        );
        assert!(bind.next.is_none());
    }
}
