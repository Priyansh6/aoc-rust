pub fn transpose<T>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if rows.is_empty() {
        return Vec::new();
    }
    let num_cols = rows[0].len();
    let mut row_iters: Vec<_> = rows.into_iter().map(IntoIterator::into_iter).collect();

    (0..num_cols)
        .map(|_| row_iters.iter_mut().filter_map(Iterator::next).collect())
        .collect()
}

pub struct GaussianEliminationGF2Result {
    pub reduced_matrix: Vec<Vec<bool>>,
    pub pivot_cols: Vec<usize>,
    pub free_cols: Vec<usize>,
}

/// Performs Gaussian elimination over GF(2) on an augmented matrix where each row
/// represents an equation: the first `n` elements are coefficients and the last
/// element is the right-hand side.
///
/// Returns a [`GaussianEliminationResult`] containing the reduced matrix, the indices
/// of pivot columns (determined variables), and the indices of free columns (free variables).
#[must_use]
pub fn gaussian_elimination_gf2(mut matrix: Vec<Vec<bool>>) -> GaussianEliminationGF2Result {
    let num_rows = matrix.len();
    if num_rows == 0 {
        return GaussianEliminationGF2Result {
            reduced_matrix: matrix,
            pivot_cols: vec![],
            free_cols: vec![],
        };
    }
    let num_cols = matrix[0].len() - 1; // exclude augmented column

    let mut pivot_cols = vec![];
    let mut free_cols = vec![];
    let mut row_i = 0;

    for col_i in 0..num_cols {
        // find a pivot row for this column
        let pivot_row = (row_i..num_rows).find(|&r| matrix[r][col_i]);
        let Some(pivot_row) = pivot_row else {
            free_cols.push(col_i); // no pivot, this is a free variable
            continue;
        };

        matrix.swap(row_i, pivot_row);
        pivot_cols.push(col_i);

        // eliminate all other rows with a 1 in this column
        let pivot = matrix[row_i].clone();
        matrix
            .iter_mut()
            .enumerate()
            .filter(|(r, row)| *r != row_i && row[col_i])
            .for_each(|(_, row)| row.iter_mut().zip(&pivot).for_each(|(a, b)| *a ^= b));

        row_i += 1;
    }

    GaussianEliminationGF2Result {
        reduced_matrix: matrix,
        pivot_cols,
        free_cols,
    }
}
