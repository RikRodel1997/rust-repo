.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$4, %rsp
	movl	$2147483647, -4(%rbp)
	negl	-4(%rbp)
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
