use std::collections::HashMap;

use aoc_lib::algebra::GaussianEliminationGF2Result;
use aoc_lib::parser::{CharParser, Parser, StrParser, lsplit_once, rsplit_once};
use aoc_lib::solution::Solution;
use aoc_lib::{algebra, char_match, parser};
use itertools::Itertools;

fn expand_schematics(schematics: &[Vec<usize>], num_indicators: usize) -> Vec<Vec<bool>> {
    schematics
        .iter()
        .map(|schematic| {
            let mut schematic_vector = vec![false; num_indicators];
            for &i in schematic {
                schematic_vector[i] = true;
            }
            schematic_vector
        })
        .collect_vec()
}

fn get_possible_presses_for_indicators(
    schematics_bool: &[Vec<bool>],
    indicators: &[bool],
) -> impl Iterator<Item = Vec<usize>> {
    let mut schematics_bool = schematics_bool.to_vec();
    schematics_bool.push(indicators.to_vec());
    let matrix = algebra::transpose(schematics_bool);

    let GaussianEliminationGF2Result {
        reduced_matrix,
        pivot_cols,
        free_cols,
    } = algebra::gaussian_elimination_gf2(matrix);

    let aug_col = reduced_matrix[0].len() - 1;

    itertools::repeat_n([false, true], free_cols.len())
        .multi_cartesian_product()
        .map(move |free_vals| {
            let free_press_cols = free_cols
                .iter()
                .zip_eq(&free_vals)
                .filter(|&(_, &pressed)| pressed)
                .map(|(&free_col, _)| free_col);
            let pivot_press_cols = pivot_cols
                .iter()
                .enumerate()
                .filter(|&(row, _)| {
                    free_cols.iter().enumerate().fold(
                        reduced_matrix[row][aug_col],
                        |val, (free_col_i, &free_col)| {
                            val ^ (free_vals[free_col_i] && reduced_matrix[row][free_col])
                        },
                    )
                })
                .map(|(_, &pivot_col)| pivot_col);
            free_press_cols.chain(pivot_press_cols).collect_vec()
        })
}

fn min_presses_for_joltages(
    joltages: &Vec<i64>,
    schematics_bool: &Vec<Vec<bool>>,
    schematics_idx: &Vec<Vec<usize>>,
) -> u64 {
    let mut cache = HashMap::new();
    cache.insert(vec![0; joltages.len()], 0);
    min_presses_for_joltages_helper(joltages, schematics_bool, schematics_idx, &mut cache)
}

fn min_presses_for_joltages_helper(
    joltages: &Vec<i64>,
    schematics_bool: &Vec<Vec<bool>>,
    schematics_idx: &Vec<Vec<usize>>,
    cache: &mut HashMap<Vec<i64>, u64>,
) -> u64 {
    if let Some(&cached) = cache.get(joltages) {
        return cached;
    }

    let joltage_parity = joltages.iter().map(|&j| j % 2 == 1).collect_vec();
    let mut min_cost = u64::MAX;

    'candidate_loop: for button_indices in
        get_possible_presses_for_indicators(schematics_bool, &joltage_parity)
    {
        let mut residual = joltages.clone();
        let num_presses = button_indices.len() as u64;
        for button_i in button_indices {
            for &counter_i in &schematics_idx[button_i] {
                residual[counter_i] -= 1;
            }
        }
        for joltage in &mut residual {
            if *joltage % 2 != 0 || *joltage < 0 {
                continue 'candidate_loop;
            }
            *joltage /= 2;
        }
        let sub_cost =
            min_presses_for_joltages_helper(&residual, schematics_bool, schematics_idx, cache);
        let cost = sub_cost.saturating_mul(2).saturating_add(num_presses);
        min_cost = min_cost.min(cost);
    }

    cache.insert(joltages.clone(), min_cost);
    min_cost
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Vec<(Vec<bool>, Vec<Vec<usize>>, Vec<i64>)>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        let indicator = char_match! {
            '.' => false,
            '#' => true,
        };
        let indicator_parser = indicator.chars().wrapped("[", "]");
        let schematic_parser = parser::from_str::<usize>
            .split(",")
            .wrapped("(", ")")
            .split_whitespace();
        let requirement_parser = parser::from_str::<i64>.split(",").wrapped("{", "}");
        lsplit_once(
            indicator_parser,
            rsplit_once(schematic_parser, requirement_parser, " "),
            " ",
        )
        .map(|(indicators, (schematics, requirements))| (indicators, schematics, requirements))
        .lines()
    }

    fn part1(&self, machines: &Self::Parsed) -> Option<String> {
        let mut sum_presses = 0;
        for (indicators, schematics, _) in machines {
            let schematics = expand_schematics(schematics, indicators.len());
            sum_presses += get_possible_presses_for_indicators(&schematics, indicators)
                .map(|presses| presses.len())
                .min()
                .unwrap();
        }
        Some(sum_presses.to_string())
    }

    fn part2(&self, machines: &Self::Parsed) -> Option<String> {
        let mut sum_presses = 0;
        for (_, schematics, requirements) in machines {
            let schematics_bool = expand_schematics(schematics, requirements.len());
            sum_presses += min_presses_for_joltages(requirements, &schematics_bool, schematics);
        }
        Some(sum_presses.to_string())
    }
}

#[cfg(test)]
mod tests {
    use aoc_lib::solution::{check_part1, check_part2};

    use super::*;

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "7");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "33");
    }
}
