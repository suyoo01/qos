#![no_std]
#![no_main]
#![feature(global_asm)]

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn entry() -> ! { 
    let uart = 0x10009000 as *mut u32;
    unsafe {
        *uart = 'a' as u32;
    }
    loop {}   
}



#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}