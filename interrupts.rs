use crate::mm::Freelist;

#[repr(C)]
pub struct InterruptGate {
    offset_1: u16,
    selector: u16,
    ist: u8,
    attrs: u8,
    offset_2: u16,
    offset_3: u32,
    zero: u32,
}

#[repr(C, packed)]
pub struct InterruptTablePtr {
    size: u16,
    offset: *mut [InterruptGate;256]
}

static mut IDTR: InterruptTablePtr = InterruptTablePtr {size:0,offset:0 as *mut [InterruptGate;256]};
static mut IDT: Option<*mut [InterruptGate;256]> = None;

impl InterruptGate {
    pub fn new(offset: usize, selector: u16, ist: u8, attrs: u8) -> Self {
        Self {
            offset_1: (offset & 0xFFFF) as u16,
            selector: selector,
            ist, attrs,
            offset_2: ((offset >> 16) & 0xFFFF) as u16,
            offset_3: (offset >> 32) as u32,
            zero:0
        }
    }
}

impl InterruptTablePtr {
    pub fn new(entries: u16, offset: *mut [InterruptGate;256]) -> Self {
        Self {
            size:entries*16-1,
            offset
        }
    }
}

#[no_mangle]
extern "C" fn interrupt_stub_err_handler(code: usize) {
    panic!("recieved unhandled interrupt with error code {}", code);
}

#[no_mangle]
extern "C" fn interrupt_stub_no_err_handler() {
    panic!("recieved unhandled interrupt");
}

extern {
    fn interrupt_stub_no_err();
    fn interrupt_stub_err();
    fn enable_interrupts(idtr: *mut InterruptTablePtr);
}

pub fn setup_interrupts() {
    unsafe{
        IDT = Some(Freelist::alloc().expect("no memory for interrupt table").cast());
        if let Some(idt) = IDT {
            for entry in 0..=9 {
                (*idt)[entry] = InterruptGate::new(interrupt_stub_no_err as usize, 0x28, 0, 0x8E);
            }
            (*idt)[8] = InterruptGate::new(interrupt_stub_err as usize, 0x28, 0, 0x8E);
            for entry in 10..=14 {
                (*idt)[entry] = InterruptGate::new(interrupt_stub_err as usize, 0x28, 0, 0x8E);
            }
            for entry in 15..=31 {
                (*idt)[entry] = InterruptGate::new(interrupt_stub_no_err as usize, 0x28, 0, 0x8E);
            }
            (*idt)[17] = InterruptGate::new(interrupt_stub_err as usize, 0x28, 0, 0x8E);
            (*idt)[21] = InterruptGate::new(interrupt_stub_err as usize, 0x28, 0, 0x8E);
            (*idt)[29] = InterruptGate::new(interrupt_stub_err as usize, 0x28, 0, 0x8E);
            (*idt)[30] = InterruptGate::new(interrupt_stub_err as usize, 0x28, 0, 0x8E);
        }
        IDTR = InterruptTablePtr::new(32, IDT.unwrap());
        enable_interrupts(&mut IDTR);
    };
}