#![allow(clippy::cast_possible_truncation)]

use aoc_lib::grid::{Grid, GridPosition};
use aoc_lib::parser::{CharParser, Parser, StrParser};
use aoc_lib::solution::Solution;
use aoc_lib::{char_match, parser};
use itertools::Itertools;

pub const SHAPE_LENGTH: u32 = 3;
pub const SHAPE_LENGTH_USIZE: usize = SHAPE_LENGTH as usize;

struct ShapePermutation {
    mask: u32,
    first_block_offset: usize,
}

/// Every shape is encoded with a special mask for good performance when shifting in any direction.
/// E.g. for `SHAPE_LENGTH = 3`, the mask is `0b00_111_00_111_00_111` where underscores are for
/// illustration purposes, separating the "padding" unnecessary bits from the used bits
const fn grid_cell_mask() -> u32 {
    let row_width = 2 * SHAPE_LENGTH_USIZE - 1;
    let row_bits = (1 << SHAPE_LENGTH_USIZE) - 1;
    let mut mask = 0u32;
    let mut row = 0;
    while row < SHAPE_LENGTH_USIZE {
        mask |= row_bits << (row * row_width);
        row += 1;
    }
    mask
}

const fn shift_left(mask: u32, n: u32) -> u32 {
    (mask << n) & grid_cell_mask()
}
const fn shift_up(mask: u32, n: u32) -> u32 {
    mask << (n * (2 * SHAPE_LENGTH - 1)) & grid_cell_mask()
}

const fn shift_right(mask: u32, n: u32) -> u32 {
    (mask >> n) & grid_cell_mask()
}
const fn shift_down(mask: u32, n: u32) -> u32 {
    mask >> (n * (2 * SHAPE_LENGTH - 1))
}

const TOP_LEFT_BIT: u32 =
    1 << ((SHAPE_LENGTH_USIZE - 1) * (2 * SHAPE_LENGTH_USIZE - 1) + (SHAPE_LENGTH_USIZE - 1));

/// Rotates the shape 90° clockwise.
fn rotate(shape: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let n = shape.len();
    (0..n)
        .map(|col| (0..n).rev().map(|row| shape[row][col]).collect())
        .collect()
}

/// Flips the shape horizontally (reflects about the vertical axis).
fn flip(shape: &[Vec<bool>]) -> Vec<Vec<bool>> {
    shape
        .iter()
        .map(|row| row.iter().rev().copied().collect())
        .collect()
}

fn construct_shape_mask(shape: &[Vec<bool>]) -> u32 {
    let row_width = 2 * SHAPE_LENGTH_USIZE - 1;
    let mut mask: u32 = 0;
    for (row, cols) in shape.iter().enumerate() {
        let mut row_mask: u32 = 0;
        for (col, cell) in cols.iter().enumerate() {
            if *cell {
                row_mask |= 1 << (SHAPE_LENGTH_USIZE - 1 - col);
            }
        }
        mask |= row_mask << ((SHAPE_LENGTH_USIZE - 1 - row) * row_width);
    }
    mask
}

/// Returns the column index of the first filled cell in the top row of the shape.
fn find_first_block_offset(shape: &[Vec<bool>]) -> usize {
    shape[0].iter().position(|&val| val).unwrap()
}

fn shape_permutations(shape: &[Vec<bool>]) -> Vec<ShapePermutation> {
    let r0 = shape.to_vec();
    let r1 = rotate(&r0);
    let r2 = rotate(&r1);
    let r3 = rotate(&r2);
    let f0 = flip(&r0);
    let f1 = flip(&r1);
    let f2 = flip(&r2);
    let f3 = flip(&r3);
    let mut out = [r0, r1, r2, r3, f0, f1, f2, f3]
        .iter()
        .map(|p| ShapePermutation {
            mask: construct_shape_mask(p),
            first_block_offset: find_first_block_offset(p),
        })
        .collect_vec();
    out.sort_unstable_by_key(|permutation| permutation.mask);
    out.dedup_by_key(|permutation| permutation.mask);
    out
}

fn construct_shape_masks(shapes: &[Vec<Vec<bool>>]) -> Vec<Vec<ShapePermutation>> {
    shapes
        .iter()
        .map(|shape| shape_permutations(shape))
        .collect()
}

fn toggle_shape(mask_grid: &mut Grid<u32>, pos: GridPosition, mask: u32) {
    let l = SHAPE_LENGTH_USIZE.cast_signed();
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
                shift_left(mask, dc.unsigned_abs() as u32)
            } else {
                shift_right(mask, dc.unsigned_abs() as u32)
            };
            let shifted = if dr >= 0 {
                shift_up(shifted_h, dr.unsigned_abs() as u32)
            } else {
                shift_down(shifted_h, dr.unsigned_abs() as u32)
            };

            mask_grid[GridPosition { x, y }] ^= shifted;
        }
    }
}

