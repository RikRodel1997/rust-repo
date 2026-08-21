.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	$33, -4(%rbp)
	movl	$4, %ecx
	shll	%cl, -4(%rbp)
	movl	-4(%rbp), %r10d
	movl	%r10d, -8(%rbp)
	movl	$2, %ecx
	shrl	%cl, -8(%rbp)
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
