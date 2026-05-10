use aoc_lib::parser::{Parser, StrParser};
use aoc_lib::solution::Solution;
use aoc_lib::{char_match, parser};

const DIAL_NUMBERS: i32 = 100;
const STARTING_NUMBER: i32 = 50;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

pub struct DialAction {
    direction: Direction,
    distance: i32,
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Vec<DialAction>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        let parse_direction = char_match! {
            'L' => Direction::Left,
            'R' => Direction::Right,
        };
        parser::uncons(parse_direction, parser::from_str::<i32>)
            .map(|(direction, distance)| DialAction {
                direction,
                distance,
            })
            .lines()
    }

    fn part1(&self, actions: &Self::Parsed) -> String {
        let mut result = 0;
        let mut curr = STARTING_NUMBER;
        for action in actions {
            match action.direction {
                Direction::Right => curr += action.distance,
                Direction::Left => curr -= action.distance,
            }
            curr = curr.rem_euclid(DIAL_NUMBERS);

            if curr == 0 {
                result += 1;
            }
        }
        result.to_string()
    }

    fn part2(&self, actions: &Self::Parsed) -> String {
        let mut result = 0;
        let mut curr = STARTING_NUMBER;
        let mut was_zero = false;
        for action in actions {
            match action.direction {
                Direction::Right => curr += action.distance,
                Direction::Left => curr -= action.distance,
            }
            result += curr.div_euclid(DIAL_NUMBERS).abs();
            curr = curr.rem_euclid(DIAL_NUMBERS);

            if action.direction == Direction::Left {
                if was_zero {
                    result -= 1;
                }
                if curr == 0 {
                    result += 1;
                }
            }
            was_zero = curr == 0;
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use aoc_lib::solution::{check_part1, check_part2};

    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "3");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "6");
    }
}
