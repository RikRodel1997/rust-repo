.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	$1, -4(%rbp)
	movl	$2, %r10d
	subl	%r10d, -4(%rbp)
	movl	-4(%rbp), %r10d
	movl	%r10d, -8(%rbp)
	movl	$3, %r10d
	subl	%r10d, -8(%rbp)
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
