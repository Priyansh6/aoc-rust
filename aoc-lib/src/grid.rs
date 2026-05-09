#![allow(dead_code)]

use std::ops::{Index, IndexMut};

use crate::parser::{CharParser, ParseError, Parser, StrParser};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn from_rows<I, J>(rows: I) -> Result<Self, ParseError>
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = T>,
    {
        let cells: Vec<Vec<T>> = rows
            .into_iter()
            .map(|row| row.into_iter().collect())
            .collect();

        let height = cells.len();
        if height == 0 {
            return Err("Grid cannot have height 0".to_string().into());
        }
        let width = cells.first().unwrap().len();
        if width == 0 {
            return Err("Grid cannot have width 0".to_string().into());
        }

        Ok(Self {
            cells,
            height,
            width,
        })
    }

    pub fn parser(
        cell: impl CharParser<Output = T>,
    ) -> impl for<'a> Parser<&'a str, Output = Self> {
        cell.chars().lines().and_then(Self::from_rows)
    }

    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub const fn below(&self, pos: &GridPosition) -> Option<GridPosition> {
        let new_y = pos.y + 1;
        if new_y >= self.height {
            return None;
        }
        Some(GridPosition { x: pos.x, y: new_y })
    }

    #[must_use]
    pub const fn above(&self, pos: &GridPosition) -> Option<GridPosition> {
        if pos.y == 0 {
            return None;
        }
        Some(GridPosition {
            x: pos.x,
            y: pos.y - 1,
        })
    }

    #[must_use]
    pub const fn right(&self, pos: &GridPosition) -> Option<GridPosition> {
        let new_x = pos.x + 1;
        if new_x >= self.width {
            return None;
        }
        Some(GridPosition { x: new_x, y: pos.y })
    }

    #[must_use]
    pub const fn left(&self, pos: &GridPosition) -> Option<GridPosition> {
        if pos.x == 0 {
            return None;
        }
        Some(GridPosition {
            x: pos.x - 1,
            y: pos.y,
        })
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = (GridPosition, &T)> {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, cell)| (GridPosition { x, y }, cell))
        })
    }

    pub fn surrounding_cells(&self, pos: GridPosition) -> impl Iterator<Item = &T> {
        const OFFSETS: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        OFFSETS.iter().filter_map(move |&(r_off, c_off)| {
            let new_x = pos.x.checked_add_signed(c_off)?;
            let new_y = pos.y.checked_add_signed(r_off)?;

            if new_x < self.width && new_y < self.height {
                Some(&self.cells[new_y][new_x])
            } else {
                None
            }
        })
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, elem: &T) -> Option<GridPosition> {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if elem == cell {
                    return Some(GridPosition { x, y });
                }
            }
        }
        None
    }
}

impl<T> Index<GridPosition> for Grid<T> {
    type Output = T;

    fn index(&self, pos: GridPosition) -> &Self::Output {
        &self.cells[pos.y][pos.x]
    }
}

impl<T> IndexMut<GridPosition> for Grid<T> {
    fn index_mut(&mut self, pos: GridPosition) -> &mut Self::Output {
        &mut self.cells[pos.y][pos.x]
    }
}
