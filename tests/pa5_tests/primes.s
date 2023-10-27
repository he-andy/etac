	.intel_syntax noprefix
	.text
	.globl _Iisprime_bi
_Iisprime_bi:
	push rbp
	mov rbp, rsp
	sub rsp, 104
	and rsp, -16
	mov rdx, QWORD PTR [rbp-8]
	mov rdx, rdi
	mov QWORD PTR [rbp-8], rdx
	mov rax, QWORD PTR [rbp-16]
	mov rax, 2
	mov QWORD PTR [rbp-16], rax
_lh9:
	mov rcx, QWORD PTR [rbp-24]
	mov rdx, QWORD PTR [rbp-16]
	mov rcx, rdx
	mov QWORD PTR [rbp-24], rcx
	mov rax, QWORD PTR [rbp-24]
	mov rcx, QWORD PTR [rbp-16]
	imul rax, rcx
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rax, QWORD PTR [rbp-8]
	cmp rdx, rax
	mov rcx, QWORD PTR [rbp-32]
	xor rcx, rcx
	setle cl
	mov rdx, QWORD PTR [rbp-40]
	mov rdx, 1
	mov QWORD PTR [rbp-40], rdx
	mov rax, QWORD PTR [rbp-40]
	mov rcx, QWORD PTR [rbp-32]
	xor rax, rcx
	mov QWORD PTR [rbp-40], rax
	mov rdx, QWORD PTR [rbp-40]
	test rdx, rdx
	jnz _lf8
	mov rax, QWORD PTR [rbp-48]
	mov rcx, QWORD PTR [rbp-16]
	mov rax, rcx
	mov QWORD PTR [rbp-48], rax
	mov rdx, QWORD PTR [rbp-56]
	mov rax, QWORD PTR [rbp-8]
	mov rdx, rax
	mov QWORD PTR [rbp-56], rdx
	mov rcx, QWORD PTR [rbp-48]
	mov rdi, rcx
	mov rdx, QWORD PTR [rbp-56]
	mov rsi, rdx
	call _Igcd_iii
	mov rax, QWORD PTR [rbp-64]
	mov rcx, QWORD PTR [rbp-72]
	mov rax, rcx
	mov QWORD PTR [rbp-64], rax
	mov rdx, QWORD PTR [rbp-64]
	cmp rdx, 1
	mov rax, QWORD PTR [rbp-80]
	xor rax, rax
	setne al
	mov rcx, QWORD PTR [rbp-88]
	mov rcx, 1
	mov QWORD PTR [rbp-88], rcx
	mov rdx, QWORD PTR [rbp-88]
	mov rax, QWORD PTR [rbp-80]
	xor rdx, rax
	mov QWORD PTR [rbp-88], rdx
	mov rcx, QWORD PTR [rbp-88]
	test rcx, rcx
	jnz _lf11
	mov rdx, QWORD PTR [rbp-72]
	mov rdx, 0
	mov QWORD PTR [rbp-72], rdx
	mov rax, QWORD PTR [rbp-72]
	mov rax, rax
	jmp __epilogue__isprime__
_lf11:
	mov rcx, QWORD PTR [rbp-16]
	add rcx, 1
	mov QWORD PTR [rbp-16], rcx
	jmp _lh9
_lf8:
	mov rdx, QWORD PTR [rbp-72]
	mov rdx, 1
	mov QWORD PTR [rbp-72], rdx
	mov rax, QWORD PTR [rbp-72]
	mov rax, rax
	jmp __epilogue__isprime__
