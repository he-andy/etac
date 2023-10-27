	.intel_syntax noprefix
	.text
	.globl _IAck_iii
_IAck_iii:
	push rbp
	mov rbp, rsp
	sub rsp, 160
	and rsp, -16
	mov rdx, rdi
	mov QWORD PTR [rbp-8], rdx
	mov rax, rsi
	mov QWORD PTR [rbp-16], rax
	mov rcx, QWORD PTR [rbp-8]
	cmp rcx, 0
	je _lt5
	mov rdx, QWORD PTR [rbp-16]
	cmp rdx, 0
	mov rax, QWORD PTR [rbp-24]
	sete al
	movsx rax, al
	mov QWORD PTR [rbp-24], rax
	mov rcx, 1
	mov QWORD PTR [rbp-32], rcx
	mov rdx, QWORD PTR [rbp-32]
	mov rax, QWORD PTR [rbp-24]
	xor rdx, rax
	mov QWORD PTR [rbp-32], rdx
	mov rcx, QWORD PTR [rbp-32]
	test rcx, rcx
	jnz _lf9
	mov rax, QWORD PTR [rbp-8]
	mov rdx, rax
	mov QWORD PTR [rbp-40], rdx
	mov rcx, QWORD PTR [rbp-40]
	sub rcx, 1
	mov QWORD PTR [rbp-40], rcx
	mov rax, QWORD PTR [rbp-40]
	mov rdx, rax
	mov QWORD PTR [rbp-48], rdx
	mov rcx, 1
	mov QWORD PTR [rbp-56], rcx
	mov rdx, QWORD PTR [rbp-48]
	mov rdi, rdx
	mov rax, QWORD PTR [rbp-56]
	mov rsi, rax
	call _IAck_iii
	mov rcx, rax
	mov QWORD PTR [rbp-64], rcx
	mov rax, QWORD PTR [rbp-64]
	mov rdx, rax
	mov QWORD PTR [rbp-72], rdx
	mov rdx, QWORD PTR [rbp-72]
	mov rcx, rdx
	mov QWORD PTR [rbp-64], rcx
	mov rax, QWORD PTR [rbp-64]
	mov rax, rax
	leave 
	ret 
_endif10:
_endif7:
_lt5:
	mov rcx, QWORD PTR [rbp-80]
	mov rdx, QWORD PTR [rbp-16]
	lea rcx, QWORD PTR [rdx+1]
	mov QWORD PTR [rbp-80], rcx
	mov rcx, QWORD PTR [rbp-80]
	mov rax, rcx
	mov QWORD PTR [rbp-64], rax
	mov rdx, QWORD PTR [rbp-64]
	mov rax, rdx
	leave 
	ret 
	jmp _endif7
_lf9:
	mov rcx, QWORD PTR [rbp-8]
	mov rax, rcx
	mov QWORD PTR [rbp-88], rax
	mov rdx, QWORD PTR [rbp-88]
	sub rdx, 1
	mov QWORD PTR [rbp-88], rdx
	mov rcx, QWORD PTR [rbp-88]
	mov rax, rcx
	mov QWORD PTR [rbp-96], rax
	mov rax, QWORD PTR [rbp-8]
	mov rdx, rax
	mov QWORD PTR [rbp-104], rdx
	mov rdx, QWORD PTR [rbp-16]
	mov rcx, rdx
	mov QWORD PTR [rbp-112], rcx
	mov rax, QWORD PTR [rbp-112]
	sub rax, 1
	mov QWORD PTR [rbp-112], rax
	mov rdx, QWORD PTR [rbp-112]
	mov rcx, rdx
	mov QWORD PTR [rbp-120], rcx
	mov rax, QWORD PTR [rbp-104]
	mov rdi, rax
	mov rcx, QWORD PTR [rbp-120]
	mov rsi, rcx
	call _IAck_iii
	mov rdx, rax
	mov QWORD PTR [rbp-64], rdx
	mov rcx, QWORD PTR [rbp-64]
	mov rax, rcx
	mov QWORD PTR [rbp-128], rax
	mov rax, QWORD PTR [rbp-128]
	mov rdx, rax
	mov QWORD PTR [rbp-136], rdx
	mov rcx, QWORD PTR [rbp-96]
	mov rdi, rcx
	mov rdx, QWORD PTR [rbp-136]
	mov rsi, rdx
	call _IAck_iii
	mov rax, rax
	mov QWORD PTR [rbp-64], rax
	mov rdx, QWORD PTR [rbp-64]
	mov rcx, rdx
	mov QWORD PTR [rbp-144], rcx
	mov rcx, QWORD PTR [rbp-144]
	mov rax, rcx
	mov QWORD PTR [rbp-64], rax
	mov rdx, QWORD PTR [rbp-64]
	mov rax, rdx
	leave 
	ret 
	jmp _endif10
	.globl _Iusage_p
