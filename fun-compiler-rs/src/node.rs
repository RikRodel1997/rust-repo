use std::fmt;

pub type Symbol = String;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    Value(Option<Type>),
    BinaryOperator(BinaryOperator),
    VariableDeclaration(Symbol),
    VariableReAssignment(Symbol),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Integer(i64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    ForwardSlash,
}

#[derive(PartialEq, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub children: Vec<Node>,
    pub next: Option<Box<Node>>,
}

impl Node {
    pub fn new(kind: NodeKind, children: Vec<Node>, next: Option<Box<Node>>) -> Self {
        Node {
            kind,
            children,
            next,
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn find_kind(&mut self, kind: NodeKind) -> Option<&mut Node> {
        if self.kind == kind {
            Some(self)
        } else {
            self.children
                .iter_mut()
                .find_map(|child| child.find_kind(kind.clone()))
        }
    }

    fn fmt_tree(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        for _ in 0..indent {
            write!(f, " ")?;
        }
        writeln!(f, "{:?}", self.kind)?;

        let mut current = self.next.as_deref();
        while let Some(node) = current {
            for _ in 0..indent {
                write!(f, " ")?;
            }
            write!(f, "└── ")?;
            node.fmt_tree(f, indent + 1)?;
            current = node.next.as_deref();
        }

        for child in self.children.iter() {
            for _ in 0..indent {
                write!(f, " ")?;
            }
            write!(f, "└── ")?;
            child.fmt_tree(f, indent + 1)?;
        }

        Ok(())
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_tree(f, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_child() {
        let mut root = Node::new(NodeKind::VariableDeclaration("x".to_string()), vec![], None);
        let child = Node::new(NodeKind::Value(Some(Type::Integer(1))), vec![], None);
        root.add_child(child);
        assert_eq!(root.children.len(), 1);
        assert_eq!(
            root.children[0].kind,
            NodeKind::Value(Some(Type::Integer(1)))
        );
    }

    #[test]
    fn test_find_kind() {
        let mut root = Node::new(
            NodeKind::VariableDeclaration("x".to_string()),
            vec![Node::new(
                NodeKind::Value(Some(Type::Integer(1))),
                vec![],
                None,
            )],
            None,
        );
        let result = root.find_kind(NodeKind::Value(Some(Type::Integer(1))));
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().kind,
            NodeKind::Value(Some(Type::Integer(1)))
        );
    }

    #[test]
    fn test_find_kind_nested() {
        let mut root = Node::new(
            NodeKind::VariableDeclaration("x".to_string()),
            vec![Node::new(
                NodeKind::Value(Some(Type::Integer(1))),
                vec![Node::new(
                    NodeKind::Value(Some(Type::Integer(2))),
                    vec![],
                    None,
                )],
                None,
            )],
            None,
        );
        let result = root.find_kind(NodeKind::Value(Some(Type::Integer(2))));
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().kind,
            NodeKind::Value(Some(Type::Integer(2)))
        );
    }

    #[test]
    fn test_find_kind_none() {
        let mut root = Node::new(
            NodeKind::VariableDeclaration("x".to_string()),
            vec![Node::new(
                NodeKind::Value(Some(Type::Integer(1))),
                vec![Node::new(
                    NodeKind::Value(Some(Type::Integer(2))),
                    vec![],
                    None,
                )],
                None,
            )],
            None,
        );
        let result = root.find_kind(NodeKind::Value(Some(Type::Integer(3))));
        assert!(result.is_none());
    }

    #[test]
    fn test_node_debug() {
        let ast = Node::new(
            NodeKind::VariableDeclaration("x".to_string()),
            vec![],
            Some(Box::new(Node::new(
                NodeKind::Value(Some(Type::Integer(1))),
                vec![
                    Node::new(NodeKind::Value(Some(Type::Integer(2))), vec![], None),
                    Node::new(NodeKind::Value(Some(Type::Integer(3))), vec![], None),
                ],
                Some(Box::new(Node::new(
                    NodeKind::Value(Some(Type::Integer(4))),
                    vec![],
                    None,
                ))),
            ))),
        );
        assert_eq!(
            format!("{:?}", ast),
            "VariableDeclaration(\"x\")\n└──  Value(Some(Integer(1)))\n └──   Value(Some(Integer(4)))\n └──   Value(Some(Integer(2)))\n └──   Value(Some(Integer(3)))\n└──  Value(Some(Integer(4)))\n"
        );
    }
}
