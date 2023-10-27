	.intel_syntax noprefix
	.text
__z:
	.zero 8
__x:
	.long 5
__y:
	.zero 8
	.globl _Imain_paai
_Imain_paai:
	push rbp
	mov rbp, rsp
	sub rsp, 40
	mov rdx, QWORD PTR [rbp-8]
	mov rdx, rdi
	mov QWORD PTR [rbp-8], rdx
	mov rcx, QWORD PTR [rbp-16]
	mov rcx, 5
	mov QWORD PTR [rbp-16], rcx
	mov rax, QWORD PTR [rbp-24]
	mov rax, 6
	mov QWORD PTR [rbp-24], rax
	mov rdx, QWORD PTR [rbp-32]
	mov rcx, QWORD PTR [rbp-16]
	mov rax, QWORD PTR [rbp-24]
	lea rdx, QWORD PTR [rcx+rax]
	mov QWORD PTR [rbp-32], rdx
	mov rdx, QWORD PTR [rbp-40]
	mov rcx, QWORD PTR [rbp-32]
	mov rdx, rcx
	mov QWORD PTR [rbp-40], rdx
	jmp __epilogue__main__
__epilogue__main__:
	mov rsp, rbp
	pop rbp
	ret 
