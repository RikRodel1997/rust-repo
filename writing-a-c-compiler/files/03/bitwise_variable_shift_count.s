.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$20, %rsp
	movl	$2, -4(%rbp)
	movl	-4(%rbp), %r11d
	imull	$2, %r11d
	movl	%r11d, -4(%rbp)
	movl	$4, -8(%rbp)
	movl	-4(%rbp), %ecx
	shll	%cl, -8(%rbp)
	movl	$1, -12(%rbp)
	movl	$2, %r10d
	addl	%r10d, -12(%rbp)
	movl	$100, -16(%rbp)
	movl	-12(%rbp), %ecx
	shrl	%cl, -16(%rbp)
	movl	-8(%rbp), %r10d
	movl	%r10d, -20(%rbp)
	movl	-16(%rbp), %r10d
	addl	%r10d, -20(%rbp)
	movl	-20(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
