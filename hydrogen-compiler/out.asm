global _start
_start:
    mov rax, 5
    push rax
    mov rax, 60
    pop rdi
    syscall
    mov rax, 60
    syscall
