.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$12, %rsp
	movl	$5, %r11d
	cmpl	$0, %r11d
	movl	$0, -4(%rbp)
	setge	-4(%rbp)
	movl	-4(%rbp), %r11d
	cmpl	$1, %r11d
	movl	$0, -8(%rbp)
	setg	-8(%rbp)
	movl	-8(%rbp), %r11d
	cmpl	$0, %r11d
	movl	$0, -12(%rbp)
	setle	-12(%rbp)
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
