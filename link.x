MEMORY 
{
    RAM (rwx): ORIGIN = 0x60000000, LENGTH = 512M
}

ENTRY(reset_handler)
SECTIONS
{
    .text :
    {
        *(.text.boot)
        *(.text .text.*)
        *(.rodata .rodata.*)
    } > RAM
    .data :
    {
        *(.data .data.*)
        . = ALIGN(8);
    } > RAM
    .bss :
    {
        _bss_start = .;
        *(.bss .bss.*)
        _bss_end = .;
    } > RAM
    /DISCARD/ :
    {
        *(.ARM.*)
    }
}