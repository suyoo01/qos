#![no_std]
#![no_main]
#![feature(global_asm)]

global_asm!(include_str!("boot.s"));

mod zynq;
use zynq::uart::uart_regs;

#[no_mangle]
pub extern "C" fn entry() -> ! { 
    let uart = (0x10009000 - 0x30) as *mut uart_regs;
    loop {
        unsafe {
            let c = (*uart).fifo.read();
            if c != 0 {
                (*uart).fifo.write(c);
            }
        }
    }   
}



#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}