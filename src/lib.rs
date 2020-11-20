#![no_std]
#![feature(asm)]
#![feature(rustc_private)]
#![feature(global_asm)]

pub mod io;
pub mod mem;
pub mod paging;
pub mod reg;
pub mod util;
pub mod interrupt;
pub mod env;

global_asm!(include_str!("init.S"));


pub fn init() {
    unsafe {
        io::uart::uart_init();// Initialize UART
        println!("Init uart");        
        paging::page_init(); // Initialize kernel page mapping
        println!("Init page");
        mem::mem_init(); // Initialize memory allocator
        println!("Init allocator");
    }
}

pub fn test() {
}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    unsafe { loop { asm!("wfe") } }
}