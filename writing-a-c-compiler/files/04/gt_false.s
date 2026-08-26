.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$12, %rsp
	movl	$1, %r11d
	cmpl	$2, %r11d
	movl	$0, -4(%rbp)
	setg	-4(%rbp)
	movl	$1, %r11d
	cmpl	$1, %r11d
	movl	$0, -8(%rbp)
	setg	-8(%rbp)
	movl	-4(%rbp), %r10d
	movl	%r10d, -12(%rbp)
	movl	-8(%rbp), %r10d
	addl	%r10d, -12(%rbp)
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
