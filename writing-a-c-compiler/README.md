# Writing A C Compiler

Rust implementation for Nora Sandlers 'Writing A C Compiler' book.

## ASDL

The ASDL is a programming language agnostic way to describe an AST that will be made by a parser. All terms on the left of the `=` represent an AST node, while everything on the right represents what the node is made out of.

```
program = Program(function)
function = Function(ident name, stmt body)
stmt = Return(exp)
exp = Constant(int) | Unary(unary_operator, exp)
unary_operator = Complement | Negate
```

## Formal Grammar
The formal grammar describes a more strict ruleset for the grammar of a particular programming language. It follows the ASDL somewhat closely, but has more details. For example, the ASDL leaves out that a function needs to have an opening and closing brace.

```
<program> = <function>
<function> = "int" <ident> "(" "void" ")" "{" <stmt> "}"
<stmt> = "return" <exp> ";"
<exp> = <int> | <unop> <exp> | "(" <exp> ")"
<unop> = "-" | "~"
<ident> = ? An identifier token ?
<int> = ? A constant token ?
```

## Tacky ASDL

```
program = Program(function)
function = Function(identifier, 1 instruction* body)
instruction = Return(val) | Unary(unop, val src, val dst)
val = Constant(int) | Var(identifier)
unop = Complement | Negate
```

## Assembly ASDL

```
program = Program(function)
function = Function(identifier name, instruction* instructions)
instruction = Mov(operand src, operand dst) 
            | Unary(unop, operand)
            | AllocateStack(int)
            | Ret
unop = Neg | Not
operand = Imm(int) | Reg(reg) | Pseudo(identifier) | Stack(int)
reg = AX | R10
```