.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	$12, -4(%rbp)
	negl	-4(%rbp)
	movl	-4(%rbp), %eax
	cdq
	movl	$5, %r10d
	idivl	%r10d
	movl	%eax, -8(%rbp)
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
