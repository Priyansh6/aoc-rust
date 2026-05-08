use crate::solutions::Solution;
use crate::utils::parser;
use crate::utils::parser::{Parser, StrParser};
use crate::utils::range::Range;

type IdType = u64;

pub struct Sol;

impl Solution for Sol {
    type Parsed = (Vec<Range<IdType>>, Vec<IdType>);

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        let range_parser = parser::from_str::<Range<IdType>>.lines();
        let id_parser = parser::from_str::<IdType>.lines();
        parser::split_pair(range_parser, id_parser, "\n\n")
    }

    fn part1(&self, (ranges, ids): &Self::Parsed) -> String {
        let mut fresh_ids = 0;
        for id in ids {
            for range in ranges {
                if range.contains(id) {
                    fresh_ids += 1;
                    break;
                }
            }
        }
        fresh_ids.to_string()
    }

    fn part2(&self, (ranges, _): &Self::Parsed) -> String {
        let mut ranges = ranges.clone();
        ranges.sort_by_key(|range| *range.start());

        let mut merged_ranges: Vec<Range<IdType>> = Vec::new();

        for range in ranges {
            if let Some(last) = merged_ranges.last_mut()
                && last.overlaps(&range)
            {
                last.merge(range);
            } else {
                merged_ranges.push(range);
            }
        }

        merged_ranges
            .iter()
            .map(Range::num_elems)
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::{check_part1, check_part2};

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "3");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "14");
    }
}
