#[derive(Debug, PartialEq)]
pub enum NodeKind {
    None,
    Integer,
    Program,
}

#[derive(Debug, PartialEq)]
pub enum NodeValue {
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub value: NodeValue,
    pub children: Box<Vec<Node>>,
}

impl Node {
    fn new(&self) {}

    fn is_none(&self) -> bool {
        self.kind == NodeKind::None
    }
}