__epilogue__isprime__:
	mov rsp, rbp
	pop rbp
	ret 
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 560
	and rsp, -16
	mov rdx, QWORD PTR [rbp-8]
	mov rdx, rdi
	mov QWORD PTR [rbp-8], rdx
	mov rax, QWORD PTR [rbp-16]
	mov rax, 272
	mov QWORD PTR [rbp-16], rax
	mov rcx, QWORD PTR [rbp-16]
	mov rdi, rcx
	call _eta_alloc
	mov rdx, QWORD PTR [rbp-24]
	mov rax, QWORD PTR [rbp-32]
	mov rdx, rax
	mov QWORD PTR [rbp-24], rdx
	mov rcx, QWORD PTR [rbp-40]
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-40], rcx
	mov rax, QWORD PTR [rbp-40]
	mov QWORD PTR [rax], 33
	mov rcx, QWORD PTR [rbp-48]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+8]
	mov QWORD PTR [rbp-48], rcx
	mov rax, QWORD PTR [rbp-48]
	mov QWORD PTR [rax], 76
	mov rcx, QWORD PTR [rbp-56]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+16]
	mov QWORD PTR [rbp-56], rcx
	mov rax, QWORD PTR [rbp-56]
	mov QWORD PTR [rax], 97
	mov rcx, QWORD PTR [rbp-64]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+24]
	mov QWORD PTR [rbp-64], rcx
	mov rax, QWORD PTR [rbp-64]
	mov QWORD PTR [rax], 114
	mov rcx, QWORD PTR [rbp-72]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+32]
	mov QWORD PTR [rbp-72], rcx
	mov rax, QWORD PTR [rbp-72]
	mov QWORD PTR [rax], 103
	mov rcx, QWORD PTR [rbp-80]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+40]
	mov QWORD PTR [rbp-80], rcx
	mov rax, QWORD PTR [rbp-80]
	mov QWORD PTR [rax], 101
	mov rcx, QWORD PTR [rbp-88]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+48]
	mov QWORD PTR [rbp-88], rcx
	mov rax, QWORD PTR [rbp-88]
	mov QWORD PTR [rax], 115
	mov rcx, QWORD PTR [rbp-96]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+56]
	mov QWORD PTR [rbp-96], rcx
	mov rax, QWORD PTR [rbp-96]
	mov QWORD PTR [rax], 116
	mov rcx, QWORD PTR [rbp-104]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+64]
	mov QWORD PTR [rbp-104], rcx
	mov rax, QWORD PTR [rbp-104]
	mov QWORD PTR [rax], 32
	mov rcx, QWORD PTR [rbp-112]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+72]
	mov QWORD PTR [rbp-112], rcx
	mov rax, QWORD PTR [rbp-112]
	mov QWORD PTR [rax], 112
	mov rcx, QWORD PTR [rbp-120]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+80]
	mov QWORD PTR [rbp-120], rcx
	mov rax, QWORD PTR [rbp-120]
	mov QWORD PTR [rax], 114
	mov rcx, QWORD PTR [rbp-128]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+88]
	mov QWORD PTR [rbp-128], rcx
	mov rax, QWORD PTR [rbp-128]
	mov QWORD PTR [rax], 105
	mov rcx, QWORD PTR [rbp-136]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+96]
	mov QWORD PTR [rbp-136], rcx
	mov rax, QWORD PTR [rbp-136]
	mov QWORD PTR [rax], 109
	mov rcx, QWORD PTR [rbp-144]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+104]
	mov QWORD PTR [rbp-144], rcx
	mov rax, QWORD PTR [rbp-144]
	mov QWORD PTR [rax], 101
	mov rcx, QWORD PTR [rbp-152]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+112]
	mov QWORD PTR [rbp-152], rcx
	mov rax, QWORD PTR [rbp-152]
	mov QWORD PTR [rax], 32
	mov rcx, QWORD PTR [rbp-160]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+120]
	mov QWORD PTR [rbp-160], rcx
	mov rax, QWORD PTR [rbp-160]
	mov QWORD PTR [rax], 108
	mov rcx, QWORD PTR [rbp-168]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+128]
	mov QWORD PTR [rbp-168], rcx
	mov rax, QWORD PTR [rbp-168]
	mov QWORD PTR [rax], 101
	mov rcx, QWORD PTR [rbp-176]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+136]
	mov QWORD PTR [rbp-176], rcx
	mov rax, QWORD PTR [rbp-176]
	mov QWORD PTR [rax], 115
	mov rcx, QWORD PTR [rbp-184]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+144]
	mov QWORD PTR [rbp-184], rcx
	mov rax, QWORD PTR [rbp-184]
	mov QWORD PTR [rax], 115
	mov rcx, QWORD PTR [rbp-192]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+152]
	mov QWORD PTR [rbp-192], rcx
	mov rax, QWORD PTR [rbp-192]
	mov QWORD PTR [rax], 32
	mov rcx, QWORD PTR [rbp-200]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+160]
	mov QWORD PTR [rbp-200], rcx
	mov rax, QWORD PTR [rbp-200]
	mov QWORD PTR [rax], 116
	mov rcx, QWORD PTR [rbp-208]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+168]
	mov QWORD PTR [rbp-208], rcx
	mov rax, QWORD PTR [rbp-208]
	mov QWORD PTR [rax], 104
	mov rcx, QWORD PTR [rbp-216]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+176]
	mov QWORD PTR [rbp-216], rcx
	mov rax, QWORD PTR [rbp-216]
	mov QWORD PTR [rax], 97
	mov rcx, QWORD PTR [rbp-224]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+184]
	mov QWORD PTR [rbp-224], rcx
	mov rax, QWORD PTR [rbp-224]
	mov QWORD PTR [rax], 110
	mov rcx, QWORD PTR [rbp-232]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+192]
	mov QWORD PTR [rbp-232], rcx
	mov rax, QWORD PTR [rbp-232]
	mov QWORD PTR [rax], 32
	mov rcx, QWORD PTR [rbp-240]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+200]
	mov QWORD PTR [rbp-240], rcx
	mov rax, QWORD PTR [rbp-240]
	mov QWORD PTR [rax], 49
	mov rcx, QWORD PTR [rbp-248]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+208]
	mov QWORD PTR [rbp-248], rcx
	mov rax, QWORD PTR [rbp-248]
	mov QWORD PTR [rax], 44
	mov rcx, QWORD PTR [rbp-256]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+216]
	mov QWORD PTR [rbp-256], rcx
	mov rax, QWORD PTR [rbp-256]
	mov QWORD PTR [rax], 48
	mov rcx, QWORD PTR [rbp-264]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+224]
	mov QWORD PTR [rbp-264], rcx
	mov rax, QWORD PTR [rbp-264]
	mov QWORD PTR [rax], 48
	mov rcx, QWORD PTR [rbp-272]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+232]
	mov QWORD PTR [rbp-272], rcx
	mov rax, QWORD PTR [rbp-272]
	mov QWORD PTR [rax], 48
	mov rcx, QWORD PTR [rbp-280]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+240]
	mov QWORD PTR [rbp-280], rcx
	mov rax, QWORD PTR [rbp-280]
	mov QWORD PTR [rax], 32
	mov rcx, QWORD PTR [rbp-288]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+248]
	mov QWORD PTR [rbp-288], rcx
	mov rax, QWORD PTR [rbp-288]
	mov QWORD PTR [rax], 105
	mov rcx, QWORD PTR [rbp-296]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+256]
	mov QWORD PTR [rbp-296], rcx
	mov rax, QWORD PTR [rbp-296]
	mov QWORD PTR [rax], 115
	mov rcx, QWORD PTR [rbp-304]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+264]
	mov QWORD PTR [rbp-304], rcx
	mov rax, QWORD PTR [rbp-304]
	mov QWORD PTR [rax], 32
	mov rcx, QWORD PTR [rbp-312]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+8]
	mov QWORD PTR [rbp-312], rcx
	mov rax, QWORD PTR [rbp-320]
	mov rcx, QWORD PTR [rbp-312]
	mov rax, rcx
	mov QWORD PTR [rbp-320], rax
	mov rdx, QWORD PTR [rbp-328]
	mov rdx, 1000
	mov QWORD PTR [rbp-328], rdx
	mov rax, QWORD PTR [rbp-328]
	mov rdi, rax
	call _Ilargestprime_ii
	mov rcx, QWORD PTR [rbp-336]
	mov rdx, QWORD PTR [rbp-32]
	mov rcx, rdx
	mov QWORD PTR [rbp-336], rcx
	mov rax, QWORD PTR [rbp-344]
	mov rcx, QWORD PTR [rbp-336]
	mov rax, rcx
	mov QWORD PTR [rbp-344], rax
	mov rdx, QWORD PTR [rbp-344]
	mov rdi, rdx
	call _IunparseInt_aii
	mov rax, QWORD PTR [rbp-352]
	mov rcx, QWORD PTR [rbp-32]
	mov rax, rcx
	mov QWORD PTR [rbp-352], rax
	mov rdx, QWORD PTR [rbp-360]
	mov rax, QWORD PTR [rbp-352]
	mov rdx, rax
	mov QWORD PTR [rbp-360], rdx
	mov rcx, QWORD PTR [rbp-368]
	mov rdx, QWORD PTR [rbp-320]
	mov rcx, rdx
	mov QWORD PTR [rbp-368], rcx
	mov rax, QWORD PTR [rbp-368]
	sub rax, 8
	mov QWORD PTR [rbp-368], rax
	mov rcx, QWORD PTR [rbp-376]
	mov rdx, QWORD PTR [rbp-368]
	mov rcx, QWORD PTR [rdx]
	mov QWORD PTR [rbp-376], rcx
	mov rax, QWORD PTR [rbp-384]
	mov rcx, QWORD PTR [rbp-360]
	mov rax, rcx
	mov QWORD PTR [rbp-384], rax
	mov rdx, QWORD PTR [rbp-384]
	sub rdx, 8
	mov QWORD PTR [rbp-384], rdx
	mov rax, QWORD PTR [rbp-392]
	mov rcx, QWORD PTR [rbp-384]
	mov rax, QWORD PTR [rcx]
	mov QWORD PTR [rbp-392], rax
	mov rdx, QWORD PTR [rbp-400]
	mov rax, QWORD PTR [rbp-376]
	mov rcx, QWORD PTR [rbp-392]
	lea rdx, QWORD PTR [rax+rcx]
	mov QWORD PTR [rbp-400], rdx
	mov rdx, QWORD PTR [rbp-408]
	mov rax, QWORD PTR [rbp-400]
	mov rdx, rax
	mov QWORD PTR [rbp-408], rdx
	mov rcx, QWORD PTR [rbp-416]
	mov rdx, QWORD PTR [rbp-408]
	lea rcx, QWORD PTR [rdx*8+8]
	mov QWORD PTR [rbp-416], rcx
	mov rax, QWORD PTR [rbp-424]
	mov rcx, QWORD PTR [rbp-416]
	mov rax, rcx
	mov QWORD PTR [rbp-424], rax
	mov rdx, QWORD PTR [rbp-424]
	mov rdi, rdx
	call _eta_alloc
	mov rax, QWORD PTR [rbp-432]
	mov rcx, QWORD PTR [rbp-32]
	mov rax, rcx
	mov QWORD PTR [rbp-432], rax
	mov rdx, QWORD PTR [rbp-440]
	mov rax, QWORD PTR [rbp-432]
	mov rdx, rax
	mov QWORD PTR [rbp-440], rdx
	mov rcx, QWORD PTR [rbp-440]
	mov rdx, QWORD PTR [rbp-408]
	mov QWORD PTR [rcx], rdx
	mov rax, QWORD PTR [rbp-448]
	mov rax, 1
	mov QWORD PTR [rbp-448], rax
