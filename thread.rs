use crate::mm::alloc_page;

pub struct Thread {
    registers: [usize;16],
    pagemap: usize,
    next: *mut Thread,
    prev: *mut Thread
}

#[repr(C)]
pub struct Context {
    stack: usize,
    pagemap: usize
}

static mut THREADS: Option<*mut Thread> = None;

impl Thread {
    pub fn new(stack: usize, pagemap: usize) {
        let page = alloc_page();
        if let Some(ptr) = page {
            let thread: &mut Thread = unsafe{&mut *ptr.cast()};
            thread.registers = [0,0,0,0,0,0,stack,0,0,0,0,0,0,0,0,0];
            thread.pagemap = pagemap;
            if let Some(threads) = unsafe{THREADS} {
                thread.next = unsafe{(*threads).next};
                thread.prev = threads;
            } else {
                thread.next = ptr.cast();
                thread.prev = ptr.cast();
                unsafe {THREADS = Some(thread)};
            }
        } else {
            panic!("Couldn't allocate memory for thread!");
        }
    }
}

#[no_mangle]
pub extern "sysv64" fn context_switch(stack: usize, pagemap: usize) -> Context {
    unsafe {
        if let Some(threads) = THREADS {
            (*threads).registers[6] = stack;
            (*threads).pagemap = pagemap;
            THREADS = Some((*threads).next);
            Context{stack:(*(*threads).next).registers[6], pagemap:(*(*threads).next).pagemap}
        } else {
            Context{stack:0,pagemap:0}
        }
    }
}

#[no_mangle]
pub extern "C" fn load_register(which: usize) -> usize {
    return unsafe {  
        if let Some(threads) = THREADS {
            (*threads).registers[which]
        } else {
            panic!("no thread to load register {} state from", which)
        }
    };
}

#[no_mangle]
pub extern "C" fn save_register(which: usize, what: usize) {
    unsafe {  
        if let Some(threads) = THREADS {
            (*threads).registers[which] = what;
        } else {
            panic!("no thread to save register {} state ({}) to", which, what);
        }
    }
}

extern {pub fn task_switch();}