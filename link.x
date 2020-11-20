ENTRY(_start)

SECTIONS
{
    . = 1M;
    .text.vector : AT(0x100000) {
        KEEP(*(.text.vector))
    }
    .text.init :
    {
        *(.text.init)
    }
    . = ALIGN(16k);
    _kern_pgdir = .;
    . = . + 16k;
    _mmio_pgtable = .;
    . = . + 1k;
    
    
    . = 0xc0109000;
    .text 0xc0109000 : AT(0x109000)
    {
        *(.text .text.*)     
    }
    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .data :
    {
        *(.data .data.*)
        . = ALIGN(4);
    }

    .bss :
    {
        _bss_start = . ;
        *(.bss .bss.*)
        . = ALIGN(4);
        _bss_end = . ; 
    }

    . = ALIGN(4k);
    . = . + 16k;
    _bootstack = .;



    .shstrtab : 
    {
        *(.shstrtab)
    }



    /DISCARD/ :
    {
        *(.ARM.*)
    }
}
