.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	$2, %r11d
	cmpl	$0, %r11d
	movl	$0, -4(%rbp)
	setge	-4(%rbp)
	movl	$2, %r11d
	cmpl	-4(%rbp), %r11d
	movl	$0, -8(%rbp)
	sete	-8(%rbp)
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
