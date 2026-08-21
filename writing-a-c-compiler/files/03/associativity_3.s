.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$20, %rsp
	movl	$3, %eax
	cdq
	movl	$2, %r10d
	idivl	%r10d
	movl	%eax, -4(%rbp)
	movl	-4(%rbp), %r10d
	movl	%r10d, -8(%rbp)
	movl	-8(%rbp), %r11d
	imull	$4, %r11d
	movl	%r11d, -8(%rbp)
	movl	$5, -12(%rbp)
	movl	$4, %r10d
	subl	%r10d, -12(%rbp)
	movl	-12(%rbp), %r10d
	movl	%r10d, -16(%rbp)
	movl	$3, %r10d
	addl	%r10d, -16(%rbp)
	movl	-8(%rbp), %r10d
	movl	%r10d, -20(%rbp)
	movl	-16(%rbp), %r10d
	addl	%r10d, -20(%rbp)
	movl	-20(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
