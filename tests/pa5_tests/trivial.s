	.intel_syntax noprefix
	.text
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 48
	and rsp, -16
	mov rdx, rdi
	mov QWORD PTR [rbp-8], rdx
	mov rax, 3
	mov QWORD PTR [rbp-16], rax
	mov rcx, QWORD PTR [rbp-16]
	mov rdi, rcx
	call _IunparseInt_aii
	mov rdx, rax
	mov QWORD PTR [rbp-24], rdx
	mov rcx, QWORD PTR [rbp-24]
	mov rax, rcx
	mov QWORD PTR [rbp-32], rax
	mov rax, QWORD PTR [rbp-32]
	mov rdx, rax
	mov QWORD PTR [rbp-40], rdx
	mov rcx, QWORD PTR [rbp-40]
	mov rdi, rcx
	call _Iprint_pai
	leave 
	ret 
