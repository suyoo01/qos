#![no_std]
#![no_main]
#![feature(global_asm)]

#[no_mangle]
pub extern "C" fn entry() -> ! {
    kernel::init();
    kernel::test();
    panic!("Done");
}


