use std::{fs::File, io::Write};

use crate::{
    parser::{Ast, Exprs, NodeExit, NodeExpr},
    tokens::TokenValue,
};

pub fn generate(ast: Ast) -> String {
    let mut asm = String::from("global _start\n_start:\n");

    match &ast.root.expr {
        node_expr => asm.push_str(&generate_node_expr(node_expr)),
    }

    asm.push_str(&builtin(&ast.root));
    asm
}

fn builtin(_node: &NodeExit) -> String {
    String::from("    mov rax, 60\n    syscall")
}

fn generate_node_expr(node: &Option<NodeExpr>) -> String {
    let expr = node.as_ref().unwrap();
    match expr.kind {
        Exprs::Literal => String::from(format!(
            "    mov rdi, {}\n",
            match expr.value.clone() {
                TokenValue::Integer(int) => int.to_string(),
                TokenValue::Float(flt) => {
                    println!("flt {flt}");
                    flt.to_string()
                }
                TokenValue::String(str) => str.to_string(),
            }
        )),
    }
}

pub fn print_asm(asm: &str) {
    println!("asm\n\n{asm}\n");
}

pub fn write_asm(asm: String) -> Result<(), std::io::Error> {
    let mut file = File::create("out.asm")?;
    file.write_all(asm.as_bytes())?;
    Ok(())
}
