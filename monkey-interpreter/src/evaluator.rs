use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast::expressions::{Identifier, IfExpression};
use crate::ast::node::Node;
use crate::ast::statements::BlockStatement;
use crate::ast::{Program, expressions};
use crate::builtins::BuiltinFunction;
use crate::object::{
    Array, Boolean, Environment, Function, HashObj, HashPair, Integer, Null, Object, StringLiteral,
};

pub fn eval(node: Node, environment: Rc<RefCell<Environment>>) -> Result<Object, String> {
    match node {
        // Statements
        Node::Program(program) => eval_program(program, environment),
        Node::ExpressionStatement(stmt) => match stmt.expression {
            Some(expr) => {
                let object = eval(*expr, environment)?;
                if let Object::Error(_) = object {
                    return Ok(object);
                }
                Ok(object)
            }
            None => Ok(Object::Null(Null::new())),
        },
        Node::BlockStatement(stmt) => eval_block_statements(stmt, environment),
        Node::ReturnStatement(stmt) => match stmt.return_value {
            Some(value) => {
                let object = eval(*value, environment)?;
                if let Object::Error(_) = object {
                    return Ok(object);
                }
                Ok(Object::Return(Box::new(object)))
            }
            None => Ok(Object::Null(Null::new())),
        },
        Node::LetStatement(stmt) => {
            let name = stmt.name.value.clone();
            let value = eval(*stmt.value, Rc::clone(&environment))?;

            if let Object::Error(_) = value {
                return Ok(value);
            }

            if let Object::Function(func) = value.clone() {
                func.environment
                    .borrow_mut()
                    .set(&name, Object::Function(func.clone()));
                environment.borrow_mut().set(&name, Object::Function(func));
            } else {
                environment.borrow_mut().set(&name, value.clone());
            }
            Ok(value)
        }

        // Expressions
        Node::PrefixExpression(expr) => {
            let right = eval(expr.right, environment)?;
            if let Object::Error(_) = right {
                return Ok(right);
            }
            eval_prefix_expression(&expr.operator, right)
        }
        Node::InfixExpression(expr) => {
            let left = eval(expr.left, Rc::clone(&environment))?;
            if let Object::Error(_) = left {
                return Ok(left);
            }

            let right = eval(expr.right, Rc::clone(&environment))?;
            if let Object::Error(_) = right {
                return Ok(right);
            }

            eval_infix_expression(&expr.operator, left, right)
        }
        Node::IfExpression(expr) => eval_if_expression(expr, environment),
        Node::IntegerLiteral(literal) => Ok(Object::Integer(Integer::new(literal.value))),
        Node::Boolean(literal) => Ok(Object::Boolean(Boolean::new(literal.value))),
        Node::Identifier(ident) => identifier(ident, environment),
        Node::FunctionLiteral(function) => Ok(Object::Function(Function::new(
            function.parameters,
            function.body,
            Rc::clone(&environment),
        ))),
        Node::CallExpression(expr) => {
            let args = expr.arguments.clone();
            let function = eval(expr.function, Rc::clone(&environment))?;
            if let Object::Error(_) = function {
                return Ok(function);
            }

            let arguments = eval_expressions(args, Rc::clone(&environment))?;
            if arguments.len() == 1
                && let Some(Object::Error(_)) = arguments.get(0)
            {
                return Ok(arguments.get(0).unwrap().clone());
            }

            apply_function(function, arguments)
        }
        Node::String(literal) => Ok(Object::String(StringLiteral::new(&literal.value))),
        Node::Array(array) => {
            let elements = eval_expressions(array.elements, environment)?;
            if elements.len() == 1
                && let Some(Object::Error(_)) = elements.get(0)
            {
                return Ok(elements.get(0).unwrap().clone());
            }
            return Ok(Object::Array(Array::new(elements)));
        }
        Node::IndexExpression(expr) => {
            let left = eval(*expr.left, Rc::clone(&environment))?;
            if let Object::Error(_) = left {
                return Ok(left);
            }

            let index = eval(*expr.index, Rc::clone(&environment))?;
            if let Object::Error(_) = index {
                return Ok(index);
            }
            eval_index_expression(left, index)
        }
        Node::Hash(hash) => eval_hash_literal(hash, Rc::clone(&environment)),
    }
}

