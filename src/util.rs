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
