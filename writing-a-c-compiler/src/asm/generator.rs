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

        Ok(asm_code)
    }

    fn node_asm(&self, node: &'a AsmNode) -> Result<String, String> {
        match node {
            AsmNode::Function { name, instructions } => {
                let mut asm_code = format!(".globl {name}\n{name}:\n");
                asm_code.push_str("\tpush\t%rbp\n");
                asm_code.push_str("\tmovq\t%rsp, %rbp\n");
                let mut instructions = instructions.iter();
                while let Some(instruction) = instructions.next() {
                    asm_code.push_str(&instruction.to_asm());
                }
                Ok(asm_code)
            }
            _ => Err(format!("unknown node to generate ASM {}", node)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::AsmParser;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::tacky::TackyParser;

    use super::*;

    #[test]
    fn test_cmp_generation() -> Result<(), String> {
        let input = "int main(void) { return 1 && 2; }";
        let tokens = Lexer::new(&input).lex()?;
        let ast = Parser::new(&tokens).parse()?;
        let tacky_ir = TackyParser::new(&ast).parse()?;
        let mut asm_parser = AsmParser::new(&tacky_ir);
        asm_parser.parse()?;
        asm_parser.parse_pseudo()?;
        asm_parser.fix_instructions()?;

        let asm_code = AsmGenerator::new(&asm_parser.asm_ast.unwrap()).generate()?;

        assert_eq!(
            asm_code,
            ".globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$4, %rsp
	movl	$1, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$2, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$1, -4(%rbp)
	jmp	.Ltmp_end.0
.Ltmp_false.0:
	movl	$0, -4(%rbp)
.Ltmp_end.0:
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
"
        );

        Ok(())
    }

    #[test]
    fn test_less_than_or_equal() -> Result<(), String> {
        let input = "int main(void) { return 1 <= -1; }";
        let tokens = Lexer::new(&input).lex()?;
        let ast = Parser::new(&tokens).parse()?;
        let tacky_ir = TackyParser::new(&ast).parse()?;

        let mut asm_parser = AsmParser::new(&tacky_ir);
        asm_parser.parse()?;
        asm_parser.parse_pseudo()?;
        asm_parser.fix_instructions()?;

        let asm_code = AsmGenerator::new(&asm_parser.asm_ast.unwrap()).generate()?;

        assert_eq!(
            asm_code,
            ".globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	$1, -4(%rbp)
	negl	-4(%rbp)
	movl	$1, %r11d
	cmpl	-4(%rbp), %r11d
	movl	$0, -8(%rbp)
	setle	-8(%rbp)
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
"
        );

        Ok(())
    }

    #[test]
    fn test_binary_generation() -> Result<(), String> {
        let input = "int main(void) { return 1 + 2; }";
        let tokens = Lexer::new(&input).lex()?;
        let ast = Parser::new(&tokens).parse()?;
        let tacky_ir = TackyParser::new(&ast).parse()?;
        let mut asm_parser = AsmParser::new(&tacky_ir);
        asm_parser.parse()?;
        asm_parser.parse_pseudo()?;
        asm_parser.fix_instructions()?;

        let asm_code = AsmGenerator::new(&asm_parser.asm_ast.unwrap()).generate()?;

        assert_eq!(
            asm_code,
            ".globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$4, %rsp
	movl	$1, -4(%rbp)
	movl	$2, %r10d
	addl	%r10d, -4(%rbp)
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
"
        );

        Ok(())
    }
}
