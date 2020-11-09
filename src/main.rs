#![no_std] 
#![no_main]
#![feature(global_asm)]
#![feature(asm)]
#![feature(rustc_private)]


mod mem;
mod io;

global_asm!(include_str!("init.S"));


#[no_mangle]
pub extern "C" fn entry() -> ! {
    init();
    loop {}
}

fn init() {
    unsafe {
        io::uart::uart_init();
        mem::mem_init();
    }
}



#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
