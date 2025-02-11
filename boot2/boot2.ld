MEMORY
{
    SRAM (rx) : ORIGIN = 0x20041f00, LENGTH = 252
}

ENTRY(_boot2)

SECTIONS
{
    .text : {
    } > SRAM
}
