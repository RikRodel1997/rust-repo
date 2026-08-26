.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$4, %rsp
	movl	$0, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.0
	movl	$0, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.0
	movl	$0, -4(%rbp)
	jmp	.Ltmp_end.0
.Ltmp_true.0:
	movl	$1, -4(%rbp)
.Ltmp_end.0:
	movl	-4(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
