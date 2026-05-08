use std::collections::{HashMap, HashSet};

use crate::char_match;
use crate::solutions::Solution;
use crate::utils::grid::{Grid, GridPosition};
use crate::utils::parser::Parser;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Square {
    Blank,
    Source,
    Splitter,
    Beam,
}

fn insert_beam(beams: &mut HashMap<GridPosition, u64>, pos: GridPosition, possibilities: u64) {
    beams
        .entry(pos)
        .and_modify(|p| *p += possibilities)
        .or_insert(possibilities);
}

fn move_beam_down_part1(
    manifold: &Grid<Square>,
    beam_pos: GridPosition,
    next_beam_pos: &mut HashSet<GridPosition>,
) -> usize {
    let below_pos = manifold.below(&beam_pos).unwrap();

    match manifold[below_pos] {
        Square::Blank => {
            next_beam_pos.insert(below_pos);
            0
        }
        Square::Splitter => {
            if let Some(left_pos) = manifold.left(&below_pos) {
                next_beam_pos.insert(left_pos);
            }
            if let Some(right_pos) = manifold.right(&below_pos) {
                next_beam_pos.insert(right_pos);
            }
            1
        }
        Square::Beam => panic!("There should not be beams on the grid"),
        Square::Source => panic!("There should only be one source on the grid"),
    }
}

fn move_beam_down_part2(
    manifold: &Grid<Square>,
    beam_pos: GridPosition,
    possibilities: u64,
    next_beam_possibilities: &mut HashMap<GridPosition, u64>,
) {
    let below_pos = manifold.below(&beam_pos).unwrap();

    match manifold[below_pos] {
        Square::Blank => {
            insert_beam(next_beam_possibilities, below_pos, possibilities);
        }
        Square::Splitter => {
            if let Some(left_pos) = manifold.left(&below_pos) {
                insert_beam(next_beam_possibilities, left_pos, possibilities);
            }
            if let Some(right_pos) = manifold.right(&below_pos) {
                insert_beam(next_beam_possibilities, right_pos, possibilities);
            }
        }
        Square::Beam => panic!("There should not be beams on the grid"),
        Square::Source => panic!("There should only be one source on the grid"),
    }
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Grid<Square>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        let parse_square = char_match! {
            '.' => Square::Blank,
            'S' => Square::Source,
            '^' => Square::Splitter,
            '|' => Square::Beam,
        };
        Grid::parser(parse_square)
    }

    fn part1(&self, manifold: &Self::Parsed) -> String {
        let source_pos = manifold.find(&Square::Source).unwrap();

        let mut beam_pos = HashSet::from([source_pos]);
        let mut collisions = 0;
        for _ in 0..(manifold.height() - 1) {
            let mut next_beam_pos = HashSet::new();
            for pos in beam_pos {
                collisions += move_beam_down_part1(manifold, pos, &mut next_beam_pos);
            }
            beam_pos = next_beam_pos;
        }
        collisions.to_string()
    }

    fn part2(&self, manifold: &Self::Parsed) -> String {
        let source_pos = manifold.find(&Square::Source).unwrap();

        let mut beam_possibilities = HashMap::from([(source_pos, 1)]);
        for _ in 0..(manifold.height() - 1) {
            let mut next_beam_possibilities = HashMap::new();
            for (pos, possibilities) in beam_possibilities {
                move_beam_down_part2(manifold, pos, possibilities, &mut next_beam_possibilities);
            }
            beam_possibilities = next_beam_possibilities;
        }
        beam_possibilities.values().sum::<u64>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::{check_part1, check_part2};

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "21");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "40");
    }
}
