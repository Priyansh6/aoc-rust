pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

use crate::utils::parser::Parser;

pub trait Solution {
    type Parsed;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed>;

    fn part1(&self, parsed: &Self::Parsed) -> String;
    fn part2(&self, parsed: &Self::Parsed) -> String;
}

#[cfg(test)]
pub fn check_part1<S: Solution>(sol: &S, input: &str, expected: &str) {
    let parsed = sol.parser().parse(input).unwrap();
    assert_eq!(sol.part1(&parsed), expected);
}

#[cfg(test)]
pub fn check_part2<S: Solution>(sol: &S, input: &str, expected: &str) {
    let parsed = sol.parser().parse(input).unwrap();
    assert_eq!(sol.part2(&parsed), expected);
}
