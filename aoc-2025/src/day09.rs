#![allow(clippy::cast_precision_loss)]

use std::cmp;
use std::cmp::PartialEq;

use itertools::Itertools;

use aoc_lib::solution::Solution;
use aoc_lib::geometry::Point2;
use aoc_lib::parser::{self, Parser, StrParser};
use aoc_lib::range::Range;

struct Wall {
    orientation: Orientation,
    anchor: u64,
    span: Range<u64>,
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

type RedTile = Point2<u64>;

fn tiles_to_wall(tiles: (&RedTile, &RedTile)) -> Wall {
    let (t1, t2) = tiles;
    if t1.x() == t2.x() {
        Wall {
            orientation: Orientation::Vertical,
            anchor: t1.x(),
            span: Range::between(t1.y(), t2.y()),
        }
    } else {
        Wall {
            orientation: Orientation::Horizontal,
            anchor: t1.y(),
            span: Range::between(t1.x(), t2.x()),
        }
    }
}

fn rectangle_has_no_cuts(
    t1: RedTile,
    t2: RedTile,
    horizontal_walls: &[Wall],
    vertical_walls: &[Wall],
) -> bool {
    let x_range = Range::between(t1.x(), t2.x());
    let y_range = Range::between(t1.y(), t2.y());

    let horizontal_cut = horizontal_walls.iter().any(|wall| {
        y_range.contains_exclusive(&wall.anchor) && wall.span.overlaps_strictly(&x_range)
    });

    let vertical_cut = vertical_walls.iter().any(|wall| {
        x_range.contains_exclusive(&wall.anchor) && wall.span.overlaps_strictly(&y_range)
    });

    !horizontal_cut && !vertical_cut
}

fn is_in_loop(x: f64, y: f64, horizontal_walls: &[Wall]) -> bool {
    horizontal_walls
        .iter()
        .filter(|wall| {
            let span = Range::new(*wall.span.start() as f64, *wall.span.end() as f64).unwrap();
            span.contains(&x) && (wall.anchor as f64) < y
        })
        .count()
        % 2
        == 1
}

pub struct Sol;

impl Solution for Sol {
    type Parsed = Vec<RedTile>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        parser::from_str::<u64>
            .split_array(",")
            .map(RedTile::new)
            .lines()
    }

    fn part1(&self, tiles: &Self::Parsed) -> String {
        tiles
            .iter()
            .tuple_combinations()
            .map(|(t1, t2)| t1.inclusive_rect_area(t2))
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self, tiles: &Self::Parsed) -> String {
        let (horizontal_walls, vertical_walls): (Vec<_>, _) = tiles
            .iter()
            .circular_tuple_windows()
            .map(tiles_to_wall)
            .partition(|wall| wall.orientation == Orientation::Horizontal);

        let mut max_area = 0;
        for (t1, t2) in tiles.iter().tuple_combinations() {
            if t1.x() == t2.x() {
                max_area = cmp::max(max_area, t1.y().abs_diff(t2.y()));
                continue;
            }
            if t1.y() == t2.y() {
                max_area = cmp::max(max_area, t1.x().abs_diff(t2.x()));
                continue;
            }
            if !rectangle_has_no_cuts(*t1, *t2, &horizontal_walls, &vertical_walls) {
                continue;
            }
            let mid_point = (
                (t1.x() + t2.x()) as f64 / 2.0,
                (t1.y() + t2.y()) as f64 / 2.0,
            );
            if is_in_loop(mid_point.0, mid_point.1, &horizontal_walls) {
                max_area = cmp::max(max_area, t1.inclusive_rect_area(t2));
            }
        }
        max_area.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::solution::{check_part1, check_part2};

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        check_part1(&Sol, TEST_INPUT, "50");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol, TEST_INPUT, "24");
    }
}
