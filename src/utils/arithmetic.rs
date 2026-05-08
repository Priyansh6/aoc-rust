#[must_use]
pub const fn num_digits(n: u64) -> u32 {
    n.ilog10() + 1
}