fn is_occupied(mask_grid: &Grid<u32>, pos: GridPosition) -> bool {
    mask_grid[pos] & TOP_LEFT_BIT != 0
}

fn can_fit_shapes(
    mask_grid: &mut Grid<u32>,
    shape_masks: &[Vec<ShapePermutation>],
    shape_counts: &mut [u32],
) -> bool {
    fn go(
        mask_grid: &mut Grid<u32>,
        shape_masks: &[Vec<ShapePermutation>],
        shape_counts: &mut [u32],
        remaining_shapes: u32,
        prev_anchor: Option<GridPosition>,
        waste_budget: u32,
    ) -> bool {
        if remaining_shapes == 0 {
            return true;
        }
        let Some(mut anchor) = prev_anchor else {
            return false;
        };
        while is_occupied(mask_grid, anchor) {
            match mask_grid.next(&anchor) {
                Some(a) => anchor = a,
                None => return false,
            }
        }

        for (i, permutations) in shape_masks.iter().enumerate() {
            if shape_counts[i] == 0 {
                continue;
            }
            for permutation in permutations {
                let Some(shape_top_left_pos) =
                    mask_grid.left_n(&anchor, permutation.first_block_offset)
                else {
                    continue;
                };
                // We check if the shape can fit in the grid
                if mask_grid[shape_top_left_pos] & permutation.mask != 0 {
                    continue;
                }

                shape_counts[i] -= 1;
                toggle_shape(mask_grid, shape_top_left_pos, permutation.mask);
                if go(
                    mask_grid,
                    shape_masks,
                    shape_counts,
                    remaining_shapes - 1,
                    mask_grid.next(&anchor),
                    waste_budget,
                ) {
                    return true;
                }
                toggle_shape(mask_grid, shape_top_left_pos, permutation.mask);
                shape_counts[i] += 1;
            }
        }

        // Skip branch: leave the anchor cell empty, only if we can still afford it.
        if waste_budget == 0 {
            return false;
        }
        go(
            mask_grid,
            shape_masks,
            shape_counts,
            remaining_shapes,
            mask_grid.next(&anchor),
            waste_budget - 1,
        )
    }

    let total_shapes: u32 = shape_counts.iter().sum();

    // All orientations of a shape have the same count of cells occupied, so permutations[0] is
    // representative.
    let total_area: u32 = shape_masks
        .iter()
        .zip(shape_counts.iter())
        .map(|(permutations, &count)| count * permutations[0].mask.count_ones())
        .sum();
    let grid_cells = (mask_grid.width() * mask_grid.height()) as u32;

    if total_area > grid_cells {
        return false;
    }
    let waste_budget = grid_cells - total_area;

    go(
        mask_grid,
        shape_masks,
        shape_counts,
        total_shapes,
        Some(GridPosition { x: 0, y: 0 }),
        waste_budget,
    )
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
                .map(|((), shape)| shape);
        let dim_parser =
            parser::split_pair(parser::from_str::<usize>, parser::from_str::<usize>, "x")
                .wrapped("", ":");
        let region_parser =
            parser::lsplit_once(dim_parser, parser::from_str::<u32>.split_whitespace(), " ");
        parser::rsplit_once(shape_parser.split("\n\n"), region_parser.lines(), "\n\n")
    }

    /// NOTE: The below solution only works if `2 * SHAPE_LENGTH ^ 2 <= 32`.
    /// It only works for square shapes with at least one block full in each wall.
    /// It uses a recursive algorithm, ideally there are less than a thousand total shapes to place.
    fn part1(&self, (shapes, regions): &Self::Parsed) -> String {
        let mut num_valid = 0;
        let shape_masks = construct_shape_masks(shapes);

        for ((cols, rows), shape_counts) in regions {
            let mut mask_grid: Grid<u32> = Grid::from_rows((0..*rows).map(|y| {
                (0..*cols).map(move |x| {
                    let right_overflow = (x + SHAPE_LENGTH_USIZE).saturating_sub(*cols) as u32;
                    let down_overflow = (y + SHAPE_LENGTH_USIZE).saturating_sub(*rows) as u32;
                    (grid_cell_mask() ^ shift_left(grid_cell_mask(), right_overflow))
                        | (grid_cell_mask() ^ shift_up(grid_cell_mask(), down_overflow))
                })
            }))
            .unwrap();

            if can_fit_shapes(&mut mask_grid, &shape_masks, &mut shape_counts.clone()) {
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
        use std::time::Instant;

        let start = Instant::now();
        check_part1(&Sol, TEST_INPUT, "2");
        let elapsed = start.elapsed();

        println!("test_part1 took {elapsed:?}");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "2");
    }
}
