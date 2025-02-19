use core::arch::asm;

// See [2.1.2. Atomic Register Access]
pub const REG_CLR: usize = 0x3000;

// See [2.2.2. Detail]
pub const RESETS_BASE: usize = 0x4000c000;
pub const IO_BANK0_BASE: usize = 0x40014000;
pub const SIO_BASE: usize = 0xd0000000;

// See [2.3.1.7. List of Registers]
pub const SIO_GPIO_OUT_SET: usize = SIO_BASE + 0x14;
pub const SIO_GPIO_OUT_CLR: usize = SIO_BASE + 0x18;
pub const SIO_GPIO_OE_SET: usize = SIO_BASE + 0x24;
pub const SIO_GPIO_OE_CLR: usize = SIO_BASE + 0x28;

// See [2.14.3. List of Registers]
pub const RESETS_RESET: usize = RESETS_BASE + 0x0;
pub const RESETS_RESET_DONE: usize = RESETS_BASE + 0x8;
pub const RESETS_IO_BANK0: u32 = 1 << 5;

// See [2.19.6.1. IO - User Bank]
pub const IO_BANK0_GPIO25_CTRL: usize = IO_BANK0_BASE + 0xcc;

pub fn reset_io_bank0() {
    unsafe { ((RESETS_RESET + REG_CLR) as *mut u32).write_volatile(RESETS_IO_BANK0) };
    while unsafe { (RESETS_RESET_DONE as *const u32).read_volatile() } & RESETS_IO_BANK0 == 0 {
        unsafe { asm!("nop") };
    }
}

pub fn setup_gpio25() {
    unsafe { (SIO_GPIO_OE_CLR as *mut u32).write_volatile(1 << 25) };
    unsafe { (SIO_GPIO_OUT_CLR as *mut u32).write_volatile(1 << 25) };

    unsafe { (IO_BANK0_GPIO25_CTRL as *mut u32).write_volatile(0x5) };
}
