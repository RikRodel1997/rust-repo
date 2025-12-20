use crate::{
    environment::{
        Environment,
        node::{Node, NodeKind, NodeValue},
    },
    errors::Error,
    lexer::DELIMITERS,
};

pub fn parse(tokens: Vec<&str>) -> Result<Node, Error> {
    Ok(Node {
        kind: NodeKind::Program,
        value: NodeValue::Integer(1),
        children: Box::new(Vec::new()),
    })
}

fn is_valid_identifier(identifier: &str) -> bool {
    if identifier.contains(DELIMITERS) {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = vec![
            "a", ":", "integer", "=", "0", "a", ":", "=", "0", "defun", "foo", "(", "a", ":",
            "integer", ",", "b", ":", "integer", ")", ":", "integer", "{", "}",
        ];
        let actual = parse(tokens).unwrap();
    }
}
