#![allow(dead_code)]

use std::cmp;
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use crate::parser;
use crate::parser::{ParseError, Parser, StrParser};

#[derive(Debug, Clone, Copy)]
pub struct Range<T> {
    start: T,
    end: T,
}

#[derive(Debug)]
pub struct RangeError;

impl Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Range start must be <= end")
    }
}

impl Error for RangeError {}

impl<T> Range<T> {
    pub const fn start(&self) -> &T {
        &self.start
    }

    pub const fn end(&self) -> &T {
        &self.end
    }
}

impl<T: PartialOrd> Range<T> {
    pub fn new(start: T, end: T) -> Result<Self, RangeError> {
        if start <= end {
            Ok(Self { start, end })
        } else {
            Err(RangeError)
        }
    }

    pub fn between(a: T, b: T) -> Self {
        if a.partial_cmp(&b).unwrap() == Ordering::Greater {
            Self { start: b, end: a }
        } else {
            Self { start: a, end: b }
        }
    }

    pub fn contains(&self, x: &T) -> bool {
        &self.start <= x && x <= &self.end
    }

    pub fn contains_exclusive(&self, x: &T) -> bool {
        &self.start < x && x < &self.end
    }

    pub fn overlaps(&self, range: &Self) -> bool {
        self.start <= range.end && range.start <= self.end
    }

    pub fn overlaps_strictly(&self, range: &Self) -> bool {
        self.start < range.end && range.start < self.end
    }
}

impl<T: Ord + Copy> Range<T> {
    pub fn merge(&mut self, range: Self) {
        self.start = cmp::min(self.start, range.start);
        self.end = cmp::max(self.end, range.end);
    }

    #[must_use]
    pub fn merged_with(mut self, range: Self) -> Self {
        self.start = cmp::min(self.start, range.start);
        self.end = cmp::max(self.end, range.end);
        self
    }
}

impl Range<u64> {
    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.start..=self.end
    }

    #[must_use]
    pub const fn num_elems(&self) -> u64 {
        self.end - self.start + 1
    }
}

impl<T> FromStr for Range<T>
where
    T: PartialOrd + FromStr,
    T::Err: Display,
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::from_str
            .split_array("-")
            .and_then(|[start, end]| {
                Self::new(start, end).map_err(|err| ParseError::Other(err.to_string()))
            })
            .parse(s)
    }
}
