extern crate compiler_builtins;
use compiler_builtins::mem::memset;


extern "C" {
    static bootstack: *mut u8;
    static _bss_start: *mut u8;
    static _bss_end: *mut u8;
}


pub unsafe fn mem_init() {
    // Init bss
    memset(_bss_start, 0, 
        _bss_end as usize - _bss_start as usize);
}