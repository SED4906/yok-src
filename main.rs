#![feature(pointer_byte_offsets)]
#![no_std]
#![no_main]

mod gdt;
mod interrupts;
mod io;
mod mm;
mod thread;

use limine::LimineBootInfoRequest;

static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);

/// Kernel Entry Point
///
/// `_start` is defined in the linker script as the entry point for the ELF file.
/// Unless the [`Entry Point`](limine::LimineEntryPointRequest) feature is requested,
/// the bootloader will transfer control to this function.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hi");

    if let Some(bootinfo) = BOOTLOADER_INFO.get_response().get() {
        println!(
            "booted by {} v{}",
            bootinfo.name.to_str().unwrap().to_str().unwrap(),
            bootinfo.version.to_str().unwrap().to_str().unwrap(),
        );
    }

    mm::build_freelist();
    println!("memory ok");
    unsafe{gdt::init()};
    println!("gdt ok");
    interrupts::setup_interrupts();
    println!("interrupts ok");
    thread::Thread::new(0, 0, false); // Our rsp and cr3 should not be 0, so
    unsafe{thread::task_switch();} // we switch tasks to set the right values there.
    println!("threads ok");
    hcf();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    hcf();
}

/// Die, spectacularly.
pub fn hcf() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
