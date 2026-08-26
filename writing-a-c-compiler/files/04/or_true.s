.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$20, %rsp
	movl	$4, %r11d
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
	movl	$0, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.1
	movl	$3, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.1
	movl	$0, -8(%rbp)
	jmp	.Ltmp_end.1
.Ltmp_true.1:
	movl	$1, -8(%rbp)
.Ltmp_end.1:
	movl	-4(%rbp), %r10d
	movl	%r10d, -12(%rbp)
	movl	-8(%rbp), %r10d
	addl	%r10d, -12(%rbp)
	movl	$5, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.2
	movl	$5, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.2
	movl	$0, -16(%rbp)
	jmp	.Ltmp_end.2
.Ltmp_true.2:
	movl	$1, -16(%rbp)
.Ltmp_end.2:
	movl	-12(%rbp), %r10d
	movl	%r10d, -20(%rbp)
	movl	-16(%rbp), %r10d
	addl	%r10d, -20(%rbp)
	movl	-20(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
