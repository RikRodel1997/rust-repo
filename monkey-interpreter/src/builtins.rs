use std::fmt;

use crate::object::{Array, Integer, Null, Object};

#[derive(Clone, Debug, PartialEq)]
pub enum BuiltinFunction {
    Len,
    First,
    Last,
    Rest,
    Push,
    Puts,
}

impl BuiltinFunction {
    pub fn call(&self, args: &[Object]) -> Object {
        match self {
            BuiltinFunction::Len => len(args),
            BuiltinFunction::First => first(args),
            BuiltinFunction::Last => last(args),
            BuiltinFunction::Rest => rest(args),
            BuiltinFunction::Push => push(args),
            BuiltinFunction::Puts => puts(args),
        }
    }
}

fn puts(args: &[Object]) -> Object {
    for arg in args.iter() {
        println!("{}", arg)
    }
    Object::Null(Null::new())
}

fn push(args: &[Object]) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "wrong number of arguments. Expected 2, got {}.",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(array) => {
            let mut elements = array.elements.clone();
            elements.push(args[1].clone());
            return Object::Array(Array::new(elements));
        }
        _ => Object::Error(format!(
            "`push()` not supported for type {}",
            &args[0].object_type()
        )),
    }
}

fn rest(args: &[Object]) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. Expected 1, got {}.",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(array) => {
            let length = array.elements.len();
            if length > 0 {
                let elements = array.elements[1..].to_vec();
                Object::Array(Array::new(elements))
            } else {
                Object::Null(Null::new())
            }
        }
        _ => Object::Error(format!(
            "`rest()` not supported for type {}",
            &args[0].object_type()
        )),
    }
}

fn last(args: &[Object]) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. Expected 1, got {}.",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(array) => array
            .elements
            .last()
            .expect("Expected array to have at least 1 item")
            .clone(),
        _ => Object::Error(format!(
            "`last()` not supported for type {}",
            &args[0].object_type()
        )),
    }
}

fn first(args: &[Object]) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. Expected 1, got {}.",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(array) => array
            .elements
            .get(0)
            .expect("Expected array to have at least 1 item")
            .clone(),
        _ => Object::Error(format!(
            "`first()` not supported for type {}",
            &args[0].object_type()
        )),
    }
}

fn len(args: &[Object]) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. Expected 1, got {}.",
            args.len()
        ));
    }

    match &args[0] {
        Object::String(string) => Object::Integer(Integer::new(string.value.len() as i64)),
        Object::Array(array) => Object::Integer(Integer::new(array.elements.len() as i64)),
        _ => Object::Error(format!(
            "`len()` not supported for type {}",
            &args[0].object_type()
        )),
    }
}

impl TryFrom<&str> for BuiltinFunction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "len" => Ok(BuiltinFunction::Len),
            "first" => Ok(BuiltinFunction::First),
            "last" => Ok(BuiltinFunction::Last),
            "rest" => Ok(BuiltinFunction::Rest),
            "push" => Ok(BuiltinFunction::Push),
            "puts" => Ok(BuiltinFunction::Puts),
            _ => Err(()),
        }
    }
}

impl fmt::Display for BuiltinFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Len => write!(f, "len"),
            Self::First => write!(f, "first"),
            Self::Last => write!(f, "last"),
            Self::Rest => write!(f, "rest"),
            Self::Push => write!(f, "push"),
            Self::Puts => write!(f, "puts"),
        }
    }
}
