use crate::environment::node::Node;

pub struct Binding {
    pub identifier: &'static str,
    pub value: Node,
    pub next: Box<Binding>,
}

impl Binding {
    fn new() {}
}