_lh27:
	mov rcx, QWORD PTR [rbp-456]
	mov rdx, QWORD PTR [rbp-376]
	lea rcx, QWORD PTR [rdx+1]
	mov QWORD PTR [rbp-456], rcx
	mov rax, QWORD PTR [rbp-448]
	mov rcx, QWORD PTR [rbp-456]
	cmp rax, rcx
	jne _lt28
	mov rdx, QWORD PTR [rbp-448]
	mov rdx, 1
	mov QWORD PTR [rbp-448], rdx
_lhh30:
	mov rax, QWORD PTR [rbp-464]
	mov rcx, QWORD PTR [rbp-392]
	lea rax, QWORD PTR [rcx+1]
	mov QWORD PTR [rbp-464], rax
	mov rdx, QWORD PTR [rbp-448]
	mov rax, QWORD PTR [rbp-464]
	cmp rdx, rax
	jne _ltt31
	mov rcx, QWORD PTR [rbp-472]
	mov rdx, QWORD PTR [rbp-440]
	lea rcx, QWORD PTR [rdx+8]
	mov QWORD PTR [rbp-472], rcx
	mov rax, QWORD PTR [rbp-480]
	mov rcx, QWORD PTR [rbp-472]
	mov rax, rcx
	mov QWORD PTR [rbp-480], rax
	mov rdx, QWORD PTR [rbp-480]
	mov rdi, rdx
	call _Iprint_pai
	jmp __epilogue__main__
