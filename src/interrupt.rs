use crate::{println, print};
use super::env::TrapFrame;



#[no_mangle]
pub extern "C" fn undefined(tf: &TrapFrame) {
    println!("undefined");
}
#[no_mangle]
pub extern "C" fn svc(tf: &TrapFrame) {
    println!("{:?}", "svc");
}
#[no_mangle]
pub extern "C" fn prefetch_abort(tf: &TrapFrame) {
    println!("prefetch");
}
#[no_mangle]
pub extern "C" fn data_abort(tf: &TrapFrame) {
    println!("data");
}
#[no_mangle]
pub extern "C" fn irq(tf: &TrapFrame) {
    println!("irq");
}
#[no_mangle]
pub extern "C" fn fiq(tf: &TrapFrame) {
    println!("fiq");
}
