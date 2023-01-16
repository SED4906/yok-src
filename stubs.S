.extern interrupt_stub_err_handler
.extern interrupt_stub_no_err_handler
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

.extern interrupt_keyb_handler
.global interrupt_keyb
interrupt_keyb:
pushq %rdi
pushq %rax
inb $0x60, %al
xchg %rdi, %rax
callq interrupt_keyb_handler
movb $0x20, %al
outb %al, $0x20
popq %rax
popq %rdi
iretq

.global pic_remap
pic_remap:
pushq %rax
pushq %rcx
pushq %rdx
inb $0x21, %al
xchg %al, %cl
inb $0xA1, %al
xchg %al, %dl
movb $0x11, %al
outb %al, $0x20
outb %al, $0xA0
movb $0x20, %al
outb %al, $0x21
movb $0x28, %al
outb %al, $0xA1
movb $4, %al
outb %al, $0x21
movb $2, %al
outb %al, $0xA1
movb $1, %al
outb %al, $0x21
outb %al, $0xA1
xchg %al, %cl
outb %al, $0x21
xchg %al, %dl
outb %al, $0xA1
popq %rdx
popq %rcx
popq %rax
retq

.global pic_unmask_devices
pic_unmask_devices:
pushq %rax
movb $0xFD, %al
outb %al, $0x21
popq %rax
retq