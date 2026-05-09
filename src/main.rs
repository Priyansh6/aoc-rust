#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::manual_let_else)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::fs;
use std::time::Instant;

use aoc_lib::solutions::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, Solution,
};
use aoc_lib::utils::parser::Parser;

const NUM_DAYS: i32 = 12;

macro_rules! run_day {
    ($day:expr, $input:expr, $($num:literal => $sol:expr),+ $(,)?) => {
        match $day {
            $($num => {
                println!("============== Day {} ==============", $num);
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

fn run_day(day: i32) {
    let input = fs::read_to_string(format!("inputs/day{day:02}.txt"))
        .unwrap_or_else(|_| panic!("Input file not found for day {day}"));
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
        11 => day11::Sol,
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day = args.get(1).map_or("all", String::as_str);

    if day == "all" {
        for d in 1..=NUM_DAYS {
            run_day(d);
        }
    } else {
        let day = day.parse().expect("Invalid day number");
        run_day(day);
    }
}