_Iusage_p:
	push rbp
	mov rbp, rsp
	sub rsp, 56
	and rsp, -16
	mov rdx, 240
	mov QWORD PTR [rbp-8], rdx
	mov rax, QWORD PTR [rbp-8]
	mov rdi, rax
	call _eta_alloc
	mov rcx, rax
	mov QWORD PTR [rbp-16], rcx
	mov rax, QWORD PTR [rbp-16]
	mov rdx, rax
	mov QWORD PTR [rbp-24], rdx
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-32], rcx
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax], 29
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+8], 80
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+16], 108
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+24], 101
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+32], 97
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+40], 115
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+48], 101
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+56], 32
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+64], 115
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+72], 112
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+80], 101
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+88], 99
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+96], 105
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+104], 102
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+112], 121
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+120], 32
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+128], 116
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+136], 104
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+144], 101
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+152], 32
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+160], 105
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+168], 110
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+176], 112
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+184], 117
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+192], 116
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+200], 32
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+208], 115
	mov rax, QWORD PTR [rbp-32]
	mov QWORD PTR [rax+216], 105
	mov rcx, QWORD PTR [rbp-32]
	mov QWORD PTR [rcx+224], 122
	mov rdx, QWORD PTR [rbp-32]
	mov QWORD PTR [rdx+232], 101
	mov rax, QWORD PTR [rbp-40]
	mov rcx, QWORD PTR [rbp-32]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-40], rax
	mov rax, QWORD PTR [rbp-40]
	mov rdx, rax
	mov QWORD PTR [rbp-48], rdx
	mov rcx, QWORD PTR [rbp-48]
	mov rdi, rcx
	call _Iprintln_pai
	leave 
	ret 
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 232
	and rsp, -16
	mov rax, rdi
	mov QWORD PTR [rbp-8], rax
	mov rcx, 11
	mov QWORD PTR [rbp-16], rcx
	mov rdx, 2
	mov QWORD PTR [rbp-24], rdx
	mov rcx, QWORD PTR [rbp-16]
	mov rax, rcx
	mov QWORD PTR [rbp-32], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rdi, rdx
	mov rax, QWORD PTR [rbp-32]
	mov rsi, rax
	call _IAck_iii
	mov rcx, rax
	mov QWORD PTR [rbp-40], rcx
	mov rax, QWORD PTR [rbp-40]
	mov rdx, rax
	mov QWORD PTR [rbp-48], rdx
	mov rcx, 56
	mov QWORD PTR [rbp-56], rcx
	mov rdx, QWORD PTR [rbp-56]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-40], rax
	mov rdx, QWORD PTR [rbp-40]
	mov rcx, rdx
	mov QWORD PTR [rbp-64], rcx
	mov rcx, QWORD PTR [rbp-64]
	mov rax, rcx
	mov QWORD PTR [rbp-72], rax
	mov rdx, QWORD PTR [rbp-72]
	mov QWORD PTR [rdx], 6
	mov rax, QWORD PTR [rbp-72]
	mov QWORD PTR [rax+8], 65
	mov rcx, QWORD PTR [rbp-72]
	mov QWORD PTR [rcx+16], 99
	mov rdx, QWORD PTR [rbp-72]
	mov QWORD PTR [rdx+24], 107
	mov rax, QWORD PTR [rbp-72]
	mov QWORD PTR [rax+32], 40
	mov rcx, QWORD PTR [rbp-72]
	mov QWORD PTR [rcx+40], 50
	mov rdx, QWORD PTR [rbp-72]
	mov QWORD PTR [rdx+48], 44
	mov rax, QWORD PTR [rbp-80]
	mov rcx, QWORD PTR [rbp-72]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-80], rax
	mov rax, QWORD PTR [rbp-80]
	mov rdx, rax
	mov QWORD PTR [rbp-88], rdx
	mov rcx, QWORD PTR [rbp-88]
	mov rdi, rcx
	call _Iprint_pai
	mov rax, QWORD PTR [rbp-16]
	mov rdx, rax
	mov QWORD PTR [rbp-96], rdx
	mov rcx, QWORD PTR [rbp-96]
	mov rdi, rcx
	call _IunparseInt_aii
	mov rdx, rax
	mov QWORD PTR [rbp-40], rdx
	mov rcx, QWORD PTR [rbp-40]
	mov rax, rcx
	mov QWORD PTR [rbp-104], rax
	mov rax, QWORD PTR [rbp-104]
	mov rdx, rax
	mov QWORD PTR [rbp-112], rdx
	mov rcx, QWORD PTR [rbp-112]
	mov rdi, rcx
	call _Iprint_pai
	mov rdx, 32
	mov QWORD PTR [rbp-120], rdx
	mov rax, QWORD PTR [rbp-120]
	mov rdi, rax
	call _eta_alloc
	mov rcx, rax
	mov QWORD PTR [rbp-40], rcx
	mov rax, QWORD PTR [rbp-40]
	mov rdx, rax
	mov QWORD PTR [rbp-128], rdx
	mov rdx, QWORD PTR [rbp-128]
	mov rcx, rdx
	mov QWORD PTR [rbp-136], rcx
	mov rax, QWORD PTR [rbp-136]
	mov QWORD PTR [rax], 3
	mov rcx, QWORD PTR [rbp-136]
	mov QWORD PTR [rcx+8], 41
	mov rdx, QWORD PTR [rbp-136]
	mov QWORD PTR [rdx+16], 58
	mov rax, QWORD PTR [rbp-136]
	mov QWORD PTR [rax+24], 32
	mov rcx, QWORD PTR [rbp-144]
	mov rdx, QWORD PTR [rbp-136]
	lea rcx, QWORD PTR [rdx+8]
	mov QWORD PTR [rbp-144], rcx
	mov rcx, QWORD PTR [rbp-144]
	mov rax, rcx
	mov QWORD PTR [rbp-152], rax
	mov rdx, QWORD PTR [rbp-152]
	mov rdi, rdx
	call _Iprint_pai
	mov rcx, QWORD PTR [rbp-48]
	mov rax, rcx
	mov QWORD PTR [rbp-160], rax
	mov rdx, QWORD PTR [rbp-160]
	mov rdi, rdx
	call _IunparseInt_aii
	mov rax, rax
	mov QWORD PTR [rbp-40], rax
	mov rdx, QWORD PTR [rbp-40]
	mov rcx, rdx
	mov QWORD PTR [rbp-168], rcx
	mov rcx, QWORD PTR [rbp-168]
	mov rax, rcx
	mov QWORD PTR [rbp-176], rax
	mov rdx, QWORD PTR [rbp-176]
	mov rdi, rdx
	call _Iprint_pai
	mov rax, 8
	mov QWORD PTR [rbp-184], rax
	mov rcx, QWORD PTR [rbp-184]
	mov rdi, rcx
	call _eta_alloc
	mov rdx, rax
	mov QWORD PTR [rbp-40], rdx
	mov rcx, QWORD PTR [rbp-40]
	mov rax, rcx
	mov QWORD PTR [rbp-192], rax
	mov rax, QWORD PTR [rbp-192]
	mov rdx, rax
	mov QWORD PTR [rbp-200], rdx
	mov rcx, QWORD PTR [rbp-200]
	mov QWORD PTR [rcx], 0
	mov rdx, QWORD PTR [rbp-208]
	mov rax, QWORD PTR [rbp-200]
	lea rdx, QWORD PTR [rax+8]
	mov QWORD PTR [rbp-208], rdx
	mov rdx, QWORD PTR [rbp-208]
	mov rcx, rdx
	mov QWORD PTR [rbp-216], rcx
	mov rax, QWORD PTR [rbp-216]
	mov rdi, rax
	call _Iprintln_pai
	leave 
	ret 
