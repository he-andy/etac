	.intel_syntax noprefix
	.text
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 168
	and rsp, -16
	mov rax, rdi
	mov QWORD PTR [rbp-8], rax
	mov rcx, 48
	mov QWORD PTR [rbp-16], rcx
	mov rdx, QWORD PTR [rbp-16]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-32], rcx
	mov rcx, QWORD PTR [rbp-32]
	mov rax, rcx
	mov QWORD PTR [rbp-40], rax
	mov rdx, QWORD PTR [rbp-40]
	mov QWORD PTR [rdx], 5
	mov rax, QWORD PTR [rbp-40]
	mov QWORD PTR [rax+8], 5
	mov rcx, QWORD PTR [rbp-40]
	mov QWORD PTR [rcx+16], 4
	mov rdx, QWORD PTR [rbp-40]
	mov QWORD PTR [rdx+24], 3
	mov rax, QWORD PTR [rbp-40]
	mov QWORD PTR [rax+32], 2
	mov rcx, QWORD PTR [rbp-40]
	mov QWORD PTR [rcx+40], 1
	mov rdx, QWORD PTR [rbp-48]
	mov rax, QWORD PTR [rbp-40]
	lea rdx, QWORD PTR [rax+8]
	mov QWORD PTR [rbp-48], rdx
	mov rdx, QWORD PTR [rbp-48]
	mov rcx, rdx
	mov QWORD PTR [rbp-56], rcx
	mov rcx, QWORD PTR [rbp-56]
	mov rax, rcx
	mov QWORD PTR [rbp-64], rax
	mov rdx, QWORD PTR [rbp-64]
	mov rdi, rdx
	call _Ibubble__sort_aiai
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-72], rcx
	mov rax, 0
	mov QWORD PTR [rbp-80], rax
_lh33:
	mov rcx, QWORD PTR [rbp-80]
	cmp rcx, 5
	mov rdx, QWORD PTR [rbp-88]
	setl dl
	movsx rdx, dl
	mov QWORD PTR [rbp-88], rdx
	mov rax, 1
	mov QWORD PTR [rbp-96], rax
	mov rcx, QWORD PTR [rbp-96]
	mov rdx, QWORD PTR [rbp-88]
	xor rcx, rdx
	mov QWORD PTR [rbp-96], rcx
	mov rax, QWORD PTR [rbp-96]
	test rax, rax
	jnz _lf32
	mov rdx, QWORD PTR [rbp-72]
	mov rcx, rdx
	mov QWORD PTR [rbp-104], rcx
	mov rcx, QWORD PTR [rbp-80]
	mov rax, rcx
	mov QWORD PTR [rbp-112], rax
	mov rax, QWORD PTR [rbp-104]
	mov rdx, rax
	mov QWORD PTR [rbp-120], rdx
	mov rcx, QWORD PTR [rbp-120]
	sub rcx, 8
	mov QWORD PTR [rbp-120], rcx
	mov rdx, QWORD PTR [rbp-112]
	mov rax, QWORD PTR [rbp-120]
	cmp rdx, QWORD PTR [rax]
	mov rcx, QWORD PTR [rbp-128]
	setb cl
	movsx rcx, cl
	mov QWORD PTR [rbp-128], rcx
	mov rdx, 1
	mov QWORD PTR [rbp-136], rdx
	mov rax, QWORD PTR [rbp-136]
	mov rcx, QWORD PTR [rbp-128]
	xor rax, rcx
	mov QWORD PTR [rbp-136], rax
	mov rdx, QWORD PTR [rbp-136]
	test rdx, rdx
	jnz _eta_out_of_bounds
	mov rcx, QWORD PTR [rbp-104]
	mov rdx, QWORD PTR [rbp-112]
	mov rax, QWORD PTR [rcx+rdx*8]
	mov QWORD PTR [rbp-144], rax
	mov rax, QWORD PTR [rbp-144]
	mov rdi, rax
	call _IunparseInt_aii
	mov rcx, rax
	mov QWORD PTR [rbp-24], rcx
	mov rax, QWORD PTR [rbp-24]
	mov rdx, rax
	mov QWORD PTR [rbp-152], rdx
	mov rdx, QWORD PTR [rbp-152]
	mov rcx, rdx
	mov QWORD PTR [rbp-160], rcx
	mov rax, QWORD PTR [rbp-160]
	mov rdi, rax
	call _Iprintln_pai
	mov rcx, QWORD PTR [rbp-80]
	add rcx, 1
	mov QWORD PTR [rbp-80], rcx
	jmp _lh33
