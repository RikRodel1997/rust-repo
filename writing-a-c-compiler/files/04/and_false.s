.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$20, %rsp
	movl	$10, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$0, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$1, -4(%rbp)
	jmp	.Ltmp_end.0
.Ltmp_false.0:
	movl	$0, -4(%rbp)
.Ltmp_end.0:
	movl	$0, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.1
	movl	$4, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.1
	movl	$1, -8(%rbp)
	jmp	.Ltmp_end.1
.Ltmp_false.1:
	movl	$0, -8(%rbp)
.Ltmp_end.1:
	movl	-4(%rbp), %r10d
	movl	%r10d, -12(%rbp)
	movl	-8(%rbp), %r10d
	addl	%r10d, -12(%rbp)
	movl	$0, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.2
	movl	$0, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.2
	movl	$1, -16(%rbp)
	jmp	.Ltmp_end.2
.Ltmp_false.2:
	movl	$0, -16(%rbp)
.Ltmp_end.2:
	movl	-12(%rbp), %r10d
	movl	%r10d, -20(%rbp)
	movl	-16(%rbp), %r10d
	addl	%r10d, -20(%rbp)
	movl	-20(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
