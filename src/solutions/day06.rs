use itertools::Itertools;

use crate::solutions::Solution;
use crate::utils::parser;
use crate::utils::parser::{CharParser, Parser, StrParser};
use crate::{char_match, utils};

pub enum Operator {
    Add,
    Multiply,
}

fn calculate_sum(operators: &[Operator], num_groups: &[Vec<u64>]) -> u64 {
    operators
        .iter()
        .zip_eq(num_groups)
        .map(|(op, nums)| match op {
            Operator::Add => nums.iter().sum::<u64>(),
            Operator::Multiply => nums.iter().product::<u64>(),
        })
        .sum()
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = (Vec<String>, Vec<Operator>);

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        let parse_operator = char_match! {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
        };
        let num_grid_lines_parser = parser::as_string.lines();
        let operators_parser = parse_operator.single_char().split_whitespace();
        parser::rsplit_once(num_grid_lines_parser, operators_parser, "\n")
    }

    fn part1(&self, (num_grid_lines, operators): &Self::Parsed) -> String {
        let num_grid = parser::from_str::<u64>
            .split_whitespace()
            .into_each()
            .parse(num_grid_lines.iter().map(String::as_str))
            .unwrap();
        let num_groups = utils::transpose(num_grid);
        calculate_sum(operators, &num_groups).to_string()
    }

    fn part2(&self, (num_grid_lines, operators): &Self::Parsed) -> String {
        let char_grid = parser::identity
            .chars()
            .into_each()
            .parse(num_grid_lines.iter().map(String::as_str))
            .unwrap();
        // Convert grid to column-major format as characters
        let col_major_char_grid: Vec<Vec<char>> = utils::transpose(char_grid);

        // Group columns by empty spaces to form number groups
        let num_groups: Vec<Vec<u64>> = col_major_char_grid
            .iter()
            .map(|col| col.iter().collect::<String>().trim().to_string())
            .batching(|it| {
                // Take consecutive non-empty strings as one group
                let nums = it
                    .take_while(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect_vec();
                (!nums.is_empty()).then_some(nums)
            })
            .collect();

        calculate_sum(operators, &num_groups).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::{check_part1, check_part2};

    const TEST_INPUT: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  "
    );

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "4277556");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "3263827");
    }
}
