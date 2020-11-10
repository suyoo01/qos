pub struct Slctr;

impl Slctr {
    #[inline(always)]
    pub fn read() -> u32 {
        let ret: u32;
        unsafe {
            asm!(
                "MRC p15, 0, {ret}, c1, c0, 0",
                ret = out(reg) ret
            );
        }
        ret
    }

    #[inline(always)]
    pub fn write(val: u32) {
        unsafe {
            asm!(
                "MCR p15, {val}, c1, c0, 0",
                val = in(reg) val
            );
        }
    }
}