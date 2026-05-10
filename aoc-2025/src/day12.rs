use std::fmt;
use std::fmt::{Debug, Formatter};

use aoc_lib::grid::{Grid, GridPosition};
use aoc_lib::parser::{CharParser, Parser, StrParser};
use aoc_lib::solution::Solution;
use aoc_lib::{char_match, parser};
use itertools::Itertools;

pub const SHAPE_LENGTH: usize = 3;

struct Bits(u32);

impl Debug for Bits {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const ROW_WIDTH: usize = 2 * SHAPE_LENGTH - 1;
        const PAD_WIDTH: usize = SHAPE_LENGTH - 1;
        const USEFUL_WIDTH: usize = SHAPE_LENGTH;

        // Print high row first (matches MSB-first binary convention)
        for row in (0..SHAPE_LENGTH).rev() {
            if row < SHAPE_LENGTH - 1 {
                write!(f, "_")?;
            }
            let chunk = (self.0 >> (row * ROW_WIDTH)) & ((1 << ROW_WIDTH) - 1);
            let padding = chunk >> USEFUL_WIDTH;
            let useful = chunk & ((1 << USEFUL_WIDTH) - 1);
            write!(
                f,
                "{padding:0p$b}_{useful:0u$b}",
                p = PAD_WIDTH,
                u = USEFUL_WIDTH
            )?;
        }
        Ok(())
    }
}

/// Every shape is encoded with a special mask for good performance when shifting in any direction.
/// E.g. for `SHAPE_LENGTH = 3`, the mask is `0b00_111_00_111_00_111` where underscores are for
/// illustration purposes, separating the "padding" unnecessary bits from the used bits
const fn grid_cell_mask() -> u32 {
    let row_width = 2 * SHAPE_LENGTH - 1;
    let row_bits = (1 << SHAPE_LENGTH) - 1;
    let mut mask = 0u32;
    let mut row = 0;
    while row < SHAPE_LENGTH {
        mask |= row_bits << (row * row_width);
        row += 1;
    }
    mask
}

const fn shift_left(mask: u32, n: u32) -> u32 {
    (mask << n) & grid_cell_mask()
}
const fn shift_up(mask: u32, n: u32) -> u32 {
    mask << (n * (2 * SHAPE_LENGTH as u32 - 1)) & grid_cell_mask()
}

const fn shift_right(mask: u32, n: u32) -> u32 {
    (mask >> n) & grid_cell_mask()
}
const fn shift_down(mask: u32, n: u32) -> u32 {
    mask >> (n * (2 * SHAPE_LENGTH as u32 - 1))
}

const TOP_LEFT_BIT: u32 = 1 << ((SHAPE_LENGTH - 1) * (2 * SHAPE_LENGTH - 1) + (SHAPE_LENGTH - 1));

fn is_occupied(mask_grid: &Grid<u32>, pos: GridPosition) -> bool {
    mask_grid[pos] & TOP_LEFT_BIT != 0
}

fn rotate(shape: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n = shape.len();
    (0..n)
        .map(|col| (0..n).rev().map(|row| shape[row][col]).collect())
        .collect()
}

