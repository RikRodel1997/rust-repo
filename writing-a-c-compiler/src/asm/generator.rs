use crate::asm::ast::{AsmNode, Instruction, Operand};

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
                let mut instructions = instructions.iter();
                while let Some(AsmNode::Instruction(instruction)) = instructions.next() {
                    asm_code.push_str(&self.instruction_asm(instruction)?);
                }
                Ok(asm_code)
            }
            _ => Err(format!("unknown node to generate ASM {}", node)),
        }
    }

    fn instruction_asm(&self, instruction: &Instruction) -> Result<String, String> {
        match instruction {
            Instruction::Mov { src, dst } => Ok(format!(
                "\tmovl {}, {}\n",
                self.operand_asm(src)?,
                self.operand_asm(dst)?
            )),
            Instruction::Ret => Ok("\tret\n".into()),
        }
    }

    fn operand_asm(&self, operand: &Operand) -> Result<String, String> {
        match operand {
            Operand::Imm(value) => Ok(format!("${value}")),
            Operand::Register => Ok("%eax".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;

    use super::*;
    use crate::{Lexer, asm::parser::AsmParser, parser::Parser};

    #[test]
    fn test_01() {
        fn expected(value: i64) -> String {
            format!(
                ".globl main\nmain:\n\tmovl ${value}, %eax\n\tret\n.section .note.GNU-stack,\"\",@progbits"
            )
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
            let tokens = Lexer::new(&input).lex().expect("lexing failed");
            let ast = Parser::new(&tokens).parse().expect("parsing failed");
            let asm_ast = AsmParser::new(&ast).parse().expect("asm parsing failed");
            let actual = AsmGenerator::new(&asm_ast)
                .generate()
                .expect("asm code generation failed");

            generate_file(&file_path, &actual);

            assert_eq!(actual, expected);
        }
    }
}
