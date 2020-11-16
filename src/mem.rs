extern crate compiler_builtins;
use compiler_builtins::mem::memset;
use spin::Mutex;
use crate::{println, print};

extern "C" {
    static _bootstack: usize;
    static _bss_start: usize;
    static _bss_end: usize;
    static _kern_pgdir: usize;
}

//pub static kern_pgdir:&mut [u32; 4096] = unsafe {_kern_pgdir};

// Initial physial memory
// 1G -----------> +------------------------------+
//                 |                              |
//                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//                 :              .               :
//                 :              .               :
//                 +------------------------------+
//                 |          boot stack          |  16KB
//                 +------------------------------+
//                 |  Kernel(text, data, bss)     |
//                 +------------------------------+
//                 |         MMIO_pgtable         |  1KB
//                 +------------------------------+
//                 |         kern_pgdir           |  16KB
//                 +------------------------------+
//                 |         .text.init           |  16KB
// 1M -----------> +------------------------------+

// Virtual memory map
// 4G -----------> +------------------------------+
//                 |          MMIO region         | 1MB
//                 +------------------------------+
//                 ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//                 :              .               :
//                 :              .               :
//                 +------------------------------+
//                 |          boot stack          |  4KB
//                 +------------------------------+
//                 |  Kernel(text, data, bss)     |
//                 +------------------------------+
//                 |         MMIO_pgtable         |  1KB
//                 +------------------------------+
//                 |         kern_pgdir           |  16KB
//                 +------------------------------+
//                 |         .text.init           |  16KB
// 3G + 1M ------> +------------------------------+
//                 |                              |
//                 +------------------------------+
//                 |            empty             |  1 MB
// 1M -----------> +------------------------------+

struct Frame {
    next: usize,
    ref_count: u32,
}


const NUM_PAGES:usize = (1<<30) / PAGE_SIZE;
use super::paging::{KERN_BASE, PAGE_SIZE};

static FRAME_ALLOCATOR: Mutex<usize> = Mutex::new(0);

#[repr(C)]
struct FrameAllocator {
    frames: [Frame; NUM_PAGES],
    next_free: usize
}

impl FrameAllocator {
    /// Allocate free frame
    /// return frame number and increment ref_count
    fn allocate_frame(&mut self) -> Option<usize> {
        if self.next_free != 0 {
            let free_frame_number = self.next_free;
            self.frames[self.next_free].ref_count += 1;
            self.next_free = self.frames[self.next_free].next;
            Some(free_frame_number)
        } else {
            None
        }
    }

    /// Deallocate frame
    /// If ref count is 0, mark frame as free
    fn deallocate_frame(&mut self, frame_number: usize) {
        self.frames[frame_number].ref_count -= 1;
        if self.frames[frame_number].ref_count == 0 {
            self.frames[frame_number].next = self.next_free;
            self.next_free = frame_number;
        }
    }
}

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
    // Init bss
    let bss_start = &_bss_start as *const usize as usize;
    let bss_end = &_bss_end as *const usize as usize;
    memset(bss_start as *mut u8, 0, bss_end - bss_start);

    // bootstack is end of kernel
    let mut end = &_bootstack as *const usize as usize;


    let frame_allocator = &mut *(boot_alloc(&mut end,
    core::mem::size_of::<FrameAllocator>()) as *mut FrameAllocator);

    for (i,frame) in frame_allocator.frames.iter_mut().enumerate() {
        frame.next = i+1;
    }

    frame_allocator.frames[NUM_PAGES-1].next = 0;

    let next_free = boot_alloc(&mut end, 0);
    frame_allocator.next_free = va_to_fn(next_free);
    *FRAME_ALLOCATOR.lock() = frame_allocator as *mut _ as usize;
}

fn allocate_frame() -> Option<usize> {
    unsafe {
        let frame_allocator = &mut *(*FRAME_ALLOCATOR.lock() as *mut FrameAllocator);
        frame_allocator.allocate_frame()
    }
}

fn deallocate_frame(frame_number: usize) {
    unsafe {
        let frame_allocator = &mut *(*FRAME_ALLOCATOR.lock() as *mut FrameAllocator);
        frame_allocator.deallocate_frame(frame_number);
    }
}

/// Allocate a physical frame
/// Panic if allocation failed
pub fn page_alloc(alloc_flag: u32) -> usize {
    let frame_number = allocate_frame().unwrap();
    if alloc_flag & 1 != 0 {
        let va = fn_to_va(frame_number);
        unsafe {
            memset(va as *mut u8, 0, PAGE_SIZE);
        }
    }
    frame_number
}

pub fn page_free(frame_number: usize) {
    deallocate_frame(frame_number);
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

