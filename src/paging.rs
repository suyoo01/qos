use crate::{println, print};
use crate::mem;

pub const KERN_BASE: usize = 0xc0000000;
pub const UTOP: usize = 0xc0000000;
pub const USTACK: usize = 0x40000000;
pub const PAGE_SIZE: usize = 4096;
pub const NUM_PAGES: usize = ((1<<30) / PAGE_SIZE) << 2;
/// Section size: 1MB
pub const SECTION_SIZE: usize = 1<<20;

const DIRECTORY_MASK: usize = !((1<<20) - 1);
const TABLE_ADDR_MASK: usize = !((1<<10) -1);
const L2_ADDR_MASK: usize = !((1<<12)-1);

const SECTION_MASK: usize = 0x2;
const TABLE_MASK: usize = 0x1;

pub const USER_FLAG:usize = 1<<5;

#[repr(C)]
pub struct L1PageTable {
    pub entries: [L1TableEntry; 4096],
}

#[repr(C)]
pub struct L1TableEntry {
    pub data: usize
}

impl L1TableEntry {
    fn is_present(&self) -> bool {
        self.is_section() || self.is_table()
    }

    fn is_section(&self) -> bool {
        self.data & SECTION_MASK != 0
    }

    fn is_table(&self) -> bool {
        self.data & TABLE_MASK != 0
    }

    fn get_l2_table(&self) -> &mut L2PageTable {
        unsafe {
            &mut *((self.data & TABLE_ADDR_MASK) as *mut L2PageTable)
        }
    }

}

#[repr(C)]
pub struct L2PageTable {
    pub entries: [L2TableEntry; 256],
}

pub struct L2TableEntry {
    data: usize
}

pub fn get_page_table() -> &'static mut L1PageTable {
    let kern_pgdir_addr = crate::reg::TTBR0::read() as usize;

    let kern_pgdir_addr = kern_pgdir_addr | KERN_BASE;


    let kern_pgdir;
    unsafe {
        kern_pgdir = &mut *(kern_pgdir_addr as *mut L1PageTable);
    }
    kern_pgdir
}

/// Map [3G, 4G-1M) => [0, 1G-1M]
pub unsafe fn page_init() {
    let kern_pgdir = get_page_table();
    
    let entries = &mut kern_pgdir.entries;

    let mut i = 0;
    let offset = KERN_BASE / (1024*1024); 
    while i < 1023 {
        entries[i+offset].data = (i << 20) | 0x1280e;
        i += 1;
    }
}
pub use mem::Vaddr;

fn directory_index(va: Vaddr) -> usize{
    va.addr / SECTION_SIZE
}

use crate::mem::{alloc_frame, fn_to_pa, pa_to_va};


/// For normal memory only, use map_va_to_device otherwise.
pub fn map_va_to_fn(va: usize, frame_number: usize, flag: usize) {
    let page_table = get_page_table();
    let directory = &mut page_table.entries[va/SECTION_SIZE];
    let pa = frame_number * PAGE_SIZE;
    if directory.is_section() {
        panic!("Can't remap section");
    } else if directory.is_table() {
    } else {
        let data = fn_to_pa(alloc_frame(1, 1)) | 0x1;
        *directory = L1TableEntry{ data};
    }
    let l2_table = directory.get_l2_table();
    l2_table.entries[(va&!DIRECTORY_MASK)/PAGE_SIZE] = L2TableEntry { data: L2_ADDR_MASK&pa | flag | 0x89e};
}


/// Map va to pa (device address) set memory attribute sharable device
pub fn map_va_to_device(va: usize, pa: usize, flag: usize) {
    let page_table = get_page_table();
    let directory = &mut page_table.entries[va/SECTION_SIZE];
    
    if directory.is_section() {
        panic!("Can't remap section");
    } else if directory.is_table() {

    } else {
        *directory = L1TableEntry{ data: fn_to_pa(alloc_frame(1, 1)) | 0x1};
    }
    let l2_table = directory.get_l2_table();
    l2_table.entries[(pa&!DIRECTORY_MASK)/PAGE_SIZE] = L2TableEntry { data: L2_ADDR_MASK&pa | flag | 0x813};
}

/// return physical address of old pgdir
pub unsafe fn change_pgdir(addr: usize) -> usize {
    let old_pgdir = mem::va_to_pa(get_page_table() as *const _ as usize);
    crate::reg::TTBR0::write(addr);
    old_pgdir
}

pub fn list_pgdir(pgdir: &L1PageTable) {
    for (i, entry) in pgdir.entries.iter().enumerate() {
        if entry.is_present() {
            println!("{} {:8x}",i, entry.data);
        }
    }
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

