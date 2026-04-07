use crate::{asm_gen::ast::AsmNode, parser::*};

pub struct AsmGen {
    pub ast: Node,
}

impl AsmGen {
    pub fn new(ast: Node) -> Self {
        Self { ast }
    }

    pub fn generate(&mut self) -> Result<AsmNode, String> {}
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::Lexer;

    #[test]
    fn test_01() {
        fn expected(value: i64) -> Node {
            return Node::Program {
                function: Box::new(Node::Function {
                    name: Box::new(Node::Ident("main".into())),
                    body: Box::new(Node::Stmt(Stmt::Return(Box::new(Node::Expr(
                        Expr::Constant(Int { value }),
                    ))))),
                }),
            };
        }

        let tests = vec![
            ("multi_digit.c", expected(100)),
            ("newlines.c", expected(0)),
            ("no_newlines.c", expected(0)),
            ("return_0.c", expected(0)),
            ("return_2.c", expected(2)),
            ("spaces.c", expected(0)),
            ("tabs.c", expected(0)),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/01/{file}");
            let input = fs::read_to_string(file_path).expect("unable to read file");
            let tokens = Lexer::new(&input).lex().expect("lexing failed");
            let ast = Parser::new(&tokens).parse().expect("parsing failed");
            let actual = AsmGen::new(ast).generate().expect("asm generation failed");
            assert_eq!(actual, expected);
        }
    }
}
