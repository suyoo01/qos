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
        . = ALIGN(8);
        _bss_end = .;
    } > RAM

    /DISCARD/ :
    {
        *(.ARM.*)
    }

    _fiq_stack_start = ADDR(.bss) + SIZEOF(.bss);
    _fiq_stack_end = _fiq_stack_start + 0x1000;

    _irq_stack_start = _fiq_stack_end;
    _irq_stack_end = _irq_stack_start + 0x1000;

    _svc_stack_start = _irq_stack_end;
    _svc_stack_end = _svc_stack_start + 0x1000;

}