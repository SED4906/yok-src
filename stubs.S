.extern interrupt_stub_err_handler
.extern interrupt_stub_no_err_handler
.extern interrupt_stub_picm_handler
.extern interrupt_stub_pics_handler
.global interrupt_stub_err
interrupt_stub_err:
popq %rdi
callq interrupt_stub_err_handler
iretq

.global interrupt_stub_no_err
interrupt_stub_no_err:
callq interrupt_stub_no_err_handler
iretq

.global interrupt_stub_picm
interrupt_stub_picm:
callq interrupt_stub_picm_handler
push %rax
movb $0x20, %al
outb %al, $0x20
pop %rax
iretq

.global interrupt_stub_pics
interrupt_stub_pics:
callq interrupt_stub_pics_handler
push %rax
movb $0x20, %al
outb %al, $0x20
outb %al, $0xA0
pop %rax
iretq

.global enable_interrupts
enable_interrupts:
lidt (%rdi)
sti
retq