fn eval_program(program: Program, environment: Rc<RefCell<Environment>>) -> Result<Object, String> {
    let mut result = Object::Null(Null::new());

    for statement in program.statements.into_iter() {
        let statement_result = eval(statement, Rc::clone(&environment))?;
        if let Object::Return(return_value) = statement_result {
            return Ok(*return_value);
        }

        if let Object::Error(_) = statement_result {
            return Ok(statement_result);
        }
        result = statement_result
    }

    Ok(result)
}

fn apply_function(function: Object, arguments: Vec<Object>) -> Result<Object, String> {
    if let Object::Function(mut function) = function {
        let extended_environment = extend_function_environment(&mut function, arguments);
        let evaluated = eval(*function.body, extended_environment)?;
        return Ok(unwrap_return_value(evaluated));
    } else if let Object::Builtin(builtin) = function {
        return Ok(builtin.call(&arguments));
    }
    Ok(Object::Error(format!(
        "not a function {}",
        function.object_type()
    )))
}

fn unwrap_return_value(evaluated: Object) -> Object {
    if let Object::Return(return_value) = evaluated {
        return *return_value;
    }
    return evaluated;
}

fn extend_function_environment(
    function: &Function,
    arguments: Vec<Object>,
) -> Rc<RefCell<Environment>> {
    let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
        &function.environment,
    )))));

    for (i, param) in function.parameters.iter().enumerate() {
        env.borrow_mut().set(&param.value, arguments[i].clone());
    }

    env
}

fn eval_hash_literal(
    hash: expressions::Hash,
    environment: Rc<RefCell<Environment>>,
) -> Result<Object, String> {
    let mut result = BTreeMap::new();
    let pairs = hash.pairs;

    for (key, value) in pairs.iter() {
        let key = eval(key.clone(), Rc::clone(&environment))?;
        if let Object::Error(err) = key {
            return Ok(Object::Error(err));
        }

        if !key.hashable() {
            return Ok(Object::Error(format!(
                "object of type {} is not hashable",
                key.object_type()
            )));
        }

        let hash_key = key.hash_key().expect("Expected object to have hash key");
        let value = eval(value.clone(), Rc::clone(&environment))?;
        result.insert(hash_key, HashPair::new(key, value));
    }

    Ok(Object::Hash(HashObj::new(result)))
}

fn eval_index_expression(left: Object, index: Object) -> Result<Object, String> {
    match (left.clone(), index) {
        (Object::Array(left), Object::Integer(index)) => {
            match left.elements.get(index.value as usize) {
                Some(element) => Ok(element.clone()),
                None => Ok(Object::Null(Null::new())),
            }
        }

        (Object::Hash(left), index) => {
            let hash = left;

            if !index.hashable() {
                return Ok(Object::Error(format!(
                    "object of type {} is not hashable",
                    index.object_type()
                )));
            }

            match hash.pairs.get(&index.hash_key().expect("Expected hashkey")) {
                Some(pair) => Ok(*pair.value.clone()),
                None => Ok(Object::Null(Null::new())),
            }
        }
        _ => Ok(Object::Error(format!(
            "index operator not supported: {}",
            left.object_type()
        ))),
    }
}

fn eval_expressions(
    expressions: Vec<Node>,
    environment: Rc<RefCell<Environment>>,
) -> Result<Vec<Object>, String> {
    let mut result = vec![];

    for expr in expressions.into_iter() {
        let evaluated = eval(expr, Rc::clone(&environment))?;
        if let Object::Error(_) = evaluated {
            return Ok(vec![evaluated]);
        }
        result.push(evaluated);
    }

    Ok(result)
}

fn identifier(ident: Identifier, environment: Rc<RefCell<Environment>>) -> Result<Object, String> {
    let value = environment.borrow().get(&ident.value);
    if value.is_some() {
        return Ok(value.unwrap().clone());
    }

    match BuiltinFunction::try_from(ident.value.as_str()) {
        Ok(builtin) => Ok(Object::Builtin(builtin)),
        Err(_) => Ok(Object::Error(format!(
            "identifier not found: {}",
            ident.value
        ))),
    }
}

