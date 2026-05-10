use std::cmp;

use aoc_lib::parser::{CharParser, Parser, StrParser};
use aoc_lib::solution::Solution;
use aoc_lib::{arithmetic, parser};

fn propagate_max_and_set_next_to_zero(slice: &mut [u32], val: u32) {
    let len = slice.len();
    for i in 0..len - 1 {
        if val > slice[i] {
            slice[i] = val;
            slice[i + 1] = 0;
            return;
        }
    }
    slice[len - 1] = cmp::max(val, slice[len - 1]);
}

fn sum_of_largest_joltages(digit_lines: &Vec<Vec<u32>>, num_batteries: usize) -> u64 {
    let mut sum: u64 = 0;
    for digits in digit_lines {
        let mut result_digits = vec![0; num_batteries];
        for (i, &digit) in digits.iter().enumerate() {
            propagate_max_and_set_next_to_zero(
                &mut result_digits[num_batteries.saturating_sub(digits.len() - i)..],
                digit,
            );
        }
        sum += arithmetic::digits_to_num(result_digits.as_slice());
    }
    sum
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Vec<Vec<u32>>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        parser::digit::<10>.chars().lines()
    }

    fn part1(&self, digit_lines: &Self::Parsed) -> String {
        sum_of_largest_joltages(digit_lines, 2).to_string()
    }

    fn part2(&self, digit_lines: &Self::Parsed) -> String {
        sum_of_largest_joltages(digit_lines, 12).to_string()
    }
}

#[cfg(test)]
mod tests {
    use aoc_lib::solution::{check_part1, check_part2};

    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "357");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "3121910778619");
    }

    #[test]
    fn test_part1_single_line() {
        check_part1(&Sol, "987654321111111", "98");
    }

    #[test]
    fn test_part2_single_line() {
        check_part2(&Sol, "987654321111111", "987654321111");
    }
}