_lf32:
	leave 
	ret 
	.globl _Ibubble__sort_aiai
_Ibubble__sort_aiai:
	push rbp
	mov rbp, rsp
	sub rsp, 432
	and rsp, -16
	mov rdx, rdi
	mov QWORD PTR [rbp-8], rdx
	mov rax, 0
	mov QWORD PTR [rbp-16], rax
	mov rcx, 0
	mov QWORD PTR [rbp-24], rcx
_lh3:
	mov rax, QWORD PTR [rbp-8]
	mov rdx, rax
	mov QWORD PTR [rbp-32], rdx
	mov rcx, QWORD PTR [rbp-32]
	sub rcx, 8
	mov QWORD PTR [rbp-32], rcx
	mov rdx, QWORD PTR [rbp-16]
	mov rax, QWORD PTR [rbp-32]
	cmp rdx, QWORD PTR [rax]
	mov rcx, QWORD PTR [rbp-40]
	setl cl
	movsx rcx, cl
	mov QWORD PTR [rbp-40], rcx
	mov rdx, 1
	mov QWORD PTR [rbp-48], rdx
	mov rax, QWORD PTR [rbp-48]
	mov rcx, QWORD PTR [rbp-40]
	xor rax, rcx
	mov QWORD PTR [rbp-48], rax
	mov rdx, QWORD PTR [rbp-48]
	test rdx, rdx
	jnz _lf2
