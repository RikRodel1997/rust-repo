.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$12, %rsp
	movl	$1, -4(%rbp)
	negl	-4(%rbp)
	movl	$2, -8(%rbp)
	negl	-8(%rbp)
	movl	-4(%rbp), %r11d
	cmpl	-8(%rbp), %r11d
	movl	$0, -12(%rbp)
	setne	-12(%rbp)
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
