pub mod binding;
pub mod node;

use binding::*;
use node::*;

use crate::environment::binding::Binding;

pub struct Environment {
    pub parent: Box<Environment>,
    pub binding: Binding,
}

impl Environment {
    fn new() {}
}
