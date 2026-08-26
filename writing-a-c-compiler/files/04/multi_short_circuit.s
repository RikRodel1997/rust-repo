.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$12, %rsp
	movl	$0, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.0
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
	jmp	.Ltmp_end.1
.Ltmp_false.0:
	movl	$0, -8(%rbp)
.Ltmp_end.1:
	movl	-8(%rbp), %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.0
	movl	$0, -12(%rbp)
	jmp	.Ltmp_end.0
.Ltmp_true.0:
	movl	$1, -12(%rbp)
.Ltmp_end.0:
	movl	-12(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
