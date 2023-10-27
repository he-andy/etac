	.intel_syntax noprefix
	.text
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 72
	and rsp, -16
	mov rcx, rdi
	mov QWORD PTR [rbp-8], rcx
	mov rdx, 24
	mov QWORD PTR [rbp-16], rdx
	mov rax, QWORD PTR [rbp-16]
	mov rdi, rax
	call _eta_alloc
	mov rcx, rax
	mov QWORD PTR [rbp-24], rcx
	mov rax, QWORD PTR [rbp-24]
	mov rdx, rax
	mov QWORD PTR [rbp-32], rdx
	mov rdx, QWORD PTR [rbp-32]
	mov rcx, rdx
	mov QWORD PTR [rbp-40], rcx
	mov rax, QWORD PTR [rbp-40]
	mov QWORD PTR [rax], 2
	mov rcx, QWORD PTR [rbp-40]
	mov QWORD PTR [rcx+8], 58
	mov rdx, QWORD PTR [rbp-40]
	mov QWORD PTR [rdx+16], 68
	mov rax, QWORD PTR [rbp-48]
	mov rcx, QWORD PTR [rbp-40]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-48], rax
	mov rax, QWORD PTR [rbp-48]
	mov rdx, rax
	mov QWORD PTR [rbp-56], rdx
	mov rdx, QWORD PTR [rbp-56]
	mov rcx, rdx
	mov QWORD PTR [rbp-64], rcx
	mov rax, QWORD PTR [rbp-64]
	mov rdi, rax
	call _Iprint_pai
	leave 
	ret 
__epilogue__main__:
	leave 
	ret 
