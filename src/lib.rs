#![no_std]
#![feature(asm)]
#![feature(rustc_private)]
#![feature(const_raw_ptr_deref)]
#![feature(const_mut_refs)]

pub mod io;
pub mod mem;
pub mod paging;
pub mod reg;
pub mod util;
pub mod interrupt;


pub fn init() {
    unsafe {
        io::uart::uart_init();// Initialize UART
        println!("Init uart");
        paging::page_init(); // Initialize kernel page mapping
        println!("Init page");
        mem::mem_init(); // Initialize memory allocator
        println!("Init allocator");
        interrupt::interrupt_init();
        println!("Init Interrupt");
    }
}
