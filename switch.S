.extern context_switch
.extern save_register
.extern load_register

.global task_switch
task_switch:
pushq %rax
pushq %rbx
pushq %rcx
pushq %rdx
pushq %rbp
pushq %rsi
pushq %rdi
pushq %r8
pushq %r9
pushq %r10
pushq %r11
pushq %r12
pushq %r13
pushq %r14
pushq %r15

popq %rsi
movq $0, %rdi
callq save_register
popq %rsi
movq $1, %rdi
callq save_register
popq %rsi
movq $2, %rdi
callq save_register
popq %rsi
movq $3, %rdi
callq save_register
popq %rsi
movq $4, %rdi
callq save_register
popq %rsi
movq $5, %rdi
callq save_register
popq %rsi
movq $7, %rdi
callq save_register
popq %rsi
movq $8, %rdi
callq save_register
popq %rsi
movq $9, %rdi
callq save_register
popq %rsi
movq $10, %rdi
callq save_register
popq %rsi
movq $11, %rdi
callq save_register
popq %rsi
movq $12, %rdi
callq save_register
popq %rsi
movq $13, %rdi
callq save_register
popq %rsi
movq $14, %rdi
callq save_register
popq %rsi
movq $15, %rdi
callq save_register

movq %rsp, %rdi
movq %cr3, %rsi
callq context_switch
movq %rax, %rsp
movq %rdx, %cr3

movq $0, %rdi
callq load_register
pushq %rax
movq $1, %rdi
callq load_register
pushq %rax
movq $2, %rdi
callq load_register
pushq %rax
movq $3, %rdi
callq load_register
pushq %rax
movq $4, %rdi
callq load_register
pushq %rax
movq $5, %rdi
callq load_register
pushq %rax
movq $7, %rdi
callq load_register
pushq %rax
movq $8, %rdi
callq load_register
pushq %rax
movq $9, %rdi
callq load_register
pushq %rax
movq $10, %rdi
callq load_register
pushq %rax
movq $11, %rdi
callq load_register
pushq %rax
movq $12, %rdi
callq load_register
pushq %rax
movq $13, %rdi
callq load_register
pushq %rax
movq $14, %rdi
callq load_register
pushq %rax
movq $15, %rdi
callq load_register
pushq %rax
popq %r15
popq %r14
popq %r13
popq %r12
popq %r11
popq %r10
popq %r9
popq %r8
popq %rdi
popq %rsi
popq %rbp
popq %rdx
popq %rcx
popq %rbx
popq %rax
retq