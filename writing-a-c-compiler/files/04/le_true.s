.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$12, %rsp
	movl	$0, %r11d
	cmpl	$2, %r11d
	movl	$0, -4(%rbp)
	setle	-4(%rbp)
	movl	$0, %r11d
	cmpl	$0, %r11d
	movl	$0, -8(%rbp)
	setle	-8(%rbp)
	movl	-4(%rbp), %r10d
	movl	%r10d, -12(%rbp)
	movl	-8(%rbp), %r10d
	addl	%r10d, -12(%rbp)
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
