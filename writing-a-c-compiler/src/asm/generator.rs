use crate::asm::ast::AsmNode;

pub struct AsmGenerator<'a> {
    pub ast: &'a AsmNode,
}

impl<'a> AsmGenerator<'a> {
    pub fn new(ast: &'a AsmNode) -> Self {
        Self { ast }
    }

    pub fn generate(&mut self) -> Result<String, String> {
        let mut asm_code = String::new();

        if let AsmNode::Program { function } = self.ast {
            asm_code.push_str(&self.node_asm(*&function)?);
        } else {
            return Err(format!(
                "invalid start node, expected Program got {}",
                self.ast
            ));
        }

        asm_code.push_str(".section .note.GNU-stack,\"\",@progbits");
        Ok(asm_code)
    }

    fn node_asm(&self, node: &'a AsmNode) -> Result<String, String> {
        match node {
            AsmNode::Function { name, instructions } => {
                let mut asm_code = format!(".globl {name}\n{name}:\n");
                asm_code.push_str("\tpush\t%rbp\n");
                asm_code.push_str("\tmovq\t%rsp, %rbp\n");
                let mut instructions = instructions.iter();
                while let Some(AsmNode::Instruction(instruction)) = instructions.next() {
                    asm_code.push_str(&format!("\t{}\n", instruction.to_asm()));
                }
                Ok(asm_code)
            }
            _ => Err(format!("unknown node to generate ASM {}", node)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    use super::*;
    use crate::{Lexer, asm::parser::AsmParser, parser::Parser, tacky::TackyParser};

    #[test]
    fn test_01() -> Result<(), String> {
        fn expected(value: i64) -> String {
            let mut expected = String::from(".globl main\n");
            expected.push_str("main:\n");
            expected.push_str("\tpush\t%rbp\n");
            expected.push_str("\tmovq\t%rsp, %rbp\n");
            expected.push_str(format!("\tmovl\t${value}, %eax\n").as_str());
            expected.push_str("\tmovq\t%rbp, %rsp\n");
            expected.push_str("\tpopq\t%rbp\n");
            expected.push_str("\tret\n");
            expected.push_str(".section .note.GNU-stack,\"\",@progbits");
            expected
        }

        fn generate_file(file_path: &str, buf: &str) -> () {
            let path = format!("{}.s", file_path.split(".c").nth(0).unwrap());
            let mut file = File::create(path).unwrap();
            file.write_all(buf.as_bytes()).unwrap();
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
            let input = fs::read_to_string(file_path.clone()).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;
            let asm_ast = asm_parser.asm_ast.expect("expected asm_ast to be present");
            let actual = AsmGenerator::new(&asm_ast).generate()?;

            generate_file(&file_path, &actual);

            assert_eq!(actual, expected);
        }
        Ok(())
    }

    #[test]
    fn test_02() -> Result<(), String> {
        fn expected(value: i64, stack_size: i8, instructions: Vec<&str>) -> String {
            let mut expected = String::from(".globl main\n");
            expected.push_str("main:\n");
            expected.push_str("\tpush\t%rbp\n");
            expected.push_str("\tmovq\t%rsp, %rbp\n");
            expected.push_str(format!("\tsubq\t${stack_size}, %rsp\n").as_str());
            expected.push_str(format!("\tmovl\t${value}, -4(%rbp)\n").as_str());
            for instruction in instructions.iter() {
                expected.push_str(*instruction);
            }
            expected.push_str("\tmovq\t%rbp, %rsp\n");
            expected.push_str("\tpopq\t%rbp\n");
            expected.push_str("\tret\n");
            expected.push_str(".section .note.GNU-stack,\"\",@progbits");
            expected
        }

        fn generate_file(file_path: &str, buf: &str) -> () {
            let path = format!("{}.s", file_path.split(".c").nth(0).unwrap());
            let mut file = File::create(path).unwrap();
            file.write_all(buf.as_bytes()).unwrap();
        }

        let tests = vec![
            (
                "bitwise_int_min.c",
                expected(
                    2147483647,
                    8,
                    vec![
                        "\tnegl\t-4(%rbp)\n",
                        "\tmovl\t-4(%rbp), %r10d\n",
                        "\tmovl\t%r10d, -8(%rbp)\n",
                        "\tnotl\t-8(%rbp)\n",
                        "\tmovl\t-8(%rbp), %eax\n",
                    ],
                ),
            ),
            (
                "bitwise_zero.c",
                expected(0, 4, vec!["\tnotl\t-4(%rbp)\n", "\tmovl\t-4(%rbp), %eax\n"]),
            ),
            (
                "bitwise.c",
                expected(
                    12,
                    4,
                    vec!["\tnotl\t-4(%rbp)\n", "\tmovl\t-4(%rbp), %eax\n"],
                ),
            ),
            (
                "neg_zero.c",
                expected(0, 4, vec!["\tnegl\t-4(%rbp)\n", "\tmovl\t-4(%rbp), %eax\n"]),
            ),
            (
                "neg.c",
                expected(5, 4, vec!["\tnegl\t-4(%rbp)\n", "\tmovl\t-4(%rbp), %eax\n"]),
            ),
            (
                "negate_int_max.c",
                expected(
                    2147483647,
                    4,
                    vec!["\tnegl\t-4(%rbp)\n", "\tmovl\t-4(%rbp), %eax\n"],
                ),
            ),
            (
                "nested_ops_2.c",
                expected(
                    0,
                    8,
                    vec![
                        "\tnotl\t-4(%rbp)\n",
                        "\tmovl\t-4(%rbp), %r10d\n",
                        "\tmovl\t%r10d, -8(%rbp)\n",
                        "\tnegl\t-8(%rbp)\n",
                        "\tmovl\t-8(%rbp), %eax\n",
                    ],
                ),
            ),
            (
                "nested_ops.c",
                expected(
                    3,
                    8,
                    vec![
                        "\tnegl\t-4(%rbp)\n",
                        "\tmovl\t-4(%rbp), %r10d\n",
                        "\tmovl\t%r10d, -8(%rbp)\n",
                        "\tnotl\t-8(%rbp)\n",
                        "\tmovl\t-8(%rbp), %eax\n",
                    ],
                ),
            ),
            (
                "parens_2.c",
                expected(2, 4, vec!["\tnotl\t-4(%rbp)\n", "\tmovl\t-4(%rbp), %eax\n"]),
            ),
            (
                "parens_3.c",
                expected(
                    4,
                    8,
                    vec![
                        "\tnegl\t-4(%rbp)\n",
                        "\tmovl\t-4(%rbp), %r10d\n",
                        "\tmovl\t%r10d, -8(%rbp)\n",
                        "\tnegl\t-8(%rbp)\n",
                        "\tmovl\t-8(%rbp), %eax\n",
                    ],
                ),
            ),
            (
                "parens.c",
                expected(2, 4, vec!["\tnegl\t-4(%rbp)\n", "\tmovl\t-4(%rbp), %eax\n"]),
            ),
            (
                "redundant_parens.c",
                expected(
                    10,
                    4,
                    vec!["\tnegl\t-4(%rbp)\n", "\tmovl\t-4(%rbp), %eax\n"],
                ),
            ),
        ];

        for (file, expected) in tests.into_iter() {
            let file_path = format!("files/02/{file}");
            let input = fs::read_to_string(file_path.clone()).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;
            asm_parser.parse_pseudo()?;
            asm_parser.fix_instructions()?;

            let asm_ast = asm_parser.asm_ast.expect("expected asm_ast to be present");
            let actual = AsmGenerator::new(&asm_ast).generate()?;

            assert_eq!(actual, expected);
            generate_file(&file_path, &actual);
        }
        Ok(())
    }

    #[test]
    fn test_03() -> Result<(), String> {
        fn generate_file(file_path: &str, buf: &str) -> () {
            let path = format!("{}.s", file_path.split(".c").nth(0).unwrap());
            let mut file = File::create(path).unwrap();
            file.write_all(buf.as_bytes()).unwrap();
        }

        let entries = fs::read_dir("files/03")
            .expect("test")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("c"))
            .collect::<Vec<_>>();

        for file_path in entries.iter() {
            let input = fs::read_to_string(file_path.clone()).expect("unable to read file");
            let tokens = Lexer::new(&input).lex()?;
            let ast = Parser::new(&tokens).parse()?;
            let tacky_ir = TackyParser::new(&ast).parse()?;
            println!("file_path {:?}", file_path);
            println!("tacky_ir {tacky_ir}");

            let mut asm_parser = AsmParser::new(&tacky_ir);
            asm_parser.parse()?;
            asm_parser.parse_pseudo()?;
            asm_parser.fix_instructions()?;

            let asm_ast = asm_parser.asm_ast.expect("expected asm_ast to be present");
            let actual = AsmGenerator::new(&asm_ast).generate()?;

            generate_file(&file_path.to_str().unwrap(), &actual);
        }

        Ok(())
    }
}
