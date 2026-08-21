.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	$5, -4(%rbp)
	negl	-4(%rbp)
	movl	-4(%rbp), %r10d
	movl	%r10d, -8(%rbp)
	movl	$30, %ecx
	shrl	%cl, -8(%rbp)
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
