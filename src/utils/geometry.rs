#![allow(dead_code)]

use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;

use crate::utils::parser;
use crate::utils::parser::{ParseError, Parser, StrParser};

#[derive(Clone, Copy, Debug)]
pub struct Vector<T, const N: usize> {
    vals: [T; N],
}
pub type Vector3<T> = Vector<T, 3>;
pub type Point<T, const N: usize> = Vector<T, N>;
pub type Point2<T> = Vector<T, 2>;
pub type Point3<T> = Vector<T, 3>;

impl<T, const N: usize> Vector<T, N> {
    pub const fn new(vals: [T; N]) -> Self {
        Self { vals }
    }
}

impl<T, const N: usize> FromStr for Vector<T, N>
where
    T: FromStr,
    T::Err: Display,
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::from_str.split_array(",").map(Self::new).parse(s)
    }
}

impl<T, const N: usize> std::ops::Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vals[index]
    }
}

macro_rules! impl_accessors {
    ($n:literal, $($method:ident => $index:literal),+ $(,)?) => {
        impl<T: Copy> Vector<T, $n> {
            $(
                pub const fn $method(&self) -> T {
                    self.vals[$index]
                }
            )+
        }
    };
}

impl_accessors!(1, x => 0);
impl_accessors!(2, x => 0, y => 1);
impl_accessors!(3, x => 0, y => 1, z => 2);

impl<const N: usize> Point<f64, N> {
    fn distance_from(&self, other: &Self) -> f64 {
        self.vals
            .iter()
            .zip_eq(other.vals.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

impl Point2<u32> {
    #[must_use]
    pub const fn inclusive_rect_area(&self, other: &Self) -> u32 {
        (self.x().abs_diff(other.x()) + 1) * (self.y().abs_diff(other.y()) + 1)
    }
}

impl Point2<u64> {
    #[must_use]
    pub const fn inclusive_rect_area(&self, other: &Self) -> u64 {
        (self.x().abs_diff(other.x()) + 1) * (self.y().abs_diff(other.y()) + 1)
    }
}

pub fn k_closest_pair_indices<const N: usize>(
    points: &[Point<f64, N>],
    k: usize,
) -> impl Iterator<Item = (usize, usize)> {
    points
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((l_i, l_point), (r_i, r_point))| ((l_i, r_i), l_point.distance_from(r_point)))
        .k_smallest_relaxed_by(k, |(_, dist1), (_, dist2)| dist1.total_cmp(dist2))
        .map(|(pair, _)| pair)
}

pub fn closest_pair_indices<const N: usize>(
    points: &[Point<f64, N>],
) -> impl Iterator<Item = (usize, usize)> {
    points
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((l_i, l_point), (r_i, r_point))| ((l_i, r_i), l_point.distance_from(r_point)))
        .sorted_by(|(_, dist1), (_, dist2)| dist1.total_cmp(dist2))
        .map(|(pair, _)| pair)
}