_lh6:
	mov rcx, QWORD PTR [rbp-8]
	mov rax, rcx
	mov QWORD PTR [rbp-56], rax
	mov rdx, QWORD PTR [rbp-56]
	sub rdx, 8
	mov QWORD PTR [rbp-56], rdx
	mov rcx, QWORD PTR [rbp-56]
	mov rax, QWORD PTR [rcx]
	mov QWORD PTR [rbp-64], rax
	mov rdx, QWORD PTR [rbp-64]
	sub rdx, 1
	mov QWORD PTR [rbp-64], rdx
	mov rcx, QWORD PTR [rbp-64]
	mov rax, rcx
	mov QWORD PTR [rbp-72], rax
	mov rdx, QWORD PTR [rbp-72]
	mov rax, QWORD PTR [rbp-16]
	sub rdx, rax
	mov QWORD PTR [rbp-72], rdx
	mov rcx, QWORD PTR [rbp-24]
	mov rdx, QWORD PTR [rbp-72]
	cmp rcx, rdx
	mov rax, QWORD PTR [rbp-80]
	setl al
	movsx rax, al
	mov QWORD PTR [rbp-80], rax
	mov rcx, 1
	mov QWORD PTR [rbp-88], rcx
	mov rdx, QWORD PTR [rbp-88]
	mov rax, QWORD PTR [rbp-80]
	xor rdx, rax
	mov QWORD PTR [rbp-88], rdx
	mov rcx, QWORD PTR [rbp-88]
	test rcx, rcx
	jnz _lf5
	mov rax, QWORD PTR [rbp-8]
	mov rdx, rax
	mov QWORD PTR [rbp-96], rdx
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-104], rcx
	mov rcx, QWORD PTR [rbp-96]
	mov rax, rcx
	mov QWORD PTR [rbp-112], rax
	mov rdx, QWORD PTR [rbp-112]
	sub rdx, 8
	mov QWORD PTR [rbp-112], rdx
	mov rax, QWORD PTR [rbp-104]
	mov rcx, QWORD PTR [rbp-112]
	cmp rax, QWORD PTR [rcx]
	mov rdx, QWORD PTR [rbp-120]
	setb dl
	movsx rdx, dl
	mov QWORD PTR [rbp-120], rdx
	mov rax, 1
	mov QWORD PTR [rbp-128], rax
	mov rcx, QWORD PTR [rbp-128]
	mov rdx, QWORD PTR [rbp-120]
	xor rcx, rdx
	mov QWORD PTR [rbp-128], rcx
	mov rax, QWORD PTR [rbp-128]
	test rax, rax
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-8]
	mov rcx, rdx
	mov QWORD PTR [rbp-136], rcx
	mov rax, QWORD PTR [rbp-144]
	mov rcx, QWORD PTR [rbp-24]
	lea rax, QWORD PTR [rcx+1]
	mov QWORD PTR [rbp-144], rax
	mov rax, QWORD PTR [rbp-144]
	mov rdx, rax
	mov QWORD PTR [rbp-152], rdx
	mov rdx, QWORD PTR [rbp-136]
	mov rcx, rdx
	mov QWORD PTR [rbp-160], rcx
	mov rax, QWORD PTR [rbp-160]
	sub rax, 8
	mov QWORD PTR [rbp-160], rax
	mov rcx, QWORD PTR [rbp-152]
	mov rdx, QWORD PTR [rbp-160]
	cmp rcx, QWORD PTR [rdx]
	mov rax, QWORD PTR [rbp-168]
	setb al
	movsx rax, al
	mov QWORD PTR [rbp-168], rax
	mov rcx, 1
	mov QWORD PTR [rbp-176], rcx
	mov rdx, QWORD PTR [rbp-176]
	mov rax, QWORD PTR [rbp-168]
	xor rdx, rax
	mov QWORD PTR [rbp-176], rdx
	mov rcx, QWORD PTR [rbp-176]
	test rcx, rcx
	jnz _eta_out_of_bounds
	mov rax, QWORD PTR [rbp-136]
	mov rcx, QWORD PTR [rbp-152]
	mov rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-184], rdx
	mov rdx, QWORD PTR [rbp-96]
	mov rax, QWORD PTR [rbp-104]
	mov rcx, QWORD PTR [rbp-184]
	cmp QWORD PTR [rdx+rax*8], rcx
	mov rdx, QWORD PTR [rbp-192]
	setg dl
	movsx rdx, dl
	mov QWORD PTR [rbp-192], rdx
	mov rax, 1
	mov QWORD PTR [rbp-200], rax
	mov rcx, QWORD PTR [rbp-200]
	mov rdx, QWORD PTR [rbp-192]
	xor rcx, rdx
	mov QWORD PTR [rbp-200], rcx
	mov rax, QWORD PTR [rbp-200]
	test rax, rax
	jnz _lf8
	mov rdx, QWORD PTR [rbp-8]
	mov rcx, rdx
	mov QWORD PTR [rbp-208], rcx
	mov rcx, QWORD PTR [rbp-24]
	mov rax, rcx
	mov QWORD PTR [rbp-216], rax
	mov rax, QWORD PTR [rbp-208]
	mov rdx, rax
	mov QWORD PTR [rbp-224], rdx
	mov rcx, QWORD PTR [rbp-224]
	sub rcx, 8
	mov QWORD PTR [rbp-224], rcx
	mov rdx, QWORD PTR [rbp-216]
	mov rax, QWORD PTR [rbp-224]
	cmp rdx, QWORD PTR [rax]
	mov rcx, QWORD PTR [rbp-232]
	setb cl
	movsx rcx, cl
	mov QWORD PTR [rbp-232], rcx
	mov rdx, 1
	mov QWORD PTR [rbp-240], rdx
	mov rax, QWORD PTR [rbp-240]
	mov rcx, QWORD PTR [rbp-232]
	xor rax, rcx
	mov QWORD PTR [rbp-240], rax
	mov rdx, QWORD PTR [rbp-240]
	test rdx, rdx
	jnz _eta_out_of_bounds
	mov rcx, QWORD PTR [rbp-208]
	mov rdx, QWORD PTR [rbp-216]
	mov rax, QWORD PTR [rcx+rdx*8]
	mov QWORD PTR [rbp-248], rax
	mov rcx, QWORD PTR [rbp-8]
	mov rax, rcx
	mov QWORD PTR [rbp-256], rax
	mov rax, QWORD PTR [rbp-24]
	mov rdx, rax
	mov QWORD PTR [rbp-264], rdx
	mov rdx, QWORD PTR [rbp-256]
	mov rcx, rdx
	mov QWORD PTR [rbp-272], rcx
	mov rax, QWORD PTR [rbp-272]
	sub rax, 8
	mov QWORD PTR [rbp-272], rax
	mov rcx, QWORD PTR [rbp-264]
	mov rdx, QWORD PTR [rbp-272]
	cmp rcx, QWORD PTR [rdx]
	mov rax, QWORD PTR [rbp-280]
	setb al
	movsx rax, al
	mov QWORD PTR [rbp-280], rax
	mov rcx, 1
	mov QWORD PTR [rbp-288], rcx
	mov rdx, QWORD PTR [rbp-288]
	mov rax, QWORD PTR [rbp-280]
	xor rdx, rax
	mov QWORD PTR [rbp-288], rdx
	mov rcx, QWORD PTR [rbp-288]
	test rcx, rcx
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-296]
	mov rax, QWORD PTR [rbp-256]
	mov rcx, QWORD PTR [rbp-264]
	lea rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-296], rdx
	mov rax, QWORD PTR [rbp-296]
	mov rdx, rax
	mov QWORD PTR [rbp-304], rdx
	mov rdx, QWORD PTR [rbp-8]
	mov rcx, rdx
	mov QWORD PTR [rbp-312], rcx
	mov rax, QWORD PTR [rbp-320]
	mov rcx, QWORD PTR [rbp-24]
	lea rax, QWORD PTR [rcx+1]
	mov QWORD PTR [rbp-320], rax
	mov rax, QWORD PTR [rbp-320]
	mov rdx, rax
	mov QWORD PTR [rbp-328], rdx
	mov rdx, QWORD PTR [rbp-312]
	mov rcx, rdx
	mov QWORD PTR [rbp-336], rcx
	mov rax, QWORD PTR [rbp-336]
	sub rax, 8
	mov QWORD PTR [rbp-336], rax
	mov rcx, QWORD PTR [rbp-328]
	mov rdx, QWORD PTR [rbp-336]
	cmp rcx, QWORD PTR [rdx]
	mov rax, QWORD PTR [rbp-344]
	setb al
	movsx rax, al
	mov QWORD PTR [rbp-344], rax
	mov rcx, 1
	mov QWORD PTR [rbp-352], rcx
	mov rdx, QWORD PTR [rbp-352]
	mov rax, QWORD PTR [rbp-344]
	xor rdx, rax
	mov QWORD PTR [rbp-352], rdx
	mov rcx, QWORD PTR [rbp-352]
	test rcx, rcx
	jnz _eta_out_of_bounds
	mov rax, QWORD PTR [rbp-312]
	mov rcx, QWORD PTR [rbp-328]
	mov rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-360], rdx
	mov rdx, QWORD PTR [rbp-304]
	mov rax, QWORD PTR [rbp-360]
	mov QWORD PTR [rdx], rax
	mov rdx, QWORD PTR [rbp-8]
	mov rcx, rdx
	mov QWORD PTR [rbp-368], rcx
	mov rax, QWORD PTR [rbp-376]
	mov rcx, QWORD PTR [rbp-24]
	lea rax, QWORD PTR [rcx+1]
	mov QWORD PTR [rbp-376], rax
	mov rax, QWORD PTR [rbp-376]
	mov rdx, rax
	mov QWORD PTR [rbp-384], rdx
	mov rdx, QWORD PTR [rbp-368]
	mov rcx, rdx
	mov QWORD PTR [rbp-392], rcx
	mov rax, QWORD PTR [rbp-392]
	sub rax, 8
	mov QWORD PTR [rbp-392], rax
	mov rcx, QWORD PTR [rbp-384]
	mov rdx, QWORD PTR [rbp-392]
	cmp rcx, QWORD PTR [rdx]
	mov rax, QWORD PTR [rbp-400]
	setb al
	movsx rax, al
	mov QWORD PTR [rbp-400], rax
	mov rcx, 1
	mov QWORD PTR [rbp-408], rcx
	mov rdx, QWORD PTR [rbp-408]
	mov rax, QWORD PTR [rbp-400]
	xor rdx, rax
	mov QWORD PTR [rbp-408], rdx
	mov rcx, QWORD PTR [rbp-408]
	test rcx, rcx
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-416]
	mov rax, QWORD PTR [rbp-368]
	mov rcx, QWORD PTR [rbp-384]
	lea rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-416], rdx
	mov rax, QWORD PTR [rbp-416]
	mov rdx, rax
	mov QWORD PTR [rbp-424], rdx
	mov rcx, QWORD PTR [rbp-424]
	mov rdx, QWORD PTR [rbp-248]
	mov QWORD PTR [rcx], rdx
_lf8:
	mov rax, QWORD PTR [rbp-24]
	add rax, 1
	mov QWORD PTR [rbp-24], rax
	jmp _lh6
_lf5:
	mov rcx, QWORD PTR [rbp-16]
	add rcx, 1
	mov QWORD PTR [rbp-16], rcx
	mov rdx, 0
	mov QWORD PTR [rbp-24], rdx
	jmp _lh3
_lf2:
	mov rcx, QWORD PTR [rbp-8]
	mov rax, rcx
	mov QWORD PTR [rbp-432], rax
	mov rdx, QWORD PTR [rbp-432]
	mov rax, rdx
	leave 
	ret 
