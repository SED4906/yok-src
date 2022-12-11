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