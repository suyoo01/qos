//! Initial physial memory
//!```
//! 1G -----------> +------------------------------+
//!                 |                              |
//!                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//!                 :              .               :
//!                 :              .               :
//!                 +------------------------------+
//!                 |          boot stack          |  16KB
//!                 +------------------------------+
//!                 |  Kernel(text, data, bss)     |
//!                 +------------------------------+
//!                 |         MMIO_pgtable         |  1KB
//!                 +------------------------------+
//!                 |         kern_pgdir           |  16KB
//!                 +------------------------------+
//!                 |         .text.init           |  16KB
//! 1M -----------> +------------------------------+
//!```
//! Virtual memory map
//!```
//! 4G -----------> +------------------------------+
//!                 |          MMIO region         | 1MB
//1                 +------------------------------+
//!                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//!                 :              .               :
//!                 :              .               :
//!                 +------------------------------+
//!                 |          boot stack          |  4KB
//!                 +------------------------------+
//!                 |  Kernel(text, data, bss)     |
//!                 +------------------------------+
//!                 |         MMIO_pgtable         |  1KB
//!                 +------------------------------+
//!                 |         kern_pgdir           |  16KB
//!                 +------------------------------+
//!                 |         .text.init           |  16KB
//! 3G + 1M ------> +------------------------------+
//!                 |                              |
//!                 +------------------------------+
//!                 |            empty             |  1 MB
//! 1M -----------> +------------------------------+
//!```



extern crate compiler_builtins;
pub use compiler_builtins::mem::{memset, memcpy};
use crate::env::{UserEnv, ENVS};
use crate::{println, print};

extern "C" {
    static _bootstack: usize;
    static _bss_start: usize;
    static _bss_end: usize;
    static _kern_pgdir: usize;
}

#[derive(Clone, Copy, Debug)]
pub struct Vaddr {
    pub addr: usize
}
#[derive(Clone, Copy, Debug)]
pub struct Paddr {
    pub addr: usize
}

use super::paging::{KERN_BASE, PAGE_SIZE};
static mut next: usize = 0;


#[inline(always)]
pub fn va_to_fn(va: usize) -> usize{
    assert!(va >= KERN_BASE); // Check va is valid
    (va - KERN_BASE) / PAGE_SIZE
}

#[inline(always)]
pub fn fn_to_va(frame_number: usize) -> usize {
    frame_number * PAGE_SIZE + KERN_BASE
}

#[inline(always)]
pub fn va_to_pa(va: usize) -> usize {
    va ^ KERN_BASE
}

#[inline(always)]
pub fn pa_to_va(pa: usize) -> usize {
    pa ^ KERN_BASE
}

#[inline(always)]
pub fn pa_to_fn(pa: usize) -> usize {
    pa / PAGE_SIZE
}

#[inline(always)]
pub fn fn_to_pa(frame_number: usize) -> usize {
    frame_number * PAGE_SIZE
}

/// Initialize bss and
/// physical frame allocator.
pub unsafe fn mem_init() {
    use core::mem::size_of;
    // Init bss
    let bss_start = &_bss_start as *const usize as usize;
    let bss_end = &_bss_end as *const usize as usize;
    memset(bss_start as *mut u8, 0, bss_end - bss_start);

    // bootstack is end of kernel
    let mut end = &_bootstack as *const usize as usize;

    // ------ Init user env array ------
    let envs = &mut *(boot_alloc(&mut end, 
        size_of::<UserEnv>()) as *mut UserEnv);        
    ENVS = envs as *mut _ as usize;

    // Get first free memory
   next = va_to_fn(boot_alloc(&mut end, 0));
}

/// Allocate a physical frame
/// Panic if allocation failed
pub fn alloc_frame(num_frames: usize, flag: u32) -> usize {
    let ret;
    unsafe {
        ret = next;
        next += num_frames;
        if flag & 1 != 0 {   
            memset(fn_to_va(ret) as *mut u8, 0, num_frames*PAGE_SIZE);
        }
    }
    ret
}

/// Do nothing
pub fn free_frame(frame_number: usize) {
}

/// Allocator for initial setup
/// allocate static kernel memory
unsafe fn boot_alloc(next_free: &mut usize, size: usize) -> usize {
    use crate::{util::round_up};
    let ret = *next_free;
    *next_free = round_up(*next_free + size, PAGE_SIZE);
    memset(ret as *mut u8, 0, size);
    ret
}

