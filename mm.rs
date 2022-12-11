use limine::{LimineHhdmRequest, LimineMemmapRequest, LimineMemoryMapEntryType};

static MEMMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);

struct Freelist (Option<*mut Freelist>);

unsafe impl Send for Freelist {}
unsafe impl Sync for Freelist {}

static mut FREELIST: Freelist = Freelist(None);

pub fn alloc_page() -> Option<*mut u8> {
    let page = unsafe {FREELIST.0};
    match page {
        Some(ptr) => {
            unsafe {FREELIST.0 = (*ptr).0};
            Some(ptr.cast())
        }
        None => None
    }
}

pub fn dealloc_page(ptr: *mut u8) {
    let page: *mut Freelist = ptr.cast();
    unsafe { 
        (*page).0 = FREELIST.0;
        FREELIST.0 = Some(page);
    }
}

pub fn build_freelist() {
    let memmap = MEMMAP.get_response().get().expect("no memmap").memmap();
    for entry in memmap {
        let ent = unsafe {&*entry.as_ptr()};
        if ent.typ != LimineMemoryMapEntryType::Usable {continue}
        let base = ent.base;
        let end = base + ent.len;
        let mut page = base;
        while page < end {
            dealloc_page(page as *mut u8);
            page += 4096;
        }
    }
}