	.intel_syntax noprefix
	.text
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 248
	and rsp, -16
	mov rcx, rdi
	mov QWORD PTR [rbp-8], rcx
	mov rdx, 16
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
	mov QWORD PTR [rax], 1
	mov rcx, QWORD PTR [rbp-40]
	mov QWORD PTR [rcx+8], 97
	mov rdx, QWORD PTR [rbp-48]
	mov rax, QWORD PTR [rbp-40]
	lea rdx, QWORD PTR [rax+8]
	mov QWORD PTR [rbp-48], rdx
	mov rdx, QWORD PTR [rbp-48]
	mov rcx, rdx
	mov QWORD PTR [rbp-56], rcx
	mov rax, 1
	mov QWORD PTR [rbp-64], rax
	mov rcx, QWORD PTR [rbp-64]
	mov rdi, rcx
	call _IunparseInt_aii
	mov rdx, rax
	mov QWORD PTR [rbp-24], rdx
	mov rcx, QWORD PTR [rbp-24]
	mov rax, rcx
	mov QWORD PTR [rbp-72], rax
	mov rax, QWORD PTR [rbp-72]
	mov rdx, rax
	mov QWORD PTR [rbp-80], rdx
	mov rdx, QWORD PTR [rbp-56]
	mov rcx, rdx
	mov QWORD PTR [rbp-88], rcx
	mov rax, QWORD PTR [rbp-88]
	sub rax, 8
	mov QWORD PTR [rbp-88], rax
	mov rdx, QWORD PTR [rbp-88]
	mov rcx, QWORD PTR [rdx]
	mov QWORD PTR [rbp-96], rcx
	mov rcx, QWORD PTR [rbp-80]
	mov rax, rcx
	mov QWORD PTR [rbp-104], rax
	mov rdx, QWORD PTR [rbp-104]
	sub rdx, 8
	mov QWORD PTR [rbp-104], rdx
	mov rcx, QWORD PTR [rbp-104]
	mov rax, QWORD PTR [rcx]
	mov QWORD PTR [rbp-112], rax
	mov rdx, QWORD PTR [rbp-120]
	mov rax, QWORD PTR [rbp-96]
	mov rcx, QWORD PTR [rbp-112]
	lea rdx, QWORD PTR [rax+rcx]
	mov QWORD PTR [rbp-120], rdx
	mov rax, QWORD PTR [rbp-120]
	mov rdx, rax
	mov QWORD PTR [rbp-128], rdx
	mov rcx, QWORD PTR [rbp-136]
	mov rdx, QWORD PTR [rbp-128]
	lea rcx, QWORD PTR [rdx*8+8]
	mov QWORD PTR [rbp-136], rcx
	mov rcx, QWORD PTR [rbp-136]
	mov rax, rcx
	mov QWORD PTR [rbp-144], rax
	mov rdx, QWORD PTR [rbp-144]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-152], rcx
	mov rcx, QWORD PTR [rbp-152]
	mov rax, rcx
	mov QWORD PTR [rbp-160], rax
	mov rdx, QWORD PTR [rbp-160]
	mov rax, QWORD PTR [rbp-128]
	mov QWORD PTR [rdx], rax
	mov rcx, 1
	mov QWORD PTR [rbp-168], rcx
_lh9:
	mov rdx, QWORD PTR [rbp-176]
	mov rax, QWORD PTR [rbp-96]
	lea rdx, QWORD PTR [rax+1]
	mov QWORD PTR [rbp-176], rdx
	mov rcx, QWORD PTR [rbp-168]
	mov rdx, QWORD PTR [rbp-176]
	cmp rcx, rdx
	jne _lt10
	mov rax, 1
	mov QWORD PTR [rbp-168], rax
_lhh12:
	mov rcx, QWORD PTR [rbp-184]
	mov rdx, QWORD PTR [rbp-112]
	lea rcx, QWORD PTR [rdx+1]
	mov QWORD PTR [rbp-184], rcx
	mov rax, QWORD PTR [rbp-168]
	mov rcx, QWORD PTR [rbp-184]
	cmp rax, rcx
	jne _ltt13
	mov rdx, QWORD PTR [rbp-192]
	mov rax, QWORD PTR [rbp-160]
	lea rdx, QWORD PTR [rax+8]
	mov QWORD PTR [rbp-192], rdx
	mov rdx, QWORD PTR [rbp-192]
	mov rcx, rdx
	mov QWORD PTR [rbp-200], rcx
	mov rax, QWORD PTR [rbp-200]
	mov rdi, rax
	call _Iprint_pai
	leave 
	ret 
_lt10:
	mov rdx, QWORD PTR [rbp-56]
	mov rcx, rdx
	mov QWORD PTR [rbp-208], rcx
	mov rax, QWORD PTR [rbp-208]
	sub rax, 8
	mov QWORD PTR [rbp-208], rax
	mov rdx, QWORD PTR [rbp-208]
	mov rax, QWORD PTR [rbp-168]
	mov rcx, QWORD PTR [rdx+rax*8]
	mov QWORD PTR [rbp-216], rcx
	mov rcx, QWORD PTR [rbp-160]
	mov rdx, QWORD PTR [rbp-168]
	mov rax, QWORD PTR [rbp-216]
	mov QWORD PTR [rcx+rdx*8], rax
	mov rcx, QWORD PTR [rbp-168]
	add rcx, 1
	mov QWORD PTR [rbp-168], rcx
	jmp _lh9
_ltt13:
	mov rdx, QWORD PTR [rbp-224]
	mov rax, QWORD PTR [rbp-96]
	mov rcx, QWORD PTR [rbp-168]
	lea rdx, QWORD PTR [rax+rcx]
	mov QWORD PTR [rbp-224], rdx
	mov rax, QWORD PTR [rbp-80]
	mov rdx, rax
	mov QWORD PTR [rbp-232], rdx
	mov rcx, QWORD PTR [rbp-232]
	sub rcx, 8
	mov QWORD PTR [rbp-232], rcx
	mov rax, QWORD PTR [rbp-232]
	mov rcx, QWORD PTR [rbp-168]
	mov rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-240], rdx
	mov rdx, QWORD PTR [rbp-160]
	mov rax, QWORD PTR [rbp-224]
	mov rcx, QWORD PTR [rbp-240]
	mov QWORD PTR [rdx+rax*8], rcx
	mov rdx, QWORD PTR [rbp-168]
	add rdx, 1
	mov QWORD PTR [rbp-168], rdx
	jmp _lhh12
