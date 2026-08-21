.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$4, %rsp
	movl	$4, %eax
	cdq
	movl	$2, %r10d
	idivl	%r10d
	movl	%edx, -4(%rbp)
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
