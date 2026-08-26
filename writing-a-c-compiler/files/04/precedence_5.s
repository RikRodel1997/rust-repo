.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$24, %rsp
	movl	$0, %r11d
	cmpl	$0, %r11d
	movl	$0, -4(%rbp)
	sete	-4(%rbp)
	movl	-4(%rbp), %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$2, -8(%rbp)
	movl	$1, %r10d
	addl	%r10d, -8(%rbp)
	movl	-8(%rbp), %r11d
	cmpl	$1, %r11d
	movl	$0, -12(%rbp)
	setg	-12(%rbp)
	movl	$3, %r11d
	cmpl	-12(%rbp), %r11d
	movl	$0, -16(%rbp)
	sete	-16(%rbp)
	movl	-16(%rbp), %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$1, -20(%rbp)
	jmp	.Ltmp_end.0
.Ltmp_false.0:
	movl	$0, -20(%rbp)
.Ltmp_end.0:
	movl	-20(%rbp), %r10d
	movl	%r10d, -24(%rbp)
	movl	$1, %r10d
	addl	%r10d, -24(%rbp)
	movl	-24(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
