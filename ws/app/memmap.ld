MEMORY
{
    FLASH (rx) : ORIGIN = 0x10000000, LENGTH = 2048K
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 256K
}

ENTRY(_entry_point)

SECTIONS
{
    .boot2 : {
        KEEP(*(.boot2))
    } > FLASH

    ASSERT(SIZEOF(.boot2) == 256, "the size of the second stage bootloader must be 256 bytes")

    .text : {
        *(.text.*)
    } > FLASH

    .ARM.exidx : {
    } > FLASH
}
