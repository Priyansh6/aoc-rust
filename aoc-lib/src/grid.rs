#![allow(dead_code)]

use std::fmt;
use std::fmt::{Debug, Formatter};
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
    pub fn new(rows: usize, cols: usize, value: T) -> Self
    where
        T: Clone,
    {
        assert!(rows > 0, "Grid cannot have height 0");
        assert!(cols > 0, "Grid cannot have width 0");
        Self {
            cells: vec![vec![value; cols]; rows],
            height: rows,
            width: cols,
        }
    }

    pub fn from_rows<I, J>(rows: I) -> Result<Self, ParseError>
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

    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Grid<U> {
        Grid {
            cells: self
                .cells
                .iter()
                .map(|row| row.iter().map(&f).collect())
                .collect(),
            height: self.height,
            width: self.width,
        }
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
    pub const fn below_n(&self, pos: &GridPosition, n: usize) -> Option<GridPosition> {
        let new_y = pos.y + n;
        if new_y >= self.height {
            return None;
        }
        Some(GridPosition { x: pos.x, y: new_y })
    }

    #[must_use]
    pub const fn above_n(&self, pos: &GridPosition, n: usize) -> Option<GridPosition> {
        if pos.y < n {
            return None;
        }
        Some(GridPosition {
            x: pos.x,
            y: pos.y - n,
        })
    }

    #[must_use]
    pub const fn right_n(&self, pos: &GridPosition, n: usize) -> Option<GridPosition> {
        let new_x = pos.x + n;
        if new_x >= self.width {
            return None;
        }
        Some(GridPosition { x: new_x, y: pos.y })
    }

    #[must_use]
    pub const fn left_n(&self, pos: &GridPosition, n: usize) -> Option<GridPosition> {
        if pos.x < n {
            return None;
        }
        Some(GridPosition {
            x: pos.x - n,
            y: pos.y,
        })
    }

    #[must_use]
    pub const fn below(&self, pos: &GridPosition) -> Option<GridPosition> {
        self.below_n(pos, 1)
    }

    #[must_use]
    pub const fn above(&self, pos: &GridPosition) -> Option<GridPosition> {
        self.above_n(pos, 1)
    }

    #[must_use]
    pub const fn right(&self, pos: &GridPosition) -> Option<GridPosition> {
        self.right_n(pos, 1)
    }

    #[must_use]
    pub const fn left(&self, pos: &GridPosition) -> Option<GridPosition> {
        self.left_n(pos, 1)
    }

    #[must_use]
    pub const fn next(&self, pos: &GridPosition) -> Option<GridPosition> {
        if let Some(p) = self.right(pos) {
            Some(p)
        } else if pos.y + 1 < self.height {
            Some(GridPosition { x: 0, y: pos.y + 1 })
        } else {
            None
        }
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = (GridPosition, &T)> {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, cell)| (GridPosition { x, y }, cell))
        })
    }

    pub fn iter_enumerated_mut(&mut self) -> impl Iterator<Item = (GridPosition, &mut T)> {
        self.cells.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
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

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Collect all values as strings first so we can measure column widths
        let cells: Vec<Vec<String>> = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| format!("{:?}", self[GridPosition { x, y }]))
                    .collect()
            })
            .collect();

        let col_widths: Vec<usize> = (0..self.width)
            .map(|x| cells.iter().map(|row| row[x].len()).max().unwrap_or(0))
            .collect();

        let row_sep = format!(
            "+{}+",
            col_widths
                .iter()
                .map(|w| "-".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("+")
        );

        writeln!(f)?;
        writeln!(f, "{row_sep}")?;
        for row in &cells {
            write!(f, "|")?;
            for (val, w) in row.iter().zip(&col_widths) {
                write!(f, " {val:>w$} |")?;
            }
            writeln!(f)?;
            writeln!(f, "{row_sep}")?;
        }
        Ok(())
    }
}
