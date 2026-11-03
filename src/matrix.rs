use std::fmt::{self, Display, Formatter};

use crate::errors::MatrixError;
use crate::functions::Lerp;
use crate::scalar::Scalar;
use crate::utils::zero_tolerance_sq;
use crate::vector::Vector;

/// A `rows` x `cols` matrix.
///
/// `Matrix::from([[a, b], [c, d]])` reads each inner array as a **row**, and
/// `Display` prints one row per line, matching the subject's examples.
/// Internally the coefficients are stored in column-major order.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

impl<K: Scalar> Matrix<K> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self::from_column_major(vec![K::zero(); rows * cols], rows, cols)
    }

    pub fn identity(size: usize) -> Self {
        let mut out = Self::zeros(size, size);
        for idx in 0..size {
            out.set(idx, idx, K::one());
        }
        out
    }

    /// Builds a matrix from coefficients listed column by column.
    pub fn from_column_major(data: Vec<K>, rows: usize, cols: usize) -> Self {
        assert_eq!(data.len(), rows * cols, "data length must be rows * cols");
        Self { data, rows, cols }
    }

    /// Reshapes into a vector, reading the coefficients column by column.
    pub fn to_vector(&self) -> Vector<K> {
        Vector::new(self.data.clone())
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    fn index_of(&self, row: usize, col: usize) -> usize {
        assert!(row < self.rows && col < self.cols, "index out of bounds");
        col * self.rows + row
    }

    pub fn get(&self, row: usize, col: usize) -> K {
        self.data[self.index_of(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, value: K) {
        let idx = self.index_of(row, col);
        self.data[idx] = value;
    }

    fn swap_rows(&mut self, lhs: usize, rhs: usize) {
        if lhs == rhs {
            return;
        }
        for col in 0..self.cols {
            let left_idx = self.index_of(lhs, col);
            let right_idx = self.index_of(rhs, col);
            self.data.swap(left_idx, right_idx);
        }
    }

    /// Squared threshold below which a coefficient of this matrix counts as zero.
    fn zero_tolerance_sq(&self) -> f32 {
        let max_modulus_sq = self
            .data
            .iter()
            .fold(0.0f32, |best, value| best.max(value.modulus_sq()));
        zero_tolerance_sq(self.rows.max(self.cols), max_modulus_sq)
    }

    /// Flushes rounding noise (relative to this matrix's own scale) to exact zeros.
    fn flush_noise(&mut self) {
        let tolerance_sq = self.zero_tolerance_sq();
        for value in &mut self.data {
            if value.modulus_sq() <= tolerance_sq {
                *value = K::zero();
            }
        }
    }

    /// Partial pivoting: the row at or below `from_row` with the largest
    /// coefficient in `col`, or `None` if they are all zero within tolerance.
    fn find_pivot(&self, from_row: usize, col: usize, tolerance_sq: f32) -> Option<usize> {
        let mut best_row = from_row;
        let mut best_value = self.get(from_row, col).modulus_sq();
        for row in (from_row + 1)..self.rows {
            let candidate = self.get(row, col).modulus_sq();
            if candidate > best_value {
                best_value = candidate;
                best_row = row;
            }
        }
        (best_value > tolerance_sq).then_some(best_row)
    }

    /// `row -= factor * pivot_row`, from column `from_col` onwards.
    fn eliminate(&mut self, row: usize, pivot_row: usize, factor: K, from_col: usize) {
        for col in from_col..self.cols {
            let value = (-factor).mul_add(self.get(pivot_row, col), self.get(row, col));
            self.set(row, col, value);
        }
    }

    /// Gauss-Jordan elimination: returns the reduced row-echelon form and the rank.
    fn reduce(&self) -> (Self, usize) {
        let mut out = self.clone();
        let tolerance_sq = self.zero_tolerance_sq();
        let mut pivot_row = 0;

        for lead_col in 0..out.cols {
            if pivot_row == out.rows {
                break;
            }
            let Some(best_row) = out.find_pivot(pivot_row, lead_col, tolerance_sq) else {
                continue;
            };
            out.swap_rows(pivot_row, best_row);

            // Columns left of `lead_col` are already zero in this row.
            let pivot = out.get(pivot_row, lead_col);
            for col in (lead_col + 1)..out.cols {
                out.set(pivot_row, col, out.get(pivot_row, col) / pivot);
            }
            // Exact by definition; avoids leftovers like `1.0+5e-9i` for complex pivots.
            out.set(pivot_row, lead_col, K::one());

            for row in 0..out.rows {
                let factor = out.get(row, lead_col);
                if row != pivot_row && factor != K::zero() {
                    out.eliminate(row, pivot_row, factor, lead_col + 1);
                    out.set(row, lead_col, K::zero());
                }
            }
            pivot_row += 1;
        }

        out.flush_noise();
        (out, pivot_row)
    }

    // ex00
    pub fn add(&mut self, v: &Matrix<K>) {
        assert_eq!(self.shape(), v.shape(), "matrix dimensions must match");
        for (lhs, &rhs) in self.data.iter_mut().zip(&v.data) {
            *lhs += rhs;
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        assert_eq!(self.shape(), v.shape(), "matrix dimensions must match");
        for (lhs, &rhs) in self.data.iter_mut().zip(&v.data) {
            *lhs -= rhs;
        }
    }

    pub fn scl(&mut self, a: K) {
        for value in &mut self.data {
            *value *= a;
        }
    }

    // ex07
    /// `A·u`, O(nm) time and space.
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        assert_eq!(
            self.cols,
            vec.size(),
            "incompatible matrix/vector dimensions"
        );
        let mut out = Vector::zeros(self.rows);
        for row in 0..self.rows {
            out[row] = (0..self.cols).fold(K::zero(), |acc, col| {
                self.get(row, col).mul_add(vec[col], acc)
            });
        }
        out
    }

    /// `A·B`, O(nmp) time, O(nm + mp + np) space.
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        assert_eq!(self.cols, mat.rows, "incompatible matrix dimensions");
        let mut out = Matrix::zeros(self.rows, mat.cols);
        for col in 0..mat.cols {
            for row in 0..self.rows {
                let value = (0..self.cols).fold(K::zero(), |acc, k| {
                    self.get(row, k).mul_add(mat.get(k, col), acc)
                });
                out.set(row, col, value);
            }
        }
        out
    }

    // ex08
    pub fn trace(&self) -> K {
        assert!(self.is_square(), "trace is defined on square matrices only");
        (0..self.rows).fold(K::zero(), |acc, idx| acc + self.get(idx, idx))
    }

    // ex09
    pub fn transpose(&self) -> Matrix<K> {
        let mut out = Matrix::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.set(col, row, self.get(row, col));
            }
        }
        out
    }

    // ex10
    /// Reduced row-echelon form, O(n³).
    pub fn row_echelon(&self) -> Matrix<K> {
        self.reduce().0
    }

    // ex11
    /// Determinant by Gaussian elimination with partial pivoting, O(n³).
    /// Works for any size; the subject only requires n <= 4.
    pub fn determinant(&self) -> K {
        assert!(
            self.is_square(),
            "determinant is defined on square matrices only"
        );
        let size = self.rows;
        let tolerance_sq = self.zero_tolerance_sq();
        let mut out = self.clone();
        let mut det = K::one();

        for pivot_col in 0..size {
            let Some(best_row) = out.find_pivot(pivot_col, pivot_col, tolerance_sq) else {
                return K::zero();
            };
            if best_row != pivot_col {
                out.swap_rows(best_row, pivot_col);
                det = -det;
            }

            let pivot = out.get(pivot_col, pivot_col);
            det *= pivot;

            for row in (pivot_col + 1)..size {
                let factor = out.get(row, pivot_col) / pivot;
                out.eliminate(row, pivot_col, factor, pivot_col);
            }
        }
        det
    }

    // ex12
    /// Inverse by Gauss-Jordan elimination on `[A | I]`, O(n³).
    pub fn inverse(&self) -> Result<Matrix<K>, MatrixError> {
        assert!(
            self.is_square(),
            "inverse is defined on square matrices only"
        );
        let size = self.rows;
        let tolerance_sq = self.zero_tolerance_sq();
        let mut left = self.clone();
        let mut right = Matrix::identity(size);

        for pivot_col in 0..size {
            let best_row = left
                .find_pivot(pivot_col, pivot_col, tolerance_sq)
                .ok_or(MatrixError::SingularMatrix)?;
            left.swap_rows(pivot_col, best_row);
            right.swap_rows(pivot_col, best_row);

            let pivot = left.get(pivot_col, pivot_col);
            for col in 0..size {
                left.set(pivot_col, col, left.get(pivot_col, col) / pivot);
                right.set(pivot_col, col, right.get(pivot_col, col) / pivot);
            }

            for row in 0..size {
                let factor = left.get(row, pivot_col);
                if row != pivot_col && factor != K::zero() {
                    left.eliminate(row, pivot_col, factor, pivot_col);
                    right.eliminate(row, pivot_col, factor, 0);
                }
            }
        }

        right.flush_noise();
        Ok(right)
    }

    // ex13
    /// Rank: the number of pivots found while reducing to row-echelon form, O(n³).
    pub fn rank(&self) -> usize {
        self.reduce().1
    }
}

impl<K: Scalar> Lerp for Matrix<K> {
    fn lerp_to(self, other: Self, t: f32) -> Self {
        assert_eq!(self.shape(), other.shape(), "matrix dimensions must match");
        let Matrix { data, rows, cols } = self;
        let data = data
            .into_iter()
            .zip(other.data)
            .map(|(u, v)| u.lerp_to(v, t))
            .collect();
        Self { data, rows, cols }
    }
}

/// Each inner array is one row.
impl<K: Scalar, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(value: [[K; C]; R]) -> Self {
        let mut out = Self::zeros(R, C);
        for (row, values) in value.iter().enumerate() {
            for (col, &coefficient) in values.iter().enumerate() {
                out.set(row, col, coefficient);
            }
        }
        out
    }
}

/// One row per line.
impl<K: Scalar> Display for Matrix<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            if row != 0 {
                writeln!(f)?;
            }
            write!(f, "[")?;
            for col in 0..self.cols {
                if col != 0 {
                    write!(f, ", ")?;
                }
                self.get(row, col).fmt_scalar(f)?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}
