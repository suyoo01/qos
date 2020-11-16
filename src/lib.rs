#![no_std]
#![feature(asm)]
#![feature(rustc_private)]

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

pub fn test() {
    
}

pub fn repl() -> ! {
    let mut buffer:[char; 1024] = ['\0';1024];
    loop {
        print!("> ");
        get_line(&mut buffer);
        eval(&buffer);
        println!();
    }
}

fn get_line(buffer: &mut [char; 1024]) {
    let mut i = 0;
    loop {
        let c = io::uart::read() as char;
        if c == '\r' {
            buffer[i] = '\0';
            println!("");
            break;
        }
        buffer[i] = c;
        i += 1;
        print!("{}", c as char);
    }
}

fn eval(buffer: &[char; 1024]) {
    for &c in buffer {
        if c == '\0' {
            break;
        }
        print!("{}", c);
    }
}