fn flip(shape: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    shape
        .iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

fn construct_shape_mask(shape: &[Vec<bool>]) -> u32 {
    let row_width = 2 * SHAPE_LENGTH - 1;
    let mut mask: u32 = 0;
    for (row, cols) in shape.iter().enumerate() {
        let mut row_mask: u32 = 0;
        for (col, cell) in cols.iter().enumerate() {
            if *cell {
                row_mask |= 1 << (SHAPE_LENGTH - 1 - col);
            }
        }
        mask |= row_mask << ((SHAPE_LENGTH - 1 - row) * row_width);
    }
    // dbg!(shape);
    // dbg!(Bits(mask));
    mask
}

fn shape_permutations(shape: &[Vec<bool>]) -> Vec<u32> {
    let r0 = shape.to_vec();
    let r1 = rotate(&r0);
    let r2 = rotate(&r1);
    let r3 = rotate(&r2);
    let f0 = flip(&r0);
    let f1 = flip(&r1);
    let f2 = flip(&r2);
    let f3 = flip(&r3);
    let mut masks: Vec<u32> = [r0, r1, r2, r3, f0, f1, f2, f3]
        .iter()
        .map(|perm| construct_shape_mask(perm))
        .collect();
    // dbg!(masks.iter().map(|&mask| Bits(mask)).collect_vec());
    masks.sort_unstable();
    masks.dedup();
    masks
}

fn construct_shape_masks(shapes: &[Vec<Vec<bool>>]) -> Vec<Vec<u32>> {
    shapes
        .iter()
        .map(|shape| shape_permutations(shape))
        .collect()
}

fn toggle_shape(mask_grid: &mut Grid<u32>, pos: GridPosition, mask: u32) {
    let l = SHAPE_LENGTH.cast_signed();
    for dr in -(l - 1)..l {
        for dc in -(l - 1)..l {
            let Some(y) = pos.y.checked_add_signed(dr) else {
                continue;
            };
            let Some(x) = pos.x.checked_add_signed(dc) else {
                continue;
            };
            if y >= mask_grid.height() || x >= mask_grid.width() {
                continue;
            }

            let shifted_h = if dc >= 0 {
                shift_left(mask, dc as u32)
            } else {
                shift_right(mask, (-dc) as u32)
            };
            let shifted = if dr >= 0 {
                shift_up(shifted_h, dr as u32)
            } else {
                shift_down(shifted_h, (-dr) as u32)
            };

            mask_grid[GridPosition { x, y }] ^= shifted;
        }
    }
}

fn can_fit_shape(mask_grid: &Grid<u32>, shape_mask: u32) -> Option<GridPosition> {
    mask_grid
        .iter_enumerated()
        .find(|(_, mask)| shape_mask & *mask == 0)
        .map(|(position, _)| position)
}

fn can_fit_shapes(
    mask_grid: &mut Grid<u32>,
    shape_masks: &[Vec<u32>],
    shape_counts: &mut [u32],
    remaining_shapes: u32,
) -> bool {
    // dbg!(remaining_shapes);
    // dbg!(&mask_grid.map(|&mask| Bits(mask)));
    if remaining_shapes == 0 {
        return true;
    }

    for (i, shape_permutations) in shape_masks.iter().enumerate() {
        // dbg!(i);
        if shape_counts[i] == 0 {
            continue;
        }

        for &permutation in shape_permutations {
            if let Some(position) = can_fit_shape(mask_grid, permutation) {
                shape_counts[i] -= 1;
                toggle_shape(mask_grid, position, permutation);
                if can_fit_shapes(mask_grid, shape_masks, shape_counts, remaining_shapes - 1) {
                    return true;
                }
                toggle_shape(mask_grid, position, permutation);
                shape_counts[i] += 1;
            }
        }
    }

    false
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = (Vec<Vec<Vec<bool>>>, Vec<((usize, usize), Vec<u32>)>);

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        let shape_inclusivity_parser = char_match! {
            '.' => false,
            '#' => true,
        };
        let shape_parser =
            parser::lsplit_once(parser::unit, shape_inclusivity_parser.chars().lines(), "\n")
                .map(|(_, shape)| shape);
        let dim_parser =
            parser::split_pair(parser::from_str::<usize>, parser::from_str::<usize>, "x")
                .wrapped("", ":");
        let region_parser =
            parser::lsplit_once(dim_parser, parser::from_str::<u32>.split_whitespace(), " ");
        parser::rsplit_once(shape_parser.split("\n\n"), region_parser.lines(), "\n\n")
    }

    /// NOTE: The below solution only works if `2 * SHAPE_LENGTH ^ 2 <= 32`.
    /// It only works for square shapes which span the square grid to make implementation simpler.
    /// It uses a recursive algorithm, ideally there are less than a thousand total shapes to place.
    fn part1(&self, (shapes, regions): &Self::Parsed) -> String {
        let mut num_valid = 0;
        let shape_masks = construct_shape_masks(shapes);

        for ((cols, rows), shape_counts) in regions {
            dbg!("lsdkjfl");
            let mut mask_grid: Grid<u32> =
                Grid::new(*rows - SHAPE_LENGTH + 1, *cols - SHAPE_LENGTH + 1, 0);
            let total_shapes = shape_counts.iter().sum();
            if can_fit_shapes(
                &mut mask_grid,
                &shape_masks,
                &mut shape_counts.clone(),
                total_shapes,
            ) {
                num_valid += 1;
            }
        }

        num_valid.to_string()
    }

    fn part2(&self, _devices: &Self::Parsed) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use aoc_lib::solution::{check_part1, check_part2};

    use super::*;

    const TEST_INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "2");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "2");
    }
}
