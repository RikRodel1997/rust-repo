.globl main
main:
	push	%rbp
	movq	%rsp, %rbp
	subq	$20, %rsp
	movl	$0, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$1, %r11d
	cmpl	$0, %r11d
	je	.Ltmp_false.0
	movl	$1, -4(%rbp)
	jmp	.Ltmp_end.0
.Ltmp_false.0:
	movl	$0, -4(%rbp)
.Ltmp_end.0:
	movl	-4(%rbp), %r10d
	movl	%r10d, -8(%rbp)
	notl	-8(%rbp)
	movl	$4, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.0
	movl	$3, %r11d
	cmpl	$0, %r11d
	jne	.Ltmp_true.0
	movl	$0, -12(%rbp)
	jmp	.Ltmp_end.1
.Ltmp_true.0:
	movl	$1, -12(%rbp)
.Ltmp_end.1:
	movl	-12(%rbp), %r10d
	movl	%r10d, -16(%rbp)
	negl	-16(%rbp)
	movl	-8(%rbp), %r10d
	movl	%r10d, -20(%rbp)
	movl	-16(%rbp), %r10d
	subl	%r10d, -20(%rbp)
	movl	-20(%rbp), %eax
	movq	%rbp, %rsp
	popq	%rbp
	ret
