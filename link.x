ENTRY(_start)

SECTIONS
{
    . = 0x100000;
    .text.init : AT(0x100000)
    {
        *(.text.init)
    }
    . = ALIGN(16k);
    _kern_page_table = .;
    
    
    . = 0xc0108000;
    .text 0xc0108000 : AT(0x108000)
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
    PROVIDE(bootstack = .);
    . = . + 4k;
    PROVIDE(bootstack_top = .);


    .shstrtab : 
    {
        *(.shstrtab)
    }



    /DISCARD/ :
    {
        *(.ARM.*)
    }
}
