.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$0, %rsp
	movl	$100, %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
