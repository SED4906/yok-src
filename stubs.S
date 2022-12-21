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

.global enable_interrupts
enable_interrupts:
lidt (%rdi)
sti
retq