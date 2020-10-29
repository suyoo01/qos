#![no_std] 
#![no_main]
#![feature(global_asm)]
 
global_asm!(include_str!("boot.s")); 
  

mod uart;


#[no_mangle]
pub extern "C" fn entry() -> ! { 
    unsafe {
        uart::init();
    }
    uart::write('a' as u32);
    loop {}
}



#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
