use core::ptr::NonNull;
use limine::LimineMemmapRequest;

static MEMORY_MAP: LimineMemmapRequest = LimineMemmapRequest::new(0);
pub trait PageMap {
    fn get_page(&self, addr: usize) -> Result<PageMapEntry, PageError>;
    fn set_page(&self, addr: usize, page: Option<PageMapEntry>, allocate: bool) -> Result<(), PageError>;
    unsafe fn switch_to(&self);
}

pub trait PageFrame {
    fn addr(&self) -> usize;
    fn present(&self) -> bool;
    fn user_mode(&self) -> bool;
    fn writable(&self) -> bool;
    fn executable(&self) -> bool;
    fn set_addr(&mut self, value: usize);
    fn set_present(&mut self, value: bool);
    fn set_user_mode(&mut self, value: bool);
    fn set_writable(&mut self, value: bool);
    fn set_executable(&mut self, value: bool);
}

pub enum PageError {
    OutOfMemory,
    AllocationFailed,
    InvalidFrame,
    NotPresent,
}

impl core::fmt::Debug for PageError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", match self {
            Self::OutOfMemory => "out of memory",
            Self::AllocationFailed => "allocation failed",
            Self::InvalidFrame => "invalid frame",
            Self::NotPresent => "address not mapped",
        })
    }
}

#[derive(Clone,Copy,Default)]
pub struct PageMapEntry(usize);

#[derive(Clone,Copy)]
pub struct Freelist(Option<NonNull<Freelist>>);

pub static mut FREELIST: Freelist = Freelist(None);

impl Freelist {
    pub unsafe fn insert(addr: usize) {
        if let Some(page)= NonNull::new(addr as *mut Freelist) {
            *page.as_ptr() = FREELIST;
            unsafe{FREELIST.0 = Some(page)};
        }
    }
    pub unsafe fn alloc() -> Result<usize,PageError> {
        if let Some(page) = FREELIST.0 {
            FREELIST = *page.as_ptr();
            return Ok(page.addr().get());
        }
        Err(PageError::OutOfMemory)
    }
}

pub struct PageTable (*mut [PageMapEntry;512]);

impl PageFrame for PageMapEntry {
    fn addr(&self) -> usize {
        self.0 & !0x1FF
    }
    fn present(&self) -> bool {
        self.0 & 1 == 1
    }
    fn writable(&self) -> bool {
        self.0 & 2 == 1
    }
    fn user_mode(&self) -> bool {
        self.0 & 4 == 1
    }
    fn executable(&self) -> bool {
        self.0 & (1<<63) == 0
    }
    fn set_addr(&mut self, value: usize) {
        self.0 = (self.0 & 0x1FF) | (value & !0x1FF);
    }
    fn set_present(&mut self, value: bool) {
        self.0 = (self.0 & !1) | (value as usize);
    }
    fn set_writable(&mut self, value: bool) {
        self.0 = (self.0 & !2) | ((value as usize) << 1);
    }
    fn set_user_mode(&mut self, value: bool) {
        self.0 = (self.0 & !4) | ((value as usize) << 2);
    }
    fn set_executable(&mut self, value: bool) {
        self.0 = (self.0 & !(1<<63)) | ((!value as usize) << 2);
    }
}

impl PageTable {
    fn descend(&self, addr: usize, allocate: bool, level: usize) -> Result<PageTable, PageError> {
        let entries = match unsafe{self.0.as_mut()} {
            Some(x) => x,
            None => return Err(PageError::InvalidFrame)
        };
        match entries.get_mut((addr >> (12 + 9*level)) & 0x1FF) {
            Some(entry) if entry.0 & 1 == 1 => Ok(PageTable((entry.0 & !0xFFF) as *mut [PageMapEntry;512])),
            Some(entry) if allocate => {
                *entry=unsafe{PageMapEntry(Freelist::alloc()?)};
                Ok(PageTable((*entry).0 as *mut [PageMapEntry;512]))
            },
            Some(_) => Err(PageError::NotPresent),
            None => Err(PageError::InvalidFrame)
        }
    }
    fn grab(&self, addr: usize, level: usize) -> Result<PageMapEntry, PageError> {
        let entries = match unsafe{self.0.as_ref()} {
            Some(x) => x,
            None => return Err(PageError::InvalidFrame)
        };
        match entries.get((addr >> (12 + 9*level)) & 0x1FF) {
            Some(entry) => Ok(*entry),
            None => Err(PageError::InvalidFrame)
        }
    }
    fn poke(&self, addr: usize, page: Option<PageMapEntry>, level: usize) -> Result<(), PageError>{
        let entries = match unsafe{self.0.as_mut()} {
            Some(x) => x,
            None => return Err(PageError::InvalidFrame)
        };
        match entries.get_mut((addr >> (12 + 9*level)) & 0x1FF) {
            Some(entry) => Ok({*entry=page.unwrap_or_default();}),
            None => Err(PageError::InvalidFrame)
        }
    }
}

impl PageMap for PageTable {
    fn get_page(&self, addr: usize) -> Result<PageMapEntry, PageError> {
        let pml3 = self.descend(addr,false,3)?;
        let pml2 = pml3.descend(addr,false,2)?;
        let pml1 = pml2.descend(addr,false,1)?;
        pml1.grab(addr,0)
    }
    fn set_page(&self, addr: usize, page: Option<PageMapEntry>, allocate: bool) -> Result<(), PageError> {
        let pml3 = self.descend(addr,allocate,3)?;
        let pml2 = pml3.descend(addr,allocate,2)?;
        let pml1 = pml2.descend(addr,allocate,1)?;
        pml1.poke(addr,page,0)
    }
    unsafe fn switch_to(&self) {
        x86::controlregs::cr3_write(self.0 as u64);
    } 
}

pub fn init() {
    let memory_map = MEMORY_MAP.get_response().get().expect("no memory map").memmap();
    for entry in memory_map.iter().map(core::ops::Deref::deref) {
        if entry.typ != limine::LimineMemoryMapEntryType::Usable {
            continue;
        }
        let mut base = entry.base;
        let end = base + entry.len;
        while base < end {
            unsafe{Freelist::insert(base.try_into().unwrap())};
            base += 4096;
        }
    }
}