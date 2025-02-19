#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use rt::hal;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfi") };
    }
}

#[link_section = ".text"]
#[no_mangle]
pub extern "C" fn _boot2() -> ! {
    hal::reset_io_bank0();
    hal::setup_gpio25();

    unsafe { (hal::SIO_GPIO_OE_SET as *mut u32).write_volatile(1 << 25) };

    loop {
        unsafe { (hal::SIO_GPIO_OUT_SET as *mut u32).write_volatile(1 << 25) }

        for _ in 0..0x100000 {
            unsafe { asm!("nop") };
        }

        unsafe { (hal::SIO_GPIO_OUT_CLR as *mut u32).write_volatile(1 << 25) };

        for _ in 0..0x100000 {
            unsafe { asm!("nop") };
        }
    }
}
