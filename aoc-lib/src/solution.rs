use std::time::Instant;
use crate::parser::Parser;

pub trait Solution {
    type Parsed;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed>;

    fn part1(&self, parsed: &Self::Parsed) -> String;
    fn part2(&self, parsed: &Self::Parsed) -> String;
}

pub fn run_solution<S: Solution>(day: u8, input: &str, sol: &S) {
    println!("============== Day {day} ==============");

    let start = Instant::now();
    let parsed = sol.parser().parse(input).expect("Parse failed");
    println!("Parse: {:?}", start.elapsed());

    let start = Instant::now();
    println!("Part 1: {} ({:?})", sol.part1(&parsed), start.elapsed());

    let start = Instant::now();
    println!("Part 2: {} ({:?})", sol.part2(&parsed), start.elapsed());
}

pub fn check_part1<S: Solution>(sol: &S, input: &str, expected: &str) {
    let parsed = sol.parser().parse(input).unwrap();
    assert_eq!(sol.part1(&parsed), expected);
}

pub fn check_part2<S: Solution>(sol: &S, input: &str, expected: &str) {
    let parsed = sol.parser().parse(input).unwrap();
    assert_eq!(sol.part2(&parsed), expected);
}
