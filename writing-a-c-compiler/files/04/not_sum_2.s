.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	$3, -4(%rbp)
	movl	$44, %r10d
	subl	%r10d, -4(%rbp)
	movl	-4(%rbp), %r11d
	cmpl	$0, %r11d
	movl	$0, -8(%rbp)
	sete	-8(%rbp)
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
