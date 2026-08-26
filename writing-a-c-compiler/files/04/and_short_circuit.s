.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$8, %rsp
	movl	$0, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$1, %eax
	cdq
	movl	$0, %r10d
	idivl	%r10d
	movl	%eax, -4(%rbp)
	movl	-4(%rbp), %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$1, -8(%rbp)
	jmp	.Ltmp_end.0
.Ltmp_false.0:
	movl	$0, -8(%rbp)
.Ltmp_end.0:
	movl	-8(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
