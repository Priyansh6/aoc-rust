#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::manual_let_else)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::fs;
use std::time::Instant;

use aoc_lib::solutions::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, Solution,
};
use aoc_lib::utils::parser::Parser;

macro_rules! run_day {
    ($day:expr, $input:expr, $($num:literal => $sol:expr),+ $(,)?) => {
        match $day {
            $($num => {
                let s = $sol;
                let start = Instant::now();
                let parsed = s.parser().parse($input).expect("Parse failed");
                println!("Parse: {:?}", start.elapsed());

                let start = Instant::now();
                let result = s.part1(&parsed);
                println!("Part 1: {} ({:?})", result, start.elapsed());

                let start = Instant::now();
                let result = s.part2(&parsed);
                println!("Part 2: {} ({:?})", result, start.elapsed());
            })+
            _ => panic!("Day {} not implemented", $day),
        }
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: u8 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);

    let input =
        fs::read_to_string(format!("inputs/day{day:02}.txt")).expect("Input file not found");
    let input = input.trim_end_matches('\n');

    run_day!(&day, &input,
        1 => day01::Sol,
        2 => day02::Sol,
        3 => day03::Sol,
        4 => day04::Sol,
        5 => day05::Sol,
        6 => day06::Sol,
        7 => day07::Sol,
        8 => day08::Sol::<{ day08::NUM_CONNECTIONS_PART_1 }>,
        9 => day09::Sol,
        10 => day10::Sol,
    );
}
