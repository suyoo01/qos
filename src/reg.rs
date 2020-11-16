//! CP15 registers


/// System control register
pub struct Slctr<T> {
    _phantom: core::marker::PhantomData<T>,
}
impl<T> Slctr<T> where 
T: From<u32> + Into<u32> {
    #[inline(always)]
    pub fn read() -> T{
        let ret: u32;
        unsafe {
            asm!(
                "MRC p15, 0, {ret}, c1, c0, 0",
                ret = out(reg) ret
            );
        }
        ret.into()
    }
    #[inline(always)]
    pub fn write(val: T) {
        let val: u32 = val.into();
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
    pub fn read() -> u32 {
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
    pub fn write(val: u32) {
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
    pub fn read() -> u32 {
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
    pub fn write(val: u32) {
        unsafe {
            asm!(
                "MCR p15, 0, {val}, c0, c0, 5",
                val = in(reg) val
            );
        }
    }
}
