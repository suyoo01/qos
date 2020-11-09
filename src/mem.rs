extern crate compiler_builtins;
use compiler_builtins::mem::memset;

pub const PGSIZE: usize = 4096;
pub const KERN_BASE: usize = 0xc0000000;


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
// 3G + 1M ------> +------------------------------+
//                 |                              |  
//                 +------------------------------+
//                 |            empty             |  1 MB
// 1M -----------> +------------------------------+

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Vaddr {
    addr: usize
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Paddr {
    addr: usize
}


impl Vaddr {
    fn new(addr: usize) -> Self {
        Vaddr{addr}
    }
    #[inline(always)]
    fn paddr(&self) -> Paddr {
        assert!(self.addr & KERN_BASE !=0);
        Paddr{addr: self.addr ^ KERN_BASE}
    }
}

impl Paddr {
    fn new(addr: usize) -> Self {
        Paddr{addr}
    }
    #[inline(always)]
    fn vaddr(&self) -> Vaddr {
        assert!(self.addr & KERN_BASE == 0);
        Vaddr{addr: self.addr ^ KERN_BASE}
    }
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
    // Init bss
    let bss_start = &_bss_start as *const usize as usize;
    let bss_end = &_bss_end as *const usize as usize;
    let boot_stack = &_bootstack as *const usize as usize;
    let kern_pgdir = &_kern_pgdir as *const usize as usize;
    let kern_pgdir = kern_pgdir | (3<<30);

    memset(bss_start as *mut u8, 0, 
        bss_end - bss_start);
    
    
    /*
    // Memory size TODO: get mem size in runtime
    let mem_size = 1024 * 1024; // memory in KB, 1024 MB = 1 GB */
    let mut next_free = Vaddr::new(boot_stack);

    let driver_addr_table = boot_alloc(&mut next_free, PGSIZE);
    //memset(driver_addr_table.addr as *mut u8, 0, PGSIZE);


    let device_pg_table = boot_alloc(&mut next_free, PGSIZE);
    /*kern_pgdir[4095] = device_pg_table.paddr().addr | PTE_PT;
    let device_pg_table = &mut *(device_pg_table.addr as *mut [usize; 256]);
    device_pg_table[0] = 0xe0001000 | 0x01b;*/

    //*((kern_pgdir + 4095 * 4) as *mut u32) = 0xe0011006;
    *((kern_pgdir + 4095 * 4) as *mut u32) = device_pg_table.paddr().addr as u32 | 1;
    *(device_pg_table.addr as *mut u32) = 0xe0001416;
    //boot_alloc(next_free, mem_size/PGSIZE * )
}

/* ARM Page table entry format (Simplified)
    Section: 1MB
    PA[31:20]   000S    AP[2]TEX[2:0]AP[1]  AP[0](IMP)00  000X    CB10
    Page table
    Table addr[31:10]                            (IMP)00  0000    0001
    Small Page

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
fn boot_alloc(next: &mut Vaddr, size: usize) -> Vaddr{
    let addr = next.addr;
    next.addr = round_up(next.addr + size, PGSIZE);
    Vaddr{addr}
}