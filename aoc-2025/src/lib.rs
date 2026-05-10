use aoc_lib::solution::run_solution;
use aoc_lib::year::AOCYear;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

pub struct AOCYear2025;

impl AOCYear for AOCYear2025 {
    fn num_days(&self) -> u8 {
        12
    }

    fn run_day(&self, day: u8, input: &str) {
        match day {
            1 => run_solution(1, input, &day01::Sol),
            2 => run_solution(2, input, &day02::Sol),
            3 => run_solution(3, input, &day03::Sol),
            4 => run_solution(4, input, &day04::Sol),
            5 => run_solution(5, input, &day05::Sol),
            6 => run_solution(6, input, &day06::Sol),
            7 => run_solution(7, input, &day07::Sol),
            8 => run_solution(8, input, &day08::Sol::<{ day08::NUM_CONNECTIONS_PART_1 }>),
            9 => run_solution(9, input, &day09::Sol),
            10 => run_solution(10, input, &day10::Sol),
            11 => run_solution(11, input, &day11::Sol),
            12 => run_solution(12, input, &day12::Sol),
            _ => eprintln!("Day {day} is not implemented"),
        }
    }
}
