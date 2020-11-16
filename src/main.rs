#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]
#![feature(rustc_private)]

global_asm!(include_str!("init.S"));

pub mod io;
pub mod mem;
pub mod paging;
pub mod reg;
pub mod util;
pub mod interrupt;

use mem::page_alloc;
use paging::map_va_to_fn;

#[no_mangle]
pub extern "C" fn entry() -> ! {
    os::init();
    os::test();
    os::repl();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    unsafe { loop { asm!("wfe") } }
}
