.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$4, %rsp
	movl	$35, -4(%rbp)
	movl	$2, %ecx
	shll	%cl, -4(%rbp)
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
