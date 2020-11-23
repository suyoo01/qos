use crate::{println, print};
use super::env::TrapFrame;
use core::ffi;
use crate::io::uart;


#[no_mangle]
pub extern "C" fn undefined(tf: &TrapFrame) {
    println!("undefined");
}
#[no_mangle]
pub unsafe extern "C" fn svc(tf: &TrapFrame) {
    match tf.reg[0] {
        0 => {
            let s = tf.reg[1] as *const u8;
            for i in 0..tf.reg[2] {
                print!("{}", *s.offset(i as isize) as char);
            }
        },
        _ => {}
    }
}
#[no_mangle]
pub extern "C" fn prefetch_abort(tf: &TrapFrame) {
    println!("prefetch");
}
#[no_mangle]
pub extern "C" fn data_abort(tf: &TrapFrame) {
    println!("{:x?}", tf);
}
#[no_mangle]
pub extern "C" fn irq(tf: &TrapFrame) {
    println!("irq");
}
#[no_mangle]
pub extern "C" fn fiq(tf: &TrapFrame) {
    println!("fiq");
}
