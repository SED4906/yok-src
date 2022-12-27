use crate::mm::Freelist;

pub struct Thread {
    registers: [usize;16],
    pagemap: usize,
    active: bool,
    user: bool,
    next: *mut Thread,
    prev: *mut Thread
}

#[repr(C)]
pub struct Context {
    stack: usize,
    pagemap: usize
}

static mut THREADS: Option<*mut Thread> = None;

fn get_current_thread() -> &'static mut Thread {
    let threads = unsafe {THREADS};
    if let Some(threads) = threads {
        unsafe {
            &mut*threads
        }
    } else {
        panic!("There is no current thread.")
    }
}

impl Thread {
    pub fn new(stack: usize, pagemap: usize, make_user: bool) {
        let page = Freelist::alloc();
        if let Some(ptr) = page {
            let thread: &mut Thread = unsafe{&mut *ptr.cast()};
            thread.registers = [0,0,0,0,0,0,stack,0,0,0,0,0,0,0,0,0];
            thread.pagemap = pagemap;
            thread.active = true;
            if let Some(threads) = unsafe{THREADS} {
                thread.user = make_user | unsafe{(*threads).user};
                thread.next = unsafe{(*threads).next};
                thread.prev = threads;
            } else {
                thread.user = make_user;
                thread.next = ptr.cast();
                thread.prev = ptr.cast();
                unsafe {THREADS = Some(thread)};
            }
        } else {
            panic!("Couldn't allocate memory for thread!");
        }
    }

    pub fn exit() {
        let mut thread = get_current_thread();
        thread.active = false;
        unsafe{task_switch()};
    }

    fn get_next(&self) -> &mut Thread {
        unsafe{&mut* self.next}
    }
}

#[no_mangle]
// We specify the sysv64 ABI here because when task_switch calls this function
// it expects the return value to be split between rax & rdx.
pub extern "sysv64" fn context_switch(stack: usize, pagemap: usize) -> Context {
    let mut thread = get_current_thread();
    thread.registers[6] = stack;
    thread.pagemap = pagemap;
    loop {
        thread = thread.get_next();
        if thread.active {break;}
    }
    unsafe {THREADS = Some(thread)}
    Context{stack:thread.get_next().registers[6], pagemap:thread.get_next().pagemap}
}

#[no_mangle]
pub extern "C" fn load_register(which: usize) -> usize {
    get_current_thread().registers[which]
}

#[no_mangle]
pub extern "C" fn save_register(which: usize, what: usize) {
    let thread = get_current_thread();
    if thread.active {
        thread.registers[which] = what;
    }
}

#[no_mangle]
pub extern "sysv64" fn is_usermode_thread() -> bool {
    get_current_thread().user
}

extern "C" {pub fn task_switch();}