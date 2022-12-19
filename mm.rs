use limine::{LimineHhdmRequest, LimineMemmapRequest, LimineMemoryMapEntryType};

static MEMMAP: LimineMemmapRequest = LimineMemmapRequest::new(0);
static HHDM: LimineHhdmRequest = LimineHhdmRequest::new(0);

pub struct Freelist (Option<*mut Freelist>);

#[repr(C)]
pub struct Pagemap (pub *mut [usize;512]);

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

impl Pagemap {
    pub fn read_mapping(pagemap: &Pagemap, vaddr: usize) -> Option<usize> {
        let hhdm = HHDM.get_response().get().expect("no hhdm").offset;
        let pagemap_hhdm = unsafe{&*pagemap.0.byte_offset(hhdm as isize)};
        let entry4 = pagemap_hhdm[(vaddr >> 39) & 0x1FF];
        if entry4 & 1 == 0 { return None; }
        let pml3 = entry4 & !0xFFF;
        let pml3_hhdm = unsafe{&*(pml3 as *mut [usize;512]).byte_offset(hhdm as isize)};
        let entry3 = pml3_hhdm[(vaddr >> 30) & 0x1FF];
        if entry3 & 1 == 0 { return None; }
        let pml2 = entry3 & !0xFFF;
        let pml2_hhdm = unsafe{&*(pml2 as *mut [usize;512]).byte_offset(hhdm as isize)};
        let entry2 = pml2_hhdm[(vaddr >> 21) & 0x1FF];
        if entry2 & 1 == 0 { return None; }
        let pml1 = entry2 & !0xFFF;
        let pml1_hhdm = unsafe{&*(pml1 as *mut [usize;512]).byte_offset(hhdm as isize)};
        return Some(pml1_hhdm[(vaddr >> 12) & 0x1FF]);
    }

    pub fn update_mapping(pagemap: &Pagemap, vaddr: usize, entry: usize) -> bool {
        let hhdm = HHDM.get_response().get().expect("no hhdm").offset;
        let pagemap_hhdm = unsafe{&*pagemap.0.byte_offset(hhdm as isize)};
        let entry4 = pagemap_hhdm[(vaddr >> 39) & 0x1FF];
        if entry4 & 1 == 0 { return false; }
        let pml3 = entry4 & !0xFFF;
        let pml3_hhdm = unsafe{&*(pml3 as *mut [usize;512]).byte_offset(hhdm as isize)};
        let entry3 = pml3_hhdm[(vaddr >> 30) & 0x1FF];
        if entry3 & 1 == 0 { return false; }
        let pml2 = entry3 & !0xFFF;
        let pml2_hhdm = unsafe{&*(pml2 as *mut [usize;512]).byte_offset(hhdm as isize)};
        let entry2 = pml2_hhdm[(vaddr >> 21) & 0x1FF];
        if entry2 & 1 == 0 { return false; }
        let pml1 = entry2 & !0xFFF;
        let pml1_hhdm = unsafe{&mut*(pml1 as *mut [usize;512]).byte_offset(hhdm as isize)};
        pml1_hhdm[(vaddr >> 12) & 0x1FF] = entry;
        return true;
    }

    pub fn create_mapping(pagemap: &Pagemap, vaddr: usize, paddr: usize, flags: usize) -> bool {
        let hhdm = HHDM.get_response().get().expect("no hhdm").offset;
        let pagemap_hhdm = unsafe{&mut*pagemap.0.byte_offset(hhdm as isize)};
        let entry4 = Freelist::alloc().unwrap_or_else(|| panic!("out of memory creating level 4 mapping on {} from {} to {} with flags {}",pagemap.0 as usize,vaddr,paddr,flags)) as usize | flags;
        pagemap_hhdm[(vaddr >> 39) & 0x1FF] = entry4;
        if entry4 & 1 == 0 { return false; }
        let pml3 = entry4 & !0xFFF;
        let pml3_hhdm = unsafe{&mut*(pml3 as *mut [usize;512]).byte_offset(hhdm as isize)};
        let entry3 = Freelist::alloc().unwrap_or_else(|| panic!("out of memory creating level 3 mapping on {} from {} to {} with flags {}",pagemap.0 as usize,vaddr,paddr,flags)) as usize | flags;
        pml3_hhdm[(vaddr >> 30) & 0x1FF] = entry3;
        if entry3 & 1 == 0 { return false; }
        let pml2 = entry3 & !0xFFF;
        let pml2_hhdm = unsafe{&mut*(pml2 as *mut [usize;512]).byte_offset(hhdm as isize)};
        let entry2 = Freelist::alloc().unwrap_or_else(|| panic!("out of memory creating level 2 mapping on {} from {} to {} with flags {}",pagemap.0 as usize,vaddr,paddr,flags)) as usize | flags;
        pml2_hhdm[(vaddr >> 21) & 0x1FF] = entry2;
        if entry2 & 1 == 0 { return false; }
        let pml1 = entry2 & !0xFFF;
        let pml1_hhdm = unsafe{&mut*(pml1 as *mut [usize;512]).byte_offset(hhdm as isize)};
        pml1_hhdm[(vaddr >> 12) & 0x1FF] = paddr | flags;
        return true;
    }
}

extern {
    pub fn get_pagemap() -> Pagemap;
}