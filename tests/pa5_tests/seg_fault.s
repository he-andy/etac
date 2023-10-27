	.intel_syntax noprefix
	.text
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 144
	and rsp, -16
	mov rcx, QWORD PTR [rbp-8]
	mov rcx, rdi
	mov QWORD PTR [rbp-8], rcx
	mov rdx, QWORD PTR [rbp-16]
	mov rdx, 16
	mov QWORD PTR [rbp-16], rdx
	mov rax, QWORD PTR [rbp-16]
	mov rdi, rax
	call _eta_alloc
	mov rcx, QWORD PTR [rbp-24]
	mov rdx, QWORD PTR [rbp-32]
	mov rcx, rdx
	mov QWORD PTR [rbp-24], rcx
	mov rax, QWORD PTR [rbp-40]
	mov rcx, QWORD PTR [rbp-24]
	mov rax, rcx
	mov QWORD PTR [rbp-40], rax
	mov rdx, QWORD PTR [rbp-40]
	mov QWORD PTR [rdx], 1
	mov rax, QWORD PTR [rbp-48]
	mov rcx, QWORD PTR [rbp-40]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-48], rax
	mov rdx, QWORD PTR [rbp-48]
	mov QWORD PTR [rdx], 1
	mov rax, QWORD PTR [rbp-56]
	mov rcx, QWORD PTR [rbp-40]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-56], rax
	mov rdx, QWORD PTR [rbp-64]
	mov rax, QWORD PTR [rbp-56]
	mov rdx, rax
	mov QWORD PTR [rbp-64], rdx
	mov rcx, QWORD PTR [rbp-72]
	mov rdx, QWORD PTR [rbp-64]
	mov rcx, rdx
	mov QWORD PTR [rbp-72], rcx
	mov rax, QWORD PTR [rbp-80]
	mov rax, 1
	mov QWORD PTR [rbp-80], rax
	mov rcx, QWORD PTR [rbp-88]
	mov rdx, QWORD PTR [rbp-72]
	mov rcx, rdx
	mov QWORD PTR [rbp-88], rcx
	mov rax, QWORD PTR [rbp-88]
	sub rax, 8
	mov QWORD PTR [rbp-88], rax
	mov rcx, QWORD PTR [rbp-80]
	mov rdx, QWORD PTR [rbp-88]
	cmp rcx, QWORD PTR [rdx]
	mov rax, QWORD PTR [rbp-96]
	xor rax, rax
	setb al
	mov rcx, QWORD PTR [rbp-104]
	mov rcx, 1
	mov QWORD PTR [rbp-104], rcx
	mov rdx, QWORD PTR [rbp-104]
	mov rax, QWORD PTR [rbp-96]
	xor rdx, rax
	mov QWORD PTR [rbp-104], rdx
	mov rcx, QWORD PTR [rbp-104]
	test rcx, rcx
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-112]
	mov rax, QWORD PTR [rbp-72]
	mov rcx, QWORD PTR [rbp-80]
	lea rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-112], rdx
	mov rdx, QWORD PTR [rbp-120]
	mov rax, QWORD PTR [rbp-112]
	mov rdx, QWORD PTR [rax]
	mov QWORD PTR [rbp-120], rdx
	mov rcx, QWORD PTR [rbp-120]
	mov rdi, rcx
	call _IunparseInt_aii
	mov rdx, QWORD PTR [rbp-128]
	mov rax, QWORD PTR [rbp-32]
	mov rdx, rax
	mov QWORD PTR [rbp-128], rdx
	mov rcx, QWORD PTR [rbp-136]
	mov rdx, QWORD PTR [rbp-128]
	mov rcx, rdx
	mov QWORD PTR [rbp-136], rcx
	mov rax, QWORD PTR [rbp-136]
	mov rdi, rax
	call _Iprint_pai
	jmp __epilogue__main__
_eta_out_of_bounds:
	call _eta_out_of_bounds
__epilogue__main__:
	mov rsp, rbp
	pop rbp
	ret 