_lt28:
	mov rax, QWORD PTR [rbp-488]
	mov rcx, QWORD PTR [rbp-440]
	mov rdx, QWORD PTR [rbp-448]
	lea rax, QWORD PTR [rcx+rdx*8]
	mov QWORD PTR [rbp-488], rax
	mov rax, QWORD PTR [rbp-496]
	mov rcx, QWORD PTR [rbp-320]
	mov rax, rcx
	mov QWORD PTR [rbp-496], rax
	mov rdx, QWORD PTR [rbp-496]
	sub rdx, 8
	mov QWORD PTR [rbp-496], rdx
	mov rax, QWORD PTR [rbp-504]
	mov rcx, QWORD PTR [rbp-496]
	mov rdx, QWORD PTR [rbp-448]
	lea rax, QWORD PTR [rcx+rdx*8]
	mov QWORD PTR [rbp-504], rax
	mov rax, QWORD PTR [rbp-512]
	mov rcx, QWORD PTR [rbp-504]
	lea rax, QWORD PTR [rcx]
	mov QWORD PTR [rbp-512], rax
	mov rdx, QWORD PTR [rbp-488]
	mov rax, QWORD PTR [rbp-512]
	mov QWORD PTR [rdx], rax
	mov rcx, QWORD PTR [rbp-448]
	add rcx, 1
	mov QWORD PTR [rbp-448], rcx
	jmp _lh27
