use limine::{LimineHhdmRequest, LimineMemmapRequest, LimineMemoryMapEntryType};

static MEMMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);
static HHDM: LimineHhdmRequest = LimineHhdmRequest::new(0);

pub struct Freelist (Option<*mut Freelist>);

unsafe impl Send for Freelist {}
unsafe impl Sync for Freelist {}

static mut FREELIST: Freelist = Freelist(None);
static mut HHDM_VAL: Option<u64> = None;

impl Freelist {
    pub fn alloc() -> Option<*mut u8> {
        let page = unsafe {FREELIST.0};
        let hhdm = unsafe{HHDM_VAL.expect("no hhdm on alloc?")};
        match page {
            Some(ptr) => {
                let ptr_hhdm: *mut Freelist = unsafe{ptr.byte_offset(hhdm as isize).cast()};
                unsafe {FREELIST.0 = (*ptr_hhdm).0};
                Some(ptr.cast())
            }
            None => None
        }
    }

    pub fn dealloc(ptr: *mut u8) {
        let page: *mut Freelist = ptr.cast();
        let hhdm = unsafe{HHDM_VAL.expect("no hhdm on dealloc?")};
        let page_hhdm: *mut Freelist = unsafe{ptr.byte_offset(hhdm as isize).cast()};
        unsafe { 
            (*page_hhdm).0 = FREELIST.0;
            FREELIST.0 = Some(page);
        }
    }
}

pub fn build_freelist() {
    let memmap = MEMMAP.get_response().get().expect("no memmap").memmap();
    let hhdm = HHDM.get_response().get().expect("no hhdm").offset;
    unsafe{HHDM_VAL = Some(hhdm)};
    for entry in memmap {
        let ent = unsafe {&*entry.as_ptr()};
        if ent.typ != LimineMemoryMapEntryType::Usable {continue}
        let base = ent.base;
        let end = base + ent.len;
        let mut page = base;
        while page < end {
            Freelist::dealloc(page as *mut u8);
            page += 4096;
        }
    }
}