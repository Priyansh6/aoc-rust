use itertools::Itertools;

use aoc_lib::solution::Solution;
use aoc_lib::parser::{Parser, StrParser};
use aoc_lib::range::Range;
use aoc_lib::{arithmetic, parser};

type IdType = u64;

fn can_split_into_n_sized_equal_chunks(id: IdType, n: u32) -> bool {
    let base = 10u64.pow(n);
    std::iter::successors(Some(id), |&id_quotient| {
        let next = id_quotient / base;
        (next > 0).then_some(next)
    })
    .map(|n| n % base)
    .all_equal()
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Vec<Range<IdType>>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        parser::from_str::<Range<IdType>>.split(",")
    }

    fn part1(&self, ranges: &Self::Parsed) -> String {
        let mut result = 0;
        for range in ranges {
            result += range
                .iter()
                .filter(|&id| {
                    let digits = arithmetic::num_digits(id);
                    if digits % 2 == 1 {
                        return false;
                    }
                    can_split_into_n_sized_equal_chunks(id, digits / 2)
                })
                .sum::<IdType>();
        }
        result.to_string()
    }

    fn part2(&self, ranges: &Self::Parsed) -> String {
        let mut result = 0;
        for range in ranges {
            result += range
                .iter()
                .filter(|&id| {
                    let digits = arithmetic::num_digits(id);
                    (1..=digits / 2)
                        .filter(|&size| digits.is_multiple_of(size))
                        .any(|size| can_split_into_n_sized_equal_chunks(id, size))
                })
                .sum::<IdType>();
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::solution::{check_part1, check_part2};

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "1227775554");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "4174379265");
    }
}
