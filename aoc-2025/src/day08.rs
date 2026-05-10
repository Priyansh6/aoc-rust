use aoc_lib::geometry::{self, Point3};
use aoc_lib::parser;
use aoc_lib::parser::{Parser, StrParser};
use aoc_lib::solution::Solution;
use aoc_lib::union_find::UnionFind;
use itertools::Itertools;

pub const NUM_CONNECTIONS_PART_1: usize = 1000;

pub struct Sol<const NUM_CONNECTIONS: usize>;

impl<const NUM_CONNECTIONS: usize> Solution for Sol<NUM_CONNECTIONS> {
    type Parsed = Vec<Point3<f64>>;

    fn parser(&self) -> impl Parser<&str, Output = Self::Parsed> {
        parser::from_str::<Point3<f64>>.lines()
    }

    fn part1(&self, points: &Self::Parsed) -> String {
        let pairs = geometry::k_closest_pair_indices(points, NUM_CONNECTIONS);
        let mut union_find = UnionFind::new(points.len());
        for (left, right) in pairs {
            union_find.union(left, right);
        }

        (0..points.len())
            .map(|i| union_find.find(i))
            .unique()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|root| union_find.get_size(root))
            .k_largest(3)
            .product::<usize>()
            .to_string()
    }

    fn part2(&self, points: &Self::Parsed) -> String {
        let pairs = geometry::closest_pair_indices(points);
        let mut union_find = UnionFind::new(points.len());

        for (left, right) in pairs {
            union_find.union(left, right);
            if union_find.get_size(left) == points.len() {
                return (points[left].x() * points[right].x()).to_string();
            }
        }
        "COULD NOT FIND ANSWER".to_string()
    }
}

#[cfg(test)]
mod tests {
    use aoc_lib::solution::{check_part1, check_part2};

    use super::*;

    const TEST_NUM_CONNECTIONS_PART_1: usize = 10;
    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        check_part1(&Sol::<TEST_NUM_CONNECTIONS_PART_1>, TEST_INPUT, "40");
    }

    #[test]
    fn test_part2() {
        check_part2(&Sol::<TEST_NUM_CONNECTIONS_PART_1>, TEST_INPUT, "25272");
    }
}
