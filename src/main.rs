#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn entry() -> ! {
    os::init();
    os::test();
    panic!("Done");
}


