use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::{collections::HashMap, fmt};

use crate::ast::expressions::Identifier;
use crate::ast::node::Node;
use crate::builtins::BuiltinFunction;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null(Null),
    Return(Box<Object>),
    Error(String),
    Function(Function),
    String(StringLiteral),
    Builtin(BuiltinFunction),
    Array(Array),
    Hash(HashObj),
}

impl Object {
    pub fn object_type(&self) -> String {
        match &self {
            Self::Integer(_) => "INTEGER".into(),
            Self::Boolean(_) => "BOOLEAN".into(),
            Self::Null(_) => "NULL".into(),
            Self::Return(_) => "RETURN".into(),
            Self::Error(_) => "ERROR".into(),
            Self::Function(_) => "FUNCTION".into(),
            Self::String(_) => "STRING".into(),
            Self::Builtin(builtin) => builtin.to_string(),
            Self::Array(_) => "ARRAY".into(),
            Self::Hash(_) => "HASH".into(),
        }
    }

    pub fn hashable(&self) -> bool {
        match &self {
            Self::Integer(_) | Self::Boolean(_) | Self::String(_) => true,
            _ => false,
        }
    }

    pub fn hash_key(&self) -> Option<HashKey> {
        match &self {
            Self::Integer(integer) => Some(HashKey {
                object_type: "INTEGER".to_string(),
                value: integer.value,
            }),
            Self::Boolean(boolean) => Some(HashKey {
                object_type: "BOOLEAN".to_string(),
                value: match boolean.value {
                    true => 1,
                    false => 0,
                },
            }),
            Self::String(string) => {
                let mut hasher = DefaultHasher::new();
                string.value.hash(&mut hasher);
                let hash_value = hasher.finish() as i64;
                Some(HashKey {
                    object_type: "STRING".to_string(),
                    value: hash_value,
                })
            }
            _ => None,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Null(null) => write!(f, "{}", null),
            Self::Return(value) => write!(f, "{}", *value),
            Self::Error(message) => write!(f, "{}", message),
            Self::Function(function) => write!(f, "{}", function),
            Self::String(string) => write!(f, "{}", string),
            Self::Builtin(builtin) => write!(f, "{}", builtin),
            Self::Array(array) => write!(f, "{}", array),
            Self::Hash(hash) => write!(f, "{}", hash),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer { value }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn new(value: &str) -> Self {
        StringLiteral {
            value: value.into(),
        }
    }
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Boolean {
    pub fn new(value: bool) -> Self {
        Boolean { value }
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Null;

impl Null {
    pub fn new() -> Self {
        Null
    }
}

impl fmt::Display for Null {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "null")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(outer: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            store: HashMap::new(),
            outer,
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone());
        }

        if let Some(outer) = &self.outer {
            return outer.borrow().get(name);
        }

        None
    }

    pub fn set(&mut self, name: &str, object: Object) -> Option<Object> {
        self.store.insert(name.into(), object)
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "env ")?;
        for (key, value) in self.store.clone().into_iter() {
            write!(f, "{} {} ", key, value)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: Box<Node>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        parameters: Vec<Identifier>,
        body: Box<Node>,
        environment: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            parameters,
            body,
            environment,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn ")?;
        write!(f, "params ")?;
        for param in self.parameters.clone().into_iter() {
            write!(f, "{} ", param.value)?;
        }
        write!(f, "body {}", self.body)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Array {
    pub elements: Vec<Object>,
}

impl Array {
    pub fn new(elements: Vec<Object>) -> Self {
        Self { elements }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for element in self.elements.clone().into_iter() {
            write!(f, "{}", element)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Ord, Eq, PartialOrd)]
pub struct HashKey {
    pub object_type: String,
    pub value: i64,
}

impl fmt::Display for HashKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.object_type, self.value)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct HashPair {
    pub key: Box<Object>,
    pub value: Box<Object>,
}

impl HashPair {
    pub fn new(key: Object, value: Object) -> Self {
        Self {
            key: Box::new(key),
            value: Box::new(value),
        }
    }
}

impl fmt::Display for HashPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.key, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct HashObj {
    pub pairs: BTreeMap<HashKey, HashPair>,
}

impl HashObj {
    pub fn new(pairs: BTreeMap<HashKey, HashPair>) -> Self {
        Self { pairs }
    }
}

impl fmt::Display for HashObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.pairs.iter().peekable();
        while let Some(pair) = it.next() {
            if it.peek().is_none() {
                write!(f, "{} : {}", pair.0, pair.1)?;
            } else {
                write!(f, "{} : {} ", pair.0, pair.1)?;
            }
        }
        Ok(())
    }
}

impl PartialEq for HashObj {
    fn eq(&self, other: &Self) -> bool {
        self.pairs == other.pairs // This is correct for BTreeMap
    }
}

impl Eq for HashObj {}
