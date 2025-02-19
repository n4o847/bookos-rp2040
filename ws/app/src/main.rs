#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}

#[link_section = ".boot2"]
#[no_mangle]
pub static BOOT2: [u8; 256] = *include_bytes!(env!("BOOT2"));

#[no_mangle]
pub extern "C" fn _entry_point() -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}
