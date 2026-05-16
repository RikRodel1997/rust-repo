.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	movl	$0, %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
.section .note.GNU-stack,"",@progbits