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
pub mod lock;
pub mod elf;

global_asm!(include_str!("init.S"));
static USER_PROG:&[u8] = include_bytes!("../usr/shell");

pub fn init() {
    unsafe {
        io::uart::uart_init();// Initialize UART
        println!("Init uart");        
        paging::page_init(); // Initialize kernel page mapping
        println!("Init page");
        mem::mem_init(); // Initialize memory allocator
        println!("Init allocator");
        env::env_init();
        let user_prog = mem::fn_to_va(mem::alloc_frame(3, 0));
        use mem::memcpy;
        memcpy(user_prog as *mut u8, USER_PROG.as_ptr(), USER_PROG.len());
        println!("{:x}", user_prog);
        env::env_create(user_prog);
    }
}

pub fn test() {
}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    unsafe { loop { asm!("wfe") } }
}