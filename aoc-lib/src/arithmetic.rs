#[must_use]
pub const fn num_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

#[must_use]
pub fn digits_to_num(digits: &[u32]) -> u64 {
    let mut result: u64 = 0;
    let mut unit: u64 = 1;
    for &digit in digits.iter().rev() {
        result += unit * u64::from(digit);
        unit *= 10;
    }
    result
}
