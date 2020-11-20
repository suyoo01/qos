//! CP15 registers


/// System control register
pub struct Slctr;
impl Slctr {
    #[inline(always)]
    pub fn read() -> usize {
        let ret: usize;
        unsafe {
            asm!(
                "MRC p15, 0, {ret}, c1, c0, 0",
                ret = out(reg) ret
            );
        }
        ret
    }
    #[inline(always)]
    pub fn write(val: usize) {
        unsafe {
            asm!(
                "MCR p15, {val}, c1, c0, 0",
                val = in(reg) val
            );
        }
    }
}

pub struct TTBR0;
impl TTBR0 {
    #[inline(always)]
    pub fn read() -> usize {
        let mut val;
        unsafe {
            asm!(
                "MRC p15, 0, {val}, c2, c0, 0",
                val = out(reg) val
            );
        }
        val
    }
    #[inline(always)]
    pub fn write(val: usize) {
        unsafe {
            asm!(
                "MCR p15, 0, {val}, c2, c0, 0",
                val = in(reg) val
            );
        }
    }
}

/// Multiprocessor Affinity Register
pub struct MPIDR;
impl MPIDR {
    #[inline(always)]
    pub fn read() -> usize {
        let mut val;
        unsafe {
            asm!(
                "MRC p15, 0, {val}, c0, c0, 5",
                val = out(reg) val
            ); 
        }
        val
    }
    #[inline(always)]
    pub fn write(val: usize) {
        unsafe {
            asm!(
                "MCR p15, 0, {val}, c0, c0, 5",
                val = in(reg) val
            );
        }
    }
}