_ltt31:
	mov rdx, QWORD PTR [rbp-520]
	mov rax, QWORD PTR [rbp-376]
	mov rcx, QWORD PTR [rbp-448]
	lea rdx, QWORD PTR [rax+rcx]
	mov QWORD PTR [rbp-520], rdx
	mov rdx, QWORD PTR [rbp-528]
	mov rax, QWORD PTR [rbp-440]
	mov rcx, QWORD PTR [rbp-520]
	lea rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-528], rdx
	mov rdx, QWORD PTR [rbp-536]
	mov rax, QWORD PTR [rbp-360]
	mov rdx, rax
	mov QWORD PTR [rbp-536], rdx
	mov rcx, QWORD PTR [rbp-536]
	sub rcx, 8
	mov QWORD PTR [rbp-536], rcx
	mov rdx, QWORD PTR [rbp-544]
	mov rax, QWORD PTR [rbp-536]
	mov rcx, QWORD PTR [rbp-448]
	lea rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-544], rdx
	mov rdx, QWORD PTR [rbp-552]
	mov rax, QWORD PTR [rbp-544]
	lea rdx, QWORD PTR [rax]
	mov QWORD PTR [rbp-552], rdx
	mov rcx, QWORD PTR [rbp-528]
	mov rdx, QWORD PTR [rbp-552]
	mov QWORD PTR [rcx], rdx
	mov rax, QWORD PTR [rbp-448]
	add rax, 1
	mov QWORD PTR [rbp-448], rax
	jmp _lhh30
__epilogue__main__:
	mov rsp, rbp
	pop rbp
	ret 
	.globl _Igcd_iii
_Igcd_iii:
	push rbp
	mov rbp, rsp
	sub rsp, 40
	and rsp, -16
	mov rax, QWORD PTR [rbp-8]
	mov rax, rdi
	mov QWORD PTR [rbp-8], rax
	mov rcx, QWORD PTR [rbp-16]
	mov rcx, rsi
	mov QWORD PTR [rbp-16], rcx
