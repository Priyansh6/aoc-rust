use aoc_lib::char_match;
use aoc_lib::grid::{Grid, GridPosition};
use aoc_lib::parser::Parser;
use aoc_lib::solution::Solution;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Square {
    Blank,
    Paper,
}

fn get_accessible_paper_positions(grid: &Grid<Square>) -> impl Iterator<Item = GridPosition> {
    grid.iter_enumerated().filter_map(|(pos, square)| {
        let is_accessible = *square == Square::Paper
            && grid
                .surrounding_cells(pos)
                .filter(|&cell| *cell == Square::Paper)
                .count()
                < 4;

        is_accessible.then_some(pos)
    })
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Grid<Square>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        let parse_square = char_match! {
            '.' => Square::Blank,
            '@' => Square::Paper,
        };
        Grid::parser(parse_square)
    }

    fn part1(&self, grid: &Self::Parsed) -> String {
        get_accessible_paper_positions(grid).count().to_string()
    }

    fn part2(&self, grid: &Self::Parsed) -> String {
        let mut grid: Grid<Square> = grid.clone();
        let mut total_accessible_squares = 0;
        while let accessible_square_positions =
            get_accessible_paper_positions(&grid).collect::<Vec<GridPosition>>()
            && !accessible_square_positions.is_empty()
        {
            for &pos in &accessible_square_positions {
                grid[pos] = Square::Blank;
            }
            total_accessible_squares += accessible_square_positions.len();
        }
        total_accessible_squares.to_string()
    }
}

#[cfg(test)]
mod tests {
    use aoc_lib::solution::{check_part1, check_part2};

    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "13");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "43");
    }
}
