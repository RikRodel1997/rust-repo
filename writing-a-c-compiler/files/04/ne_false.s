.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$4, %rsp
	movl	$0, %r11d
	cmpl	$0, %r11d
	movl	$0, -4(%rbp)
	setne	-4(%rbp)
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
