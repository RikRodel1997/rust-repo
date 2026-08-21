.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$4, %rsp
	movl	$2, -4(%rbp)
	notl	-4(%rbp)
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
