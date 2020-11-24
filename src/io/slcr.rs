use volatile_register::{RO, RW};
use crate::paging::map_va_to_device;

const _SLCR_PHYS: usize = 0xf8000000;                  // Physical base address of System Level Control registers (SLCRs)
const _SLCR_VIRT: usize = 0xfff01000;                  // Virtual base address of SLCRs -> SlcrRegs

/// Map a page (4 KB) to SLCRs 
pub fn slcr_init() {
    map_va_to_device(_SLCR_VIRT, _SLCR_PHYS, 0);
}

#[repr(C)]
pub struct SlcrRegs {                                  // Base Address - End Address:    0xFFF01000 - 0xFFF02000
    pub reserved0: [u32; 24],
    pub uart_clk_ctrl: RW<u32>,                        // UART Ref Clock Control:        0x00000154
    pub reserved1: [u32; 33],
    pub uart_rst_ctrl: RW<u32>,                        // UART Software Reset Control:   0x00000228
    pub reserved2: [u32; 103],
}

const SLCR_BASE: *mut SlcrRegs = _SLCR_VIRT as *mut SlcrRegs;

/// Wrapper for SlcrRegs
pub struct SLCR;
impl SLCR {
    fn get(&self) -> &mut SlcrRegs {
        unsafe {
            &mut *SLCR_BASE
        }
    }
}