_lh3:
	mov rdx, QWORD PTR [rbp-8]
	cmp rdx, 0
	mov rax, QWORD PTR [rbp-24]
	xor rax, rax
	setne al
	mov rcx, QWORD PTR [rbp-32]
	mov rcx, 1
	mov QWORD PTR [rbp-32], rcx
	mov rdx, QWORD PTR [rbp-32]
	mov rax, QWORD PTR [rbp-24]
	xor rdx, rax
	mov QWORD PTR [rbp-32], rdx
	mov rcx, QWORD PTR [rbp-32]
	test rcx, rcx
	jnz _lf2
	mov rdx, QWORD PTR [rbp-8]
	mov rax, QWORD PTR [rbp-16]
	cmp rdx, rax
	jl _lt4
	mov rcx, QWORD PTR [rbp-8]
	mov rdx, QWORD PTR [rbp-16]
	sub rcx, rdx
	mov QWORD PTR [rbp-8], rcx
_endif6:
	jmp _lh3
_lf2:
	mov rax, QWORD PTR [rbp-40]
	mov rcx, QWORD PTR [rbp-16]
	mov rax, rcx
	mov QWORD PTR [rbp-40], rax
	mov rdx, QWORD PTR [rbp-40]
	mov rax, rdx
	jmp __epilogue__gcd__
_lt4:
	mov rax, QWORD PTR [rbp-16]
	mov rcx, QWORD PTR [rbp-8]
	sub rax, rcx
	mov QWORD PTR [rbp-16], rax
	jmp _endif6
__epilogue__gcd__:
	mov rsp, rbp
	pop rbp
	ret 
	.globl _Ilargestprime_ii
_Ilargestprime_ii:
	push rbp
	mov rbp, rsp
	sub rsp, 80
	and rsp, -16
	mov rax, QWORD PTR [rbp-8]
	mov rax, rdi
	mov QWORD PTR [rbp-8], rax
	mov rcx, QWORD PTR [rbp-16]
	mov rcx, 1
	mov QWORD PTR [rbp-16], rcx
	mov rdx, QWORD PTR [rbp-24]
	mov rdx, 1
	mov QWORD PTR [rbp-24], rdx
_lh15:
	mov rax, QWORD PTR [rbp-16]
	mov rcx, QWORD PTR [rbp-8]
	cmp rax, rcx
	mov rdx, QWORD PTR [rbp-32]
	xor rdx, rdx
	setl dl
	mov rax, QWORD PTR [rbp-40]
	mov rax, 1
	mov QWORD PTR [rbp-40], rax
	mov rcx, QWORD PTR [rbp-40]
	mov rdx, QWORD PTR [rbp-32]
	xor rcx, rdx
	mov QWORD PTR [rbp-40], rcx
	mov rax, QWORD PTR [rbp-40]
	test rax, rax
	jnz _lf14
	mov rcx, QWORD PTR [rbp-48]
	mov rdx, QWORD PTR [rbp-16]
	mov rcx, rdx
	mov QWORD PTR [rbp-48], rcx
	mov rax, QWORD PTR [rbp-48]
	mov rdi, rax
	call _Iisprime_bi
	mov rcx, QWORD PTR [rbp-56]
	mov rdx, QWORD PTR [rbp-64]
	mov rcx, rdx
	mov QWORD PTR [rbp-56], rcx
	mov rax, QWORD PTR [rbp-72]
	mov rax, 1
	mov QWORD PTR [rbp-72], rax
	mov rcx, QWORD PTR [rbp-72]
	mov rdx, QWORD PTR [rbp-56]
	xor rcx, rdx
	mov QWORD PTR [rbp-72], rcx
	mov rax, QWORD PTR [rbp-72]
	test rax, rax
	jnz _lf17
	mov rcx, QWORD PTR [rbp-24]
	mov rdx, QWORD PTR [rbp-16]
	mov rcx, rdx
	mov QWORD PTR [rbp-24], rcx
_lf17:
	mov rax, QWORD PTR [rbp-16]
	add rax, 1
	mov QWORD PTR [rbp-16], rax
	jmp _lh15
_lf14:
	mov rcx, QWORD PTR [rbp-64]
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-64], rcx
	mov rax, QWORD PTR [rbp-64]
	mov rax, rax
	jmp __epilogue__largestprime__
__epilogue__largestprime__:
	mov rsp, rbp
	pop rbp
	ret 
