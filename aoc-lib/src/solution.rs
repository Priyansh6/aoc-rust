use std::time::Instant;

use crate::parser::Parser;

pub trait Solution {
    type Parsed;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed>;

    fn part1(&self, _parsed: &Self::Parsed) -> Option<String> {
        None
    }
    fn part2(&self, _parsed: &Self::Parsed) -> Option<String> {
        None
    }
}

pub fn run_solution<S: Solution>(day: u8, input: &str, sol: &S) {
    println!("============== Day {day} ==============");

    let start = Instant::now();
    let parsed = sol.parser().parse(input).expect("Parse failed");
    println!("Parse: {:?}", start.elapsed());

    let start = Instant::now();
    match sol.part1(&parsed) {
        Some(result) => println!("Part 1: {result} ({:?})", start.elapsed()),
        None => println!("Part 1: not implemented"),
    }

    let start = Instant::now();
    match sol.part2(&parsed) {
        Some(result) => println!("Part 2: {result} ({:?})", start.elapsed()),
        None => println!("Part 2: not implemented"),
    }
}

pub fn check_part1<S: Solution>(sol: &S, input: &str, expected: &str) {
    let parsed = sol.parser().parse(input).unwrap();
    assert_eq!(sol.part1(&parsed).as_deref(), Some(expected));
}

pub fn check_part2<S: Solution>(sol: &S, input: &str, expected: &str) {
    let parsed = sol.parser().parse(input).unwrap();
    assert_eq!(sol.part2(&parsed).as_deref(), Some(expected));
}
