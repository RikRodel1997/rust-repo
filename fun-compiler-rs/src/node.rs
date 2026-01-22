use std::fmt;

pub type Symbol = String;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    Value(Type),
    BinaryOperator(BinaryOperator),
    VariableDeclaration(Symbol),
    VariableDeclarationInitialized,
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
        let child = Node::new(NodeKind::Value(Type::Integer(1)), vec![], None);
        root.add_child(child);
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].kind, NodeKind::Value(Type::Integer(1)));
    }

    #[test]
    fn test_node_debug() {
        let ast = Node::new(
            NodeKind::VariableDeclaration("x".to_string()),
            vec![],
            Some(Box::new(Node::new(
                NodeKind::Value(Type::Integer(1)),
                vec![
                    Node::new(NodeKind::Value(Type::Integer(2)), vec![], None),
                    Node::new(NodeKind::Value(Type::Integer(3)), vec![], None),
                ],
                Some(Box::new(Node::new(
                    NodeKind::Value(Type::Integer(4)),
                    vec![],
                    None,
                ))),
            ))),
        );
        assert_eq!(
            format!("{:?}", ast),
            "VariableDeclaration(\"x\")\n└──  Value(Integer(1))\n └──   Value(Integer(4))\n └──   Value(Integer(2))\n └──   Value(Integer(3))\n└──  Value(Integer(4))\n"
        );
    }
}
