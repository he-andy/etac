	.intel_syntax noprefix
	.text
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 168
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
	call _Iprintln_pai
	mov rdx, 1
	mov QWORD PTR [rbp-48], rdx
	mov rax, QWORD PTR [rbp-48]
	mov rdi, rax
	call _IunparseInt_aii
	mov rcx, rax
	mov QWORD PTR [rbp-24], rcx
	mov rax, QWORD PTR [rbp-24]
	mov rdx, rax
	mov QWORD PTR [rbp-56], rdx
	mov rdx, QWORD PTR [rbp-56]
	mov rcx, rdx
	mov QWORD PTR [rbp-64], rcx
	mov rax, QWORD PTR [rbp-64]
	mov rdi, rax
	call _Iprintln_pai
	mov rcx, 8
	mov QWORD PTR [rbp-72], rcx
	mov rdx, QWORD PTR [rbp-72]
	mov rdi, rdx
	call _IunparseInt_aii
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-80], rcx
	mov rcx, QWORD PTR [rbp-80]
	mov rax, rcx
	mov QWORD PTR [rbp-88], rax
	mov rdx, QWORD PTR [rbp-88]
	mov rdi, rdx
	call _Iprintln_pai
	mov rax, 0
	mov QWORD PTR [rbp-96], rax
	mov rcx, QWORD PTR [rbp-96]
	mov rdi, rcx
	call _IunparseInt_aii
	mov rdx, rax
	mov QWORD PTR [rbp-24], rdx
	mov rcx, QWORD PTR [rbp-24]
	mov rax, rcx
	mov QWORD PTR [rbp-104], rax
	mov rax, QWORD PTR [rbp-104]
	mov rdx, rax
	mov QWORD PTR [rbp-112], rdx
	mov rcx, QWORD PTR [rbp-112]
	mov rdi, rcx
	call _Iprintln_pai
	mov rdx, 3
	mov QWORD PTR [rbp-120], rdx
	mov rax, QWORD PTR [rbp-120]
	mov rdi, rax
	call _IunparseInt_aii
	mov rcx, rax
	mov QWORD PTR [rbp-24], rcx
	mov rax, QWORD PTR [rbp-24]
	mov rdx, rax
	mov QWORD PTR [rbp-128], rdx
	mov rdx, QWORD PTR [rbp-128]
	mov rcx, rdx
	mov QWORD PTR [rbp-136], rcx
	mov rax, QWORD PTR [rbp-136]
	mov rdi, rax
	call _Iprintln_pai
	mov rcx, 1
	mov QWORD PTR [rbp-144], rcx
	mov rdx, QWORD PTR [rbp-144]
	mov rdi, rdx
	call _IunparseInt_aii
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-152], rcx
	mov rcx, QWORD PTR [rbp-152]
	mov rax, rcx
	mov QWORD PTR [rbp-160], rax
	mov rdx, QWORD PTR [rbp-160]
	mov rdi, rdx
	call _Iprintln_pai
	leave 
	ret 
