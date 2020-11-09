extern crate compiler_builtins;
use compiler_builtins::mem::memset;

pub const PGSIZE: usize = 4096;
pub const KERN_BASE: usize = 0xc0000000;

use crate::{print, println};

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Vaddr {
    addr: usize,
}

impl Vaddr {
    fn new(addr: usize) -> Self {
        Vaddr { addr }
    }
    #[inline(always)]
    fn paddr(&self) -> Paddr {
        assert!(self.addr & KERN_BASE != 0);
        Paddr {
            addr: self.addr ^ KERN_BASE,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Paddr {
    addr: usize,
}

impl Paddr {
    fn new(addr: usize) -> Self {
        Paddr { addr }
    }
    #[inline(always)]
    fn vaddr(&self) -> Vaddr {
        assert!(self.addr & KERN_BASE == 0);
        Vaddr {
            addr: self.addr ^ KERN_BASE,
        }
    }
}

#[repr(C)]
struct PageTable {
    entries: [u32; 4096],
}

#[inline(always)]
/// Base should be 2**x
pub fn round_up(n: usize, base: usize) -> usize {
    let mask = base - 1;
    n + mask & (!mask)
}

#[inline(always)]
/// Base should be 2**x
pub fn round_down(n: usize, base: usize) -> usize {
    let mask = base - 1;
    n & (!mask)
}

/// Initialize bss and
/// physical frame allocator.
pub unsafe fn mem_init() {
    use core::mem::size_of;
    // Init bss
    let bss_start = &_bss_start as *const usize as usize;
    let bss_end = &_bss_end as *const usize as usize;
    memset(bss_start as *mut u8, 0, bss_end - bss_start);

    let boot_stack = &_bootstack as *const usize as usize;

    let kern_pgdir = &_kern_pgdir as *const usize as usize;
    let kern_pgdir = kern_pgdir | (3 << 30);
    let kern_pgdir = &mut *(kern_pgdir as *mut PageTable);

    for (i, &entry) in kern_pgdir
        .entries
        .iter()
        .enumerate()
        .filter(|(i, &entry)| entry != 0)
    {
        println!("{}, {:x}", i, entry);
    }


    // Memory size
    // TODO: Get mem size in runtime
    let mem_size = 1024 * 1024 as usize; // memory in KB, 1024 MB = 1 GB

    let mut next_free = Vaddr::new(boot_stack);
    let num_pages = mem_size / PGSIZE;
    let pages = boot_alloc(&mut next_free, num_pages);
    memset(pages.addr as *mut u8, 0, num_pages);
}

/* ARM Page table entry format (Simplified)
    Section: 1MB
    PA[31:20]   000S    AP[2]TEX[2:0]AP[1]  AP[0](IMP)00  000X    CB10
    Page table
    Table addr[31:10]                            (IMP)00  0000    0001
    Small Page
    PA[31:12]                               0SAP[2]TEX[2] TEX[1:0]AP[1:0] CB1X


    Sharable device
    TEX = 0 C = 0 B = 1 S = 1
    Normal Outer sharable Write-Back Write-Allocate Cacheable
    TEX = 001 C =1 B = 1 S = 1
*/

/* ARM VMSAv7 MMU access permissions
    AP[2]   AP[1:0]     Kernel      User
    0       00          x           x
    0       01          RW          x
    0       10          RW          RO
    0       11          RW          RW
    1       00    reserved
    1       01          RO          x
    1       10          RO          RO (deprecated)
    1       11          RO          RO

    Simplified model
    AP[0] is always set
    AP[1] is user bit
    AP[2] is RO bit
*/

const SEC_RO: usize = 0x00008800;
const SEC_U: usize = 0x00001800;
const PTE_SEC: usize = 0x2;
const PTE_PT: usize = 0x1;
const PTE_PG: usize = 0x2;

/// Allocator for initial setup
/// allocate static kernel memory
fn boot_alloc(next: &mut Vaddr, size: usize) -> Vaddr {
    let addr = next.addr;
    next.addr = round_up(next.addr + size, PGSIZE);
    Vaddr { addr }
}

fn page_init() {}
