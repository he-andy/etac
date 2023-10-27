	.intel_syntax noprefix
	.text
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 872
	and rsp, -16
	mov rcx, rdi
	mov QWORD PTR [rbp-8], rcx
	mov rdx, 32
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
	mov QWORD PTR [rax], 3
	mov rcx, 32
	mov QWORD PTR [rbp-48], rcx
	mov rdx, QWORD PTR [rbp-48]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-56], rcx
	mov rcx, QWORD PTR [rbp-56]
	mov rax, rcx
	mov QWORD PTR [rbp-64], rax
	mov rdx, QWORD PTR [rbp-64]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-64]
	mov QWORD PTR [rax+8], 1
	mov rcx, QWORD PTR [rbp-64]
	mov QWORD PTR [rcx+16], 2
	mov rdx, QWORD PTR [rbp-64]
	mov QWORD PTR [rdx+24], 3
	mov rax, QWORD PTR [rbp-72]
	mov rcx, QWORD PTR [rbp-64]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-72], rax
	mov rdx, QWORD PTR [rbp-40]
	mov rax, QWORD PTR [rbp-72]
	mov QWORD PTR [rdx+8], rax
	mov rcx, 32
	mov QWORD PTR [rbp-80], rcx
	mov rdx, QWORD PTR [rbp-80]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-88], rcx
	mov rcx, QWORD PTR [rbp-88]
	mov rax, rcx
	mov QWORD PTR [rbp-96], rax
	mov rdx, QWORD PTR [rbp-96]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-96]
	mov QWORD PTR [rax+8], 4
	mov rcx, QWORD PTR [rbp-96]
	mov QWORD PTR [rcx+16], 5
	mov rdx, QWORD PTR [rbp-96]
	mov QWORD PTR [rdx+24], 6
	mov rax, QWORD PTR [rbp-104]
	mov rcx, QWORD PTR [rbp-96]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-104], rax
	mov rdx, QWORD PTR [rbp-40]
	mov rax, QWORD PTR [rbp-104]
	mov QWORD PTR [rdx+16], rax
	mov rcx, 32
	mov QWORD PTR [rbp-112], rcx
	mov rdx, QWORD PTR [rbp-112]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-120], rcx
	mov rcx, QWORD PTR [rbp-120]
	mov rax, rcx
	mov QWORD PTR [rbp-128], rax
	mov rdx, QWORD PTR [rbp-128]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-128]
	mov QWORD PTR [rax+8], 7
	mov rcx, QWORD PTR [rbp-128]
	mov QWORD PTR [rcx+16], 8
	mov rdx, QWORD PTR [rbp-128]
	mov QWORD PTR [rdx+24], 9
	mov rax, QWORD PTR [rbp-136]
	mov rcx, QWORD PTR [rbp-128]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-136], rax
	mov rdx, QWORD PTR [rbp-40]
	mov rax, QWORD PTR [rbp-136]
	mov QWORD PTR [rdx+24], rax
	mov rcx, QWORD PTR [rbp-144]
	mov rdx, QWORD PTR [rbp-40]
	lea rcx, QWORD PTR [rdx+8]
	mov QWORD PTR [rbp-144], rcx
	mov rcx, QWORD PTR [rbp-144]
	mov rax, rcx
	mov QWORD PTR [rbp-152], rax
	mov rdx, 32
	mov QWORD PTR [rbp-160], rdx
	mov rax, QWORD PTR [rbp-160]
	mov rdi, rax
	call _eta_alloc
	mov rcx, rax
	mov QWORD PTR [rbp-24], rcx
	mov rax, QWORD PTR [rbp-24]
	mov rdx, rax
	mov QWORD PTR [rbp-168], rdx
	mov rdx, QWORD PTR [rbp-168]
	mov rcx, rdx
	mov QWORD PTR [rbp-176], rcx
	mov rax, QWORD PTR [rbp-176]
	mov QWORD PTR [rax], 3
	mov rcx, 32
	mov QWORD PTR [rbp-184], rcx
	mov rdx, QWORD PTR [rbp-184]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-192], rcx
	mov rcx, QWORD PTR [rbp-192]
	mov rax, rcx
	mov QWORD PTR [rbp-200], rax
	mov rdx, QWORD PTR [rbp-200]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-200]
	mov QWORD PTR [rax+8], 1
	mov rcx, QWORD PTR [rbp-200]
	mov QWORD PTR [rcx+16], 2
	mov rdx, QWORD PTR [rbp-200]
	mov QWORD PTR [rdx+24], 3
	mov rax, QWORD PTR [rbp-208]
	mov rcx, QWORD PTR [rbp-200]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-208], rax
	mov rdx, QWORD PTR [rbp-176]
	mov rax, QWORD PTR [rbp-208]
	mov QWORD PTR [rdx+8], rax
	mov rcx, 32
	mov QWORD PTR [rbp-216], rcx
	mov rdx, QWORD PTR [rbp-216]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-224], rcx
	mov rcx, QWORD PTR [rbp-224]
	mov rax, rcx
	mov QWORD PTR [rbp-232], rax
	mov rdx, QWORD PTR [rbp-232]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-232]
	mov QWORD PTR [rax+8], 4
	mov rcx, QWORD PTR [rbp-232]
	mov QWORD PTR [rcx+16], 5
	mov rdx, QWORD PTR [rbp-232]
	mov QWORD PTR [rdx+24], 6
	mov rax, QWORD PTR [rbp-240]
	mov rcx, QWORD PTR [rbp-232]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-240], rax
	mov rdx, QWORD PTR [rbp-176]
	mov rax, QWORD PTR [rbp-240]
	mov QWORD PTR [rdx+16], rax
	mov rcx, 32
	mov QWORD PTR [rbp-248], rcx
	mov rdx, QWORD PTR [rbp-248]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-256], rcx
	mov rcx, QWORD PTR [rbp-256]
	mov rax, rcx
	mov QWORD PTR [rbp-264], rax
	mov rdx, QWORD PTR [rbp-264]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-264]
	mov QWORD PTR [rax+8], 7
	mov rcx, QWORD PTR [rbp-264]
	mov QWORD PTR [rcx+16], 8
	mov rdx, QWORD PTR [rbp-264]
	mov QWORD PTR [rdx+24], 9
	mov rax, QWORD PTR [rbp-272]
	mov rcx, QWORD PTR [rbp-264]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-272], rax
	mov rdx, QWORD PTR [rbp-176]
	mov rax, QWORD PTR [rbp-272]
	mov QWORD PTR [rdx+24], rax
	mov rcx, QWORD PTR [rbp-280]
	mov rdx, QWORD PTR [rbp-176]
	lea rcx, QWORD PTR [rdx+8]
	mov QWORD PTR [rbp-280], rcx
	mov rcx, QWORD PTR [rbp-280]
	mov rax, rcx
	mov QWORD PTR [rbp-288], rax
	mov rdx, 32
	mov QWORD PTR [rbp-296], rdx
	mov rax, QWORD PTR [rbp-296]
	mov rdi, rax
	call _eta_alloc
	mov rcx, rax
	mov QWORD PTR [rbp-24], rcx
	mov rax, QWORD PTR [rbp-24]
	mov rdx, rax
	mov QWORD PTR [rbp-304], rdx
	mov rdx, QWORD PTR [rbp-304]
	mov rcx, rdx
	mov QWORD PTR [rbp-312], rcx
	mov rax, QWORD PTR [rbp-312]
	mov QWORD PTR [rax], 3
	mov rcx, 32
	mov QWORD PTR [rbp-320], rcx
	mov rdx, QWORD PTR [rbp-320]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-328], rcx
	mov rcx, QWORD PTR [rbp-328]
	mov rax, rcx
	mov QWORD PTR [rbp-336], rax
	mov rdx, QWORD PTR [rbp-336]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-336]
	mov QWORD PTR [rax+8], 0
	mov rcx, QWORD PTR [rbp-336]
	mov QWORD PTR [rcx+16], 0
	mov rdx, QWORD PTR [rbp-336]
	mov QWORD PTR [rdx+24], 0
	mov rax, QWORD PTR [rbp-344]
	mov rcx, QWORD PTR [rbp-336]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-344], rax
	mov rdx, QWORD PTR [rbp-312]
	mov rax, QWORD PTR [rbp-344]
	mov QWORD PTR [rdx+8], rax
	mov rcx, 32
	mov QWORD PTR [rbp-352], rcx
	mov rdx, QWORD PTR [rbp-352]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-360], rcx
	mov rcx, QWORD PTR [rbp-360]
	mov rax, rcx
	mov QWORD PTR [rbp-368], rax
	mov rdx, QWORD PTR [rbp-368]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-368]
	mov QWORD PTR [rax+8], 0
	mov rcx, QWORD PTR [rbp-368]
	mov QWORD PTR [rcx+16], 0
	mov rdx, QWORD PTR [rbp-368]
	mov QWORD PTR [rdx+24], 0
	mov rax, QWORD PTR [rbp-376]
	mov rcx, QWORD PTR [rbp-368]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-376], rax
	mov rdx, QWORD PTR [rbp-312]
	mov rax, QWORD PTR [rbp-376]
	mov QWORD PTR [rdx+16], rax
	mov rcx, 32
	mov QWORD PTR [rbp-384], rcx
	mov rdx, QWORD PTR [rbp-384]
	mov rdi, rdx
	call _eta_alloc
	mov rax, rax
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-24]
	mov rcx, rdx
	mov QWORD PTR [rbp-392], rcx
	mov rcx, QWORD PTR [rbp-392]
	mov rax, rcx
	mov QWORD PTR [rbp-400], rax
	mov rdx, QWORD PTR [rbp-400]
	mov QWORD PTR [rdx], 3
	mov rax, QWORD PTR [rbp-400]
	mov QWORD PTR [rax+8], 0
	mov rcx, QWORD PTR [rbp-400]
	mov QWORD PTR [rcx+16], 0
	mov rdx, QWORD PTR [rbp-400]
	mov QWORD PTR [rdx+24], 0
	mov rax, QWORD PTR [rbp-408]
	mov rcx, QWORD PTR [rbp-400]
	lea rax, QWORD PTR [rcx+8]
	mov QWORD PTR [rbp-408], rax
	mov rdx, QWORD PTR [rbp-312]
	mov rax, QWORD PTR [rbp-408]
	mov QWORD PTR [rdx+24], rax
	mov rcx, QWORD PTR [rbp-416]
	mov rdx, QWORD PTR [rbp-312]
	lea rcx, QWORD PTR [rdx+8]
	mov QWORD PTR [rbp-416], rcx
	mov rcx, QWORD PTR [rbp-416]
	mov rax, rcx
	mov QWORD PTR [rbp-424], rax
	mov rdx, 0
	mov QWORD PTR [rbp-432], rdx
	mov rax, 0
	mov QWORD PTR [rbp-440], rax
	mov rcx, 3
	mov QWORD PTR [rbp-448], rcx
	mov rdx, 3
	mov QWORD PTR [rbp-456], rdx
