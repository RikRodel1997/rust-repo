.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$12, %rsp
	movl	$20, -4(%rbp)
	movl	$4, %ecx
	shrl	%cl, -4(%rbp)
	movl	$3, -8(%rbp)
	movl	$1, %ecx
	shll	%cl, -8(%rbp)
	movl	-4(%rbp), %r11d
	cmpl	-8(%rbp), %r11d
	movl	$0, -12(%rbp)
	setg	-12(%rbp)
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