fn eval_block_statements(
    block: BlockStatement,
    environment: Rc<RefCell<Environment>>,
) -> Result<Object, String> {
    let mut result = Object::Null(Null::new());

    for statement in block.statements.into_iter() {
        let statement_result = eval(statement, Rc::clone(&environment))?;
        if let Object::Return(_) = statement_result {
            return Ok(statement_result);
        }

        if let Object::Error(_) = statement_result {
            return Ok(statement_result);
        }
        result = statement_result
    }

    Ok(result)
}
fn eval_prefix_expression(operator: &str, right: Object) -> Result<Object, String> {
    match operator {
        "!" => Ok(match right {
            Object::Boolean(bool) => Object::Boolean(Boolean::new(!bool.value)),
            _ => Object::Boolean(Boolean::new(false)),
        }),
        "-" => Ok(match right {
            Object::Integer(integer) => Object::Integer(Integer::new(-integer.value)),
            _ => Object::Error(format!("unknown operator: -{}", right.object_type())),
        }),
        _ => Ok(Object::Error(format!(
            "unknown operator: {}{}",
            operator,
            right.to_string()
        ))),
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Result<Object, String> {
    if left.clone().object_type() != right.clone().object_type() {
        return Ok(Object::Error(format!(
            "type mismatch: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        )));
    }
    match (left, right) {
        (Object::Integer(left), Object::Integer(right)) => {
            let left = left.value;
            let right = right.value;
            Ok(match operator {
                "+" => Object::Integer(Integer::new(left + right)),
                "-" => Object::Integer(Integer::new(left - right)),
                "*" => Object::Integer(Integer::new(left * right)),
                "/" => Object::Integer(Integer::new(left / right)),
                "<" => Object::Boolean(Boolean::new(left < right)),
                ">" => Object::Boolean(Boolean::new(left > right)),
                "==" => Object::Boolean(Boolean::new(left == right)),
                "!=" => Object::Boolean(Boolean::new(left != right)),
                _ => Object::Error(format!("unknown operator: INTEGER {} INTEGER", operator)),
            })
        }
        (Object::String(left), Object::String(right)) => {
            let left = left.value;
            let right = right.value;
            Ok(match operator {
                "+" => Object::String(StringLiteral::new((left + &right).as_str())),
                _ => Object::Error(format!("unknown operator: STRING {} STRING", operator)),
            })
        }
        (Object::Boolean(left), Object::Boolean(right)) => match operator {
            "==" => Ok(Object::Boolean(Boolean::new(left == right))),
            "!=" => Ok(Object::Boolean(Boolean::new(left != right))),
            _ => Ok(Object::Error(format!(
                "unknown operator: BOOLEAN {} BOOLEAN",
                operator
            ))),
        },
        (left, right) => Ok(Object::Error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        ))),
    }
}

fn eval_if_expression(
    expr: IfExpression,
    environment: Rc<RefCell<Environment>>,
) -> Result<Object, String> {
    let condition = eval(*expr.condition, Rc::clone(&environment))?;
    if let Object::Error(_) = condition {
        return Ok(condition);
    }
    if is_truthy(condition) {
        eval(*expr.consequence, environment)
    } else if expr.alternative.is_some() {
        eval(*expr.alternative.unwrap(), Rc::clone(&environment))
    } else {
        Ok(Object::Null(Null::new()))
    }
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Null(_) => false,
        Object::Boolean(bool) => bool.value,
        _ => true,
    }
}

#[cfg(test)]
mod test {
    use crate::{
        lexer::Lexer,
        object::{HashKey, HashObj, HashPair},
        parser::Parser,
        token::{Token, TokenType},
    };

    use super::*;

    fn test_eval(input: &str) -> Result<Object, String> {
        let mut lexer = Lexer::new(input.into());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program()?;
        let env = Rc::new(RefCell::new(Environment::new(None)));
        eval(program, env)
    }

    #[test]

    fn test_integer_expression() {
        let tests = vec![
            ("5", Object::Integer(Integer::new(5))),
            ("10", Object::Integer(Integer::new(10))),
            ("-5", Object::Integer(Integer::new(-5))),
            ("-10", Object::Integer(Integer::new(-10))),
            ("5 + 5 + 5 + 5 - 10", Object::Integer(Integer::new(10))),
            ("2 * 2 * 2 * 2 * 2", Object::Integer(Integer::new(32))),
            ("-50 + 100 + -50", Object::Integer(Integer::new(0))),
            ("5 * 2 + 10", Object::Integer(Integer::new(20))),
            ("5 + 2 * 10", Object::Integer(Integer::new(25))),
            ("20 + 2 * -10", Object::Integer(Integer::new(0))),
            ("50 / 2 * 2 + 10", Object::Integer(Integer::new(60))),
            ("2 * (5 + 10)", Object::Integer(Integer::new(30))),
            ("3 * 3 * 3 + 10", Object::Integer(Integer::new(37))),
            ("3 * (3 * 3) + 10", Object::Integer(Integer::new(37))),
            (
                "(5 + 10 * 2 + 15 / 3) * 2 + -10",
                Object::Integer(Integer::new(50)),
            ),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]

    fn test_boolean_expression() {
        let tests = vec![
            ("true", Object::Boolean(Boolean::new(true))),
            ("false", Object::Boolean(Boolean::new(false))),
            ("1 < 2", Object::Boolean(Boolean::new(true))),
            ("1 > 2", Object::Boolean(Boolean::new(false))),
            ("1 < 1", Object::Boolean(Boolean::new(false))),
            ("1 > 1", Object::Boolean(Boolean::new(false))),
            ("1 == 1", Object::Boolean(Boolean::new(true))),
            ("1 != 1", Object::Boolean(Boolean::new(false))),
            ("1 == 2", Object::Boolean(Boolean::new(false))),
            ("1 != 2", Object::Boolean(Boolean::new(true))),
            ("true == true", Object::Boolean(Boolean::new(true))),
            ("false == false", Object::Boolean(Boolean::new(true))),
            ("true == false", Object::Boolean(Boolean::new(false))),
            ("true != false", Object::Boolean(Boolean::new(true))),
            ("false != true", Object::Boolean(Boolean::new(true))),
            ("(1 < 2) == true", Object::Boolean(Boolean::new(true))),
            ("(1 < 2) == false", Object::Boolean(Boolean::new(false))),
            ("(1 > 2) == true", Object::Boolean(Boolean::new(false))),
            ("(1 > 2) == false", Object::Boolean(Boolean::new(true))),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]

    fn test_bang_operator() {
        let tests = vec![
            ("!true", Object::Boolean(Boolean::new(false))),
            ("!false", Object::Boolean(Boolean::new(true))),
            ("!5", Object::Boolean(Boolean::new(false))),
            ("!!true", Object::Boolean(Boolean::new(true))),
            ("!!false", Object::Boolean(Boolean::new(false))),
            ("!!5", Object::Boolean(Boolean::new(true))),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]

    fn test_if_else_expressions() {
        let tests = vec![
            ("if (true) { 10 }", Object::Integer(Integer::new(10))),
            ("if (false) { 10 }", Object::Null(Null::new())),
            ("if (1) { 10 }", Object::Integer(Integer::new(10))),
            ("if (1 < 2 ) { 10 }", Object::Integer(Integer::new(10))),
            ("if (1 > 2 ) { 10 }", Object::Null(Null::new())),
            (
                "if (1 > 2 ) { 10 } else { 20 }",
                Object::Integer(Integer::new(20)),
            ),
            (
                "if (1 < 2 ) { 10 } else { 20 }",
                Object::Integer(Integer::new(10)),
            ),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]

    fn test_return_statement() {
        let tests = vec![
            ("return 10;", Object::Integer(Integer::new(10))),
            ("return 10; 9;", Object::Integer(Integer::new(10))),
            ("return 2 * 5; 9;", Object::Integer(Integer::new(10))),
            ("9; return 2 * 5; 9;", Object::Integer(Integer::new(10))),
            (
                "if (10 > 1) { if (10 > 1) { return 10; } return 1; }",
                Object::Integer(Integer::new(10)),
            ),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]

    fn test_error_handling() {
        let tests = vec![
            (
                "5 + true;",
                Object::Error("type mismatch: INTEGER + BOOLEAN".to_string()),
            ),
            (
                "5 + true; 5;",
                Object::Error("type mismatch: INTEGER + BOOLEAN".to_string()),
            ),
            (
                "-true;",
                Object::Error("unknown operator: -BOOLEAN".to_string()),
            ),
            (
                "true + false;",
                Object::Error("unknown operator: BOOLEAN + BOOLEAN".to_string()),
            ),
            (
                "5; true + false; 5",
                Object::Error("unknown operator: BOOLEAN + BOOLEAN".to_string()),
            ),
            (
                "if (10 > 1) { true + false; }",
                Object::Error("unknown operator: BOOLEAN + BOOLEAN".to_string()),
            ),
            (
                "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }",
                Object::Error("unknown operator: BOOLEAN + BOOLEAN".to_string()),
            ),
            (
                "foobar",
                Object::Error("identifier not found: foobar".to_string()),
            ),
            (
                "\"Hello\" - \"World!\"",
                Object::Error("unknown operator: STRING - STRING".to_string()),
            ),
            (
                "{\"name\": \"Monkey\"}[fn(x) { x }];",
                Object::Error("object of type FUNCTION is not hashable".to_string()),
            ),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_let_statements() {
        let tests = vec![
            ("let a = 5; a;", Object::Integer(Integer::new(5))),
            ("let a = 5 * 5; a;", Object::Integer(Integer::new(25))),
            ("let a = 5; let b = a; b;", Object::Integer(Integer::new(5))),
            (
                "let a = 5; let b = a; let c = a + b + 5; c;",
                Object::Integer(Integer::new(15)),
            ),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_function() {
        let evaluated = test_eval("fn(x) { x + 2; }").unwrap();
        if let Object::Function(function) = evaluated {
            assert_eq!(
                function.parameters,
                vec![Identifier::new(
                    Token::new(TokenType::Ident, "x".to_string()),
                    "x".to_string()
                )]
            )
        } else {
            assert_eq!(true, false);
        }
    }

    #[test]
    fn test_function_applications() {
        let tests = vec![
            (
                "let identity = fn(x) { x; }; identity(5);",
                Object::Integer(Integer::new(5)),
            ),
            (
                "let identity = fn(x) { return x; }; identity(5);",
                Object::Integer(Integer::new(5)),
            ),
            (
                "let double = fn(x) { x * 2; }; double(5);",
                Object::Integer(Integer::new(10)),
            ),
            (
                "let add = fn(x, y) { x + y; }; add(5, 5);",
                Object::Integer(Integer::new(10)),
            ),
            (
                "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
                Object::Integer(Integer::new(20)),
            ),
            ("fn(x) { x; }(5);", Object::Integer(Integer::new(5))),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_closures() {
        let tests = vec![(
            "let newAdder = fn(x) {
                fn(y) { x + y };
            };
            let addTwo = newAdder(2);
            addTwo(2);",
            Object::Integer(Integer::new(4)),
        )];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_string_literal() {
        let tests = vec![(
            "\"Hello World!\"",
            Object::String(StringLiteral::new("Hello World!".into())),
        )];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_string_concatenation() {
        let tests = vec![(
            "\"Hello\" + \" \" + \"World!\"",
            Object::String(StringLiteral::new("Hello World!".into())),
        )];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_len_builtin() {
        let tests = vec![
            ("len(\"\")", Object::Integer(Integer::new(0))),
            ("len(\"four\")", Object::Integer(Integer::new(4))),
            ("len(\"hello world\")", Object::Integer(Integer::new(11))),
            (
                "len(1)",
                Object::Error("`len()` not supported for type INTEGER".into()),
            ),
            (
                "len(\"one\", \"two\")",
                Object::Error("wrong number of arguments. Expected 1, got 2.".into()),
            ),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_array_literals() {
        let tests = vec![(
            "[1, 2 * 2, 3 + 3]",
            Object::Array(Array::new(vec![
                Object::Integer(Integer::new(1)),
                Object::Integer(Integer::new(4)),
                Object::Integer(Integer::new(6)),
            ])),
        )];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_index_expression() {
        let tests = vec![
            ("[1, 2, 3][0]", Object::Integer(Integer::new(1))),
            ("[1, 2, 3][1]", Object::Integer(Integer::new(2))),
            ("[1, 2, 3][2]", Object::Integer(Integer::new(3))),
            ("let i = 0; [1][i];", Object::Integer(Integer::new(1))),
            ("[1, 2, 3][1 + 1];", Object::Integer(Integer::new(3))),
            (
                "let myArray = [1, 2, 3]; myArray[2];",
                Object::Integer(Integer::new(3)),
            ),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                Object::Integer(Integer::new(6)),
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
                Object::Integer(Integer::new(2)),
            ),
            ("[1, 2, 3][3]", Object::Null(Null::new())),
            ("[1, 2, 3][-1]", Object::Null(Null::new())),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_array_builtins() {
        let tests = vec![
            (
                "let map = fn(arr, f) {
                let iter = fn(arr, acc) {
                    if (len(arr) == 0) {
                        acc
                    } else {
                        iter(rest(arr), push(acc, f(first(arr))));
                    }
                };
                iter(arr, []);
            };

            let a = [1, 2, 3, 4];
            let double = fn(x) { x * 2 };
            map(a, double);",
                Object::Array(Array::new(vec![
                    Object::Integer(Integer::new(2)),
                    Object::Integer(Integer::new(4)),
                    Object::Integer(Integer::new(6)),
                    Object::Integer(Integer::new(8)),
                ])),
            ),
            (
                "let reduce = fn(arr, initial, f) {
                    let iter = fn(arr, result) {
                        if (len(arr) == 0) {
                            result
                        } else {
                            iter(rest(arr), f(result, first(arr)));
                        }
                    };
                    iter(arr, initial);
                };

                let sum = fn(arr) {
                    reduce(arr, 0, fn(initial, el) { initial + el });
                };

                sum([1, 2, 3, 4, 5]);",
                Object::Integer(Integer::new(15)),
            ),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_hash() {
        let tests = vec![(
            "let two = \"two\";
                {
                    \"one\": 10 - 9,
                    two: 1 + 1,
                    \"thr\" + \"ee\": 6 / 2,
                    4: 4,
                    true: 5,
                    false: 6
                }",
            Object::Hash(HashObj::new(BTreeMap::from([
                (
                    HashKey {
                        object_type: "BOOLEAN".into(),
                        value: 0,
                    },
                    HashPair::new(
                        Object::Boolean(Boolean::new(false)),
                        Object::Integer(Integer::new(6)),
                    ),
                ),
                (
                    HashKey {
                        object_type: "BOOLEAN".into(),
                        value: 1,
                    },
                    HashPair::new(
                        Object::Boolean(Boolean::new(true)),
                        Object::Integer(Integer::new(5)),
                    ),
                ),
                (
                    HashKey {
                        object_type: "INTEGER".into(),
                        value: 4,
                    },
                    HashPair::new(
                        Object::Integer(Integer::new(4)),
                        Object::Integer(Integer::new(4)),
                    ),
                ),
                (
                    HashKey {
                        object_type: "STRING".into(),
                        value: -1184685006806621382,
                    },
                    HashPair::new(
                        Object::String(StringLiteral::new("three")),
                        Object::Integer(Integer::new(3)),
                    ),
                ),
                (
                    HashKey {
                        object_type: "STRING".into(),
                        value: -3989856919191216141,
                    },
                    HashPair::new(
                        Object::String(StringLiteral::new("two")),
                        Object::Integer(Integer::new(2)),
                    ),
                ),
                (
                    HashKey {
                        object_type: "STRING".into(),
                        value: -4370429131690437635,
                    },
                    HashPair::new(
                        Object::String(StringLiteral::new("one")),
                        Object::Integer(Integer::new(1)),
                    ),
                ),
            ]))),
        )];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_hash_index_expressions() {
        let tests = vec![
            ("{\"foo\": 5}[\"foo\"]", Object::Integer(Integer::new(5))),
            ("{\"foo\": 5}[\"bar\"]", Object::Null(Null::new())),
            (
                "let key = \"foo\"; {\"foo\": 5}[key]",
                Object::Integer(Integer::new(5)),
            ),
            ("{}[\"foo\"]", Object::Null(Null::new())),
            ("{5: 5}[5]", Object::Integer(Integer::new(5))),
            ("{true: 5}[true]", Object::Integer(Integer::new(5))),
            ("{false: 5}[false]", Object::Integer(Integer::new(5))),
        ];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }

    #[test]
    fn test_puts_builtin() {
        let tests = vec![("puts(\"Hello World!\")", Object::Null(Null::new()))];

        for test in tests.iter() {
            let evaluated = test_eval(test.0).unwrap();
            assert_eq!(evaluated, test.1);
        }
    }
}