_lh15:
	mov rax, QWORD PTR [rbp-432]
	mov rcx, QWORD PTR [rbp-448]
	cmp rax, rcx
	mov rdx, QWORD PTR [rbp-464]
	setl dl
	movsx rdx, dl
	mov QWORD PTR [rbp-464], rdx
	mov rax, 1
	mov QWORD PTR [rbp-472], rax
	mov rcx, QWORD PTR [rbp-472]
	mov rdx, QWORD PTR [rbp-464]
	xor rcx, rdx
	mov QWORD PTR [rbp-472], rcx
	mov rax, QWORD PTR [rbp-472]
	test rax, rax
	jnz _lf14
_lh18:
	mov rcx, QWORD PTR [rbp-440]
	mov rdx, QWORD PTR [rbp-456]
	cmp rcx, rdx
	mov rax, QWORD PTR [rbp-480]
	setl al
	movsx rax, al
	mov QWORD PTR [rbp-480], rax
	mov rcx, 1
	mov QWORD PTR [rbp-488], rcx
	mov rdx, QWORD PTR [rbp-488]
	mov rax, QWORD PTR [rbp-480]
	xor rdx, rax
	mov QWORD PTR [rbp-488], rdx
	mov rcx, QWORD PTR [rbp-488]
	test rcx, rcx
	jnz _lf17
	mov rax, QWORD PTR [rbp-424]
	mov rdx, rax
	mov QWORD PTR [rbp-496], rdx
	mov rdx, QWORD PTR [rbp-432]
	mov rcx, rdx
	mov QWORD PTR [rbp-504], rcx
	mov rcx, QWORD PTR [rbp-496]
	mov rax, rcx
	mov QWORD PTR [rbp-512], rax
	mov rdx, QWORD PTR [rbp-512]
	sub rdx, 8
	mov QWORD PTR [rbp-512], rdx
	mov rax, QWORD PTR [rbp-504]
	mov rcx, QWORD PTR [rbp-512]
	cmp rax, QWORD PTR [rcx]
	mov rdx, QWORD PTR [rbp-520]
	setb dl
	movsx rdx, dl
	mov QWORD PTR [rbp-520], rdx
	mov rax, 1
	mov QWORD PTR [rbp-528], rax
	mov rcx, QWORD PTR [rbp-528]
	mov rdx, QWORD PTR [rbp-520]
	xor rcx, rdx
	mov QWORD PTR [rbp-528], rcx
	mov rax, QWORD PTR [rbp-528]
	test rax, rax
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-496]
	mov rax, QWORD PTR [rbp-504]
	mov rcx, QWORD PTR [rdx+rax*8]
	mov QWORD PTR [rbp-536], rcx
	mov rdx, QWORD PTR [rbp-440]
	mov rcx, rdx
	mov QWORD PTR [rbp-544], rcx
	mov rcx, QWORD PTR [rbp-536]
	mov rax, rcx
	mov QWORD PTR [rbp-552], rax
	mov rdx, QWORD PTR [rbp-552]
	sub rdx, 8
	mov QWORD PTR [rbp-552], rdx
	mov rax, QWORD PTR [rbp-544]
	mov rcx, QWORD PTR [rbp-552]
	cmp rax, QWORD PTR [rcx]
	mov rdx, QWORD PTR [rbp-560]
	setb dl
	movsx rdx, dl
	mov QWORD PTR [rbp-560], rdx
	mov rax, 1
	mov QWORD PTR [rbp-568], rax
	mov rcx, QWORD PTR [rbp-568]
	mov rdx, QWORD PTR [rbp-560]
	xor rcx, rdx
	mov QWORD PTR [rbp-568], rcx
	mov rax, QWORD PTR [rbp-568]
	test rax, rax
	jnz _eta_out_of_bounds
	mov rcx, QWORD PTR [rbp-576]
	mov rdx, QWORD PTR [rbp-536]
	mov rax, QWORD PTR [rbp-544]
	lea rcx, QWORD PTR [rdx+rax*8]
	mov QWORD PTR [rbp-576], rcx
	mov rdx, QWORD PTR [rbp-576]
	mov rcx, rdx
	mov QWORD PTR [rbp-584], rcx
	mov rcx, QWORD PTR [rbp-152]
	mov rax, rcx
	mov QWORD PTR [rbp-592], rax
	mov rax, QWORD PTR [rbp-432]
	mov rdx, rax
	mov QWORD PTR [rbp-600], rdx
	mov rdx, QWORD PTR [rbp-592]
	mov rcx, rdx
	mov QWORD PTR [rbp-608], rcx
	mov rax, QWORD PTR [rbp-608]
	sub rax, 8
	mov QWORD PTR [rbp-608], rax
	mov rcx, QWORD PTR [rbp-600]
	mov rdx, QWORD PTR [rbp-608]
	cmp rcx, QWORD PTR [rdx]
	mov rax, QWORD PTR [rbp-616]
	setb al
	movsx rax, al
	mov QWORD PTR [rbp-616], rax
	mov rcx, 1
	mov QWORD PTR [rbp-624], rcx
	mov rdx, QWORD PTR [rbp-624]
	mov rax, QWORD PTR [rbp-616]
	xor rdx, rax
	mov QWORD PTR [rbp-624], rdx
	mov rcx, QWORD PTR [rbp-624]
	test rcx, rcx
	jnz _eta_out_of_bounds
	mov rax, QWORD PTR [rbp-592]
	mov rcx, QWORD PTR [rbp-600]
	mov rdx, QWORD PTR [rax+rcx*8]
	mov QWORD PTR [rbp-632], rdx
	mov rax, QWORD PTR [rbp-440]
	mov rdx, rax
	mov QWORD PTR [rbp-640], rdx
	mov rdx, QWORD PTR [rbp-632]
	mov rcx, rdx
	mov QWORD PTR [rbp-648], rcx
	mov rax, QWORD PTR [rbp-648]
	sub rax, 8
	mov QWORD PTR [rbp-648], rax
	mov rcx, QWORD PTR [rbp-640]
	mov rdx, QWORD PTR [rbp-648]
	cmp rcx, QWORD PTR [rdx]
	mov rax, QWORD PTR [rbp-656]
	setb al
	movsx rax, al
	mov QWORD PTR [rbp-656], rax
	mov rcx, 1
	mov QWORD PTR [rbp-664], rcx
	mov rdx, QWORD PTR [rbp-664]
	mov rax, QWORD PTR [rbp-656]
	xor rdx, rax
	mov QWORD PTR [rbp-664], rdx
	mov rcx, QWORD PTR [rbp-664]
	test rcx, rcx
	jnz _eta_out_of_bounds
	mov rax, QWORD PTR [rbp-288]
	mov rdx, rax
	mov QWORD PTR [rbp-672], rdx
	mov rdx, QWORD PTR [rbp-432]
	mov rcx, rdx
	mov QWORD PTR [rbp-680], rcx
	mov rcx, QWORD PTR [rbp-672]
	mov rax, rcx
	mov QWORD PTR [rbp-688], rax
	mov rdx, QWORD PTR [rbp-688]
	sub rdx, 8
	mov QWORD PTR [rbp-688], rdx
	mov rax, QWORD PTR [rbp-680]
	mov rcx, QWORD PTR [rbp-688]
	cmp rax, QWORD PTR [rcx]
	mov rdx, QWORD PTR [rbp-696]
	setb dl
	movsx rdx, dl
	mov QWORD PTR [rbp-696], rdx
	mov rax, 1
	mov QWORD PTR [rbp-704], rax
	mov rcx, QWORD PTR [rbp-704]
	mov rdx, QWORD PTR [rbp-696]
	xor rcx, rdx
	mov QWORD PTR [rbp-704], rcx
	mov rax, QWORD PTR [rbp-704]
	test rax, rax
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-672]
	mov rax, QWORD PTR [rbp-680]
	mov rcx, QWORD PTR [rdx+rax*8]
	mov QWORD PTR [rbp-712], rcx
	mov rdx, QWORD PTR [rbp-440]
	mov rcx, rdx
	mov QWORD PTR [rbp-720], rcx
	mov rcx, QWORD PTR [rbp-712]
	mov rax, rcx
	mov QWORD PTR [rbp-728], rax
	mov rdx, QWORD PTR [rbp-728]
	sub rdx, 8
	mov QWORD PTR [rbp-728], rdx
	mov rax, QWORD PTR [rbp-720]
	mov rcx, QWORD PTR [rbp-728]
	cmp rax, QWORD PTR [rcx]
	mov rdx, QWORD PTR [rbp-736]
	setb dl
	movsx rdx, dl
	mov QWORD PTR [rbp-736], rdx
	mov rax, 1
	mov QWORD PTR [rbp-744], rax
	mov rcx, QWORD PTR [rbp-744]
	mov rdx, QWORD PTR [rbp-736]
	xor rcx, rdx
	mov QWORD PTR [rbp-744], rcx
	mov rax, QWORD PTR [rbp-744]
	test rax, rax
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-760]
	mov rcx, QWORD PTR [rdx+rdx]
	mov QWORD PTR [rbp-752], rcx
	mov rax, QWORD PTR [rbp-584]
	mov rcx, QWORD PTR [rbp-752]
	lea QWORD PTR [rax], rcx
	mov rax, QWORD PTR [rbp-424]
	mov rdx, rax
	mov QWORD PTR [rbp-768], rdx
	mov rdx, QWORD PTR [rbp-432]
	mov rcx, rdx
	mov QWORD PTR [rbp-776], rcx
	mov rcx, QWORD PTR [rbp-768]
	mov rax, rcx
	mov QWORD PTR [rbp-784], rax
	mov rdx, QWORD PTR [rbp-784]
	sub rdx, 8
	mov QWORD PTR [rbp-784], rdx
	mov rax, QWORD PTR [rbp-776]
	mov rcx, QWORD PTR [rbp-784]
	cmp rax, QWORD PTR [rcx]
	mov rdx, QWORD PTR [rbp-792]
	setb dl
	movsx rdx, dl
	mov QWORD PTR [rbp-792], rdx
	mov rax, 1
	mov QWORD PTR [rbp-800], rax
	mov rcx, QWORD PTR [rbp-800]
	mov rdx, QWORD PTR [rbp-792]
	xor rcx, rdx
	mov QWORD PTR [rbp-800], rcx
	mov rax, QWORD PTR [rbp-800]
	test rax, rax
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-768]
	mov rax, QWORD PTR [rbp-776]
	mov rcx, QWORD PTR [rdx+rax*8]
	mov QWORD PTR [rbp-808], rcx
	mov rdx, QWORD PTR [rbp-440]
	mov rcx, rdx
	mov QWORD PTR [rbp-816], rcx
	mov rcx, QWORD PTR [rbp-808]
	mov rax, rcx
	mov QWORD PTR [rbp-824], rax
	mov rdx, QWORD PTR [rbp-824]
	sub rdx, 8
	mov QWORD PTR [rbp-824], rdx
	mov rax, QWORD PTR [rbp-816]
	mov rcx, QWORD PTR [rbp-824]
	cmp rax, QWORD PTR [rcx]
	mov rdx, QWORD PTR [rbp-832]
	setb dl
	movsx rdx, dl
	mov QWORD PTR [rbp-832], rdx
	mov rax, 1
	mov QWORD PTR [rbp-840], rax
	mov rcx, QWORD PTR [rbp-840]
	mov rdx, QWORD PTR [rbp-832]
	xor rcx, rdx
	mov QWORD PTR [rbp-840], rcx
	mov rax, QWORD PTR [rbp-840]
	test rax, rax
	jnz _eta_out_of_bounds
	mov rdx, QWORD PTR [rbp-808]
	mov rax, QWORD PTR [rbp-816]
	mov rcx, QWORD PTR [rdx+rax*8]
	mov QWORD PTR [rbp-848], rcx
	mov rcx, QWORD PTR [rbp-848]
	mov rdi, rcx
	call _IunparseInt_aii
	mov rdx, rax
	mov QWORD PTR [rbp-24], rdx
	mov rcx, QWORD PTR [rbp-24]
	mov rax, rcx
	mov QWORD PTR [rbp-856], rax
	mov rax, QWORD PTR [rbp-856]
	mov rdx, rax
	mov QWORD PTR [rbp-864], rdx
	mov rcx, QWORD PTR [rbp-864]
	mov rdi, rcx
	call _Iprintln_pai
	mov rdx, QWORD PTR [rbp-440]
	add rdx, 1
	mov QWORD PTR [rbp-440], rdx
	jmp _lh18
_lf17:
	mov rax, 0
	mov QWORD PTR [rbp-440], rax
	mov rcx, QWORD PTR [rbp-432]
	add rcx, 1
	mov QWORD PTR [rbp-432], rcx
	jmp _lh15
_lf14:
	leave 
	ret 
