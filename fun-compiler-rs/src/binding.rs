use crate::node::Node;

#[derive(Debug, Clone)]
pub struct Binding {
    pub identifier: Option<Node>,
    pub value: Option<Node>,
    pub next: Option<Box<Binding>>,
}

impl Binding {
    pub fn new(identifier: Option<Node>, value: Option<Node>, next: Option<Box<Binding>>) -> Self {
        Binding {
            identifier,
            value,
            next,
        }
    }
}
