use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

use crate::complex::Complex;
use crate::errors::MatrixError;
use crate::utils::{abs_f32, is_near_zero, normalize_near_zero};
use crate::vector::Vector;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

impl Matrix<f32> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut out = Self::zeros(size, size);
        for idx in 0..size {
            out.set(idx, idx, 1.0);
        }
        out
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    fn index_of(&self, row: usize, col: usize) -> usize {
        col * self.rows + row
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[self.index_of(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
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

    pub fn add(&mut self, v: &Matrix<f32>) {
        assert_eq!(self.shape(), v.shape(), "matrix dimensions must match");
        for (lhs, rhs) in self.data.iter_mut().zip(v.data.iter()) {
            *lhs += *rhs;
        }
    }

    pub fn sub(&mut self, v: &Matrix<f32>) {
        assert_eq!(self.shape(), v.shape(), "matrix dimensions must match");
        for (lhs, rhs) in self.data.iter_mut().zip(v.data.iter()) {
            *lhs -= *rhs;
        }
    }

    pub fn scl(&mut self, a: f32) {
        for value in &mut self.data {
            *value *= a;
        }
    }

    pub fn mul_vec(&self, vec: &Vector<f32>) -> Vector<f32> {
        assert_eq!(
            self.cols,
            vec.size(),
            "incompatible matrix/vector dimensions"
        );
        let mut out = vec![0.0; self.rows];
        for (row, slot) in out.iter_mut().enumerate() {
            let mut acc = 0.0;
            for col in 0..self.cols {
                acc = self.get(row, col).mul_add(vec[col], acc);
            }
            *slot = acc;
        }
        Vector::from(out)
    }

    pub fn mul_mat(&self, mat: &Matrix<f32>) -> Matrix<f32> {
        assert_eq!(self.cols, mat.rows, "incompatible matrix dimensions");
        let mut out = Matrix::<f32>::zeros(self.rows, mat.cols);
        for col in 0..mat.cols {
            for row in 0..self.rows {
                let mut acc = 0.0;
                for pivot in 0..self.cols {
                    acc = self.get(row, pivot).mul_add(mat.get(pivot, col), acc);
                }
                out.set(row, col, acc);
            }
        }
        out
    }

    pub fn trace(&self) -> f32 {
        assert!(self.is_square(), "trace is defined on square matrices only");
        let mut acc = 0.0;
        for idx in 0..self.rows {
            acc += self.get(idx, idx);
        }
        acc
    }

    pub fn transpose(&self) -> Matrix<f32> {
        let mut out = Matrix::<f32>::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.set(col, row, self.get(row, col));
            }
        }
        out
    }

    pub fn row_echelon(&self) -> Matrix<f32> {
        let mut out = self.clone();
        let mut lead_col = 0usize;
        let mut pivot_row = 0usize;

        while pivot_row < out.rows && lead_col < out.cols {
            let mut best_row = pivot_row;
            let mut best_value = abs_f32(out.get(best_row, lead_col));
            for row in (pivot_row + 1)..out.rows {
                let candidate = abs_f32(out.get(row, lead_col));
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row;
                }
            }
            if is_near_zero(best_value) {
                lead_col += 1;
                continue;
            }

            out.swap_rows(pivot_row, best_row);
            let pivot = out.get(pivot_row, lead_col);
            for col in 0..out.cols {
                out.set(pivot_row, col, out.get(pivot_row, col) / pivot);
            }

            for row in 0..out.rows {
                if row == pivot_row {
                    continue;
                }
                let factor = out.get(row, lead_col);
                if is_near_zero(factor) {
                    continue;
                }
                for col in 0..out.cols {
                    let value = out.get(pivot_row, col).mul_add(-factor, out.get(row, col));
                    out.set(row, col, value);
                }
            }

            pivot_row += 1;
            lead_col += 1;
        }

        for value in &mut out.data {
            *value = normalize_near_zero(*value);
        }
        out
    }

    pub fn determinant(&self) -> f32 {
        assert!(
            self.is_square(),
            "determinant is defined on square matrices only"
        );
        assert!(
            self.rows <= 4,
            "subject only requires determinant up to 4x4"
        );

        let mut out = self.clone();
        let mut sign = 1.0f32;
        let mut det = 1.0f32;

        for pivot_col in 0..out.cols {
            let mut best_row = pivot_col;
            let mut best_value = abs_f32(out.get(best_row, pivot_col));
            for row in (pivot_col + 1)..out.rows {
                let candidate = abs_f32(out.get(row, pivot_col));
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row;
                }
            }
            if is_near_zero(best_value) {
                return 0.0;
            }

            if best_row != pivot_col {
                out.swap_rows(best_row, pivot_col);
                sign = -sign;
            }

            let pivot = out.get(pivot_col, pivot_col);
            det *= pivot;

            for row in (pivot_col + 1)..out.rows {
                let factor = out.get(row, pivot_col) / pivot;
                for col in pivot_col..out.cols {
                    let new_value = out.get(pivot_col, col).mul_add(-factor, out.get(row, col));
                    out.set(row, col, new_value);
                }
            }
        }

        sign * det
    }

    pub fn inverse(&self) -> Result<Matrix<f32>, MatrixError> {
        assert!(
            self.is_square(),
            "inverse is defined on square matrices only"
        );
        let size = self.rows;

        let mut left = self.clone();
        let mut right = Matrix::<f32>::identity(size);

        for pivot_col in 0..size {
            let mut best_row = pivot_col;
            let mut best_value = abs_f32(left.get(best_row, pivot_col));
            for row in (pivot_col + 1)..size {
                let candidate = abs_f32(left.get(row, pivot_col));
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row;
                }
            }
            if is_near_zero(best_value) {
                return Err(MatrixError::SingularMatrix);
            }

            left.swap_rows(best_row, pivot_col);
            right.swap_rows(best_row, pivot_col);

            let pivot = left.get(pivot_col, pivot_col);
            for col in 0..size {
                left.set(pivot_col, col, left.get(pivot_col, col) / pivot);
                right.set(pivot_col, col, right.get(pivot_col, col) / pivot);
            }

            for row in 0..size {
                if row == pivot_col {
                    continue;
                }
                let factor = left.get(row, pivot_col);
                if is_near_zero(factor) {
                    continue;
                }
                for col in 0..size {
                    let left_value = left
                        .get(pivot_col, col)
                        .mul_add(-factor, left.get(row, col));
                    let right_value = right
                        .get(pivot_col, col)
                        .mul_add(-factor, right.get(row, col));
                    left.set(row, col, left_value);
                    right.set(row, col, right_value);
                }
            }
        }

        for value in &mut right.data {
            *value = normalize_near_zero(*value);
        }
        Ok(right)
    }

    pub fn rank(&self) -> usize {
        let reduced = self.row_echelon();
        let mut rank = 0usize;
        for row in 0..reduced.rows {
            let mut non_zero = false;
            for col in 0..reduced.cols {
                if !is_near_zero(reduced.get(row, col)) {
                    non_zero = true;
                    break;
                }
            }
            if non_zero {
                rank += 1;
            }
        }
        rank
    }
}

impl<const M: usize, const N: usize> From<[[f32; M]; N]> for Matrix<f32> {
    fn from(value: [[f32; M]; N]) -> Self {
        let rows = M;
        let cols = N;
        let mut data = Vec::with_capacity(rows * cols);
        for column in value {
            data.extend_from_slice(&column);
        }
        Self { data, rows, cols }
    }
}

impl Display for Matrix<f32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for col in 0..self.cols {
            write!(f, "[")?;
            for row in 0..self.rows {
                write!(f, "{}", self.get(row, col))?;
                if row + 1 != self.rows {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if col + 1 != self.cols {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Add for Matrix<f32> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "matrix dimensions must match");
        let Matrix { data, rows, cols } = self;
        let data = data
            .into_iter()
            .zip(rhs.data)
            .map(|(lhs, rhs)| lhs + rhs)
            .collect();
        Self { data, rows, cols }
    }
}

impl Sub for Matrix<f32> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "matrix dimensions must match");
        let Matrix { data, rows, cols } = self;
        let data = data
            .into_iter()
            .zip(rhs.data)
            .map(|(lhs, rhs)| lhs - rhs)
            .collect();
        Self { data, rows, cols }
    }
}

impl Mul<f32> for Matrix<f32> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let Matrix { data, rows, cols } = self;
        let data = data.into_iter().map(|value| value * rhs).collect();
        Self { data, rows, cols }
    }
}

impl Matrix<Complex> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![Complex::zero(); rows * cols],
            rows,
            cols,
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut out = Self::zeros(size, size);
        for idx in 0..size {
            out.set(idx, idx, Complex::one());
        }
        out
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    fn index_of(&self, row: usize, col: usize) -> usize {
        col * self.rows + row
    }

    pub fn get(&self, row: usize, col: usize) -> Complex {
        self.data[self.index_of(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, value: Complex) {
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

    pub fn add(&mut self, v: &Matrix<Complex>) {
        assert_eq!(self.shape(), v.shape(), "matrix dimensions must match");
        for (lhs, rhs) in self.data.iter_mut().zip(v.data.iter()) {
            *lhs += *rhs;
        }
    }

    pub fn sub(&mut self, v: &Matrix<Complex>) {
        assert_eq!(self.shape(), v.shape(), "matrix dimensions must match");
        for (lhs, rhs) in self.data.iter_mut().zip(v.data.iter()) {
            *lhs -= *rhs;
        }
    }

    pub fn scl(&mut self, a: Complex) {
        for value in &mut self.data {
            *value *= a;
        }
    }

    pub fn mul_vec(&self, vec: &Vector<Complex>) -> Vector<Complex> {
        assert_eq!(
            self.cols,
            vec.size(),
            "incompatible matrix/vector dimensions"
        );
        let mut out = vec![Complex::zero(); self.rows];
        for (row, slot) in out.iter_mut().enumerate() {
            let mut acc = Complex::zero();
            for col in 0..self.cols {
                acc += self.get(row, col) * vec[col];
            }
            *slot = acc;
        }
        Vector::from(out)
    }

    pub fn mul_mat(&self, mat: &Matrix<Complex>) -> Matrix<Complex> {
        assert_eq!(self.cols, mat.rows, "incompatible matrix dimensions");
        let mut out = Matrix::<Complex>::zeros(self.rows, mat.cols);
        for col in 0..mat.cols {
            for row in 0..self.rows {
                let mut acc = Complex::zero();
                for pivot in 0..self.cols {
                    acc += self.get(row, pivot) * mat.get(pivot, col);
                }
                out.set(row, col, acc);
            }
        }
        out
    }

    pub fn trace(&self) -> Complex {
        assert!(self.is_square(), "trace is defined on square matrices only");
        let mut acc = Complex::zero();
        for idx in 0..self.rows {
            acc += self.get(idx, idx);
        }
        acc
    }

    pub fn transpose(&self) -> Matrix<Complex> {
        let mut out = Matrix::<Complex>::zeros(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.set(col, row, self.get(row, col));
            }
        }
        out
    }

    pub fn row_echelon(&self) -> Matrix<Complex> {
        let mut out = self.clone();
        let mut lead_col = 0usize;
        let mut pivot_row = 0usize;

        while pivot_row < out.rows && lead_col < out.cols {
            let mut best_row = pivot_row;
            let mut best_value = out.get(best_row, lead_col).abs_sq();
            for row in (pivot_row + 1)..out.rows {
                let candidate = out.get(row, lead_col).abs_sq();
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row;
                }
            }
            if is_near_zero(best_value) {
                lead_col += 1;
                continue;
            }

            out.swap_rows(pivot_row, best_row);
            let pivot = out.get(pivot_row, lead_col);
            for col in 0..out.cols {
                out.set(pivot_row, col, out.get(pivot_row, col) / pivot);
            }

            for row in 0..out.rows {
                if row == pivot_row {
                    continue;
                }
                let factor = out.get(row, lead_col);
                if factor.is_near_zero() {
                    continue;
                }
                for col in 0..out.cols {
                    let value = out.get(row, col) - factor * out.get(pivot_row, col);
                    out.set(row, col, value);
                }
            }

            pivot_row += 1;
            lead_col += 1;
        }

        for value in &mut out.data {
            *value = value.normalize_near_zero();
        }
        out
    }

    pub fn determinant(&self) -> Complex {
        assert!(
            self.is_square(),
            "determinant is defined on square matrices only"
        );
        assert!(
            self.rows <= 4,
            "subject only requires determinant up to 4x4"
        );

        let mut out = self.clone();
        let mut sign = Complex::one();
        let mut det = Complex::one();

        for pivot_col in 0..out.cols {
            let mut best_row = pivot_col;
            let mut best_value = out.get(best_row, pivot_col).abs_sq();
            for row in (pivot_col + 1)..out.rows {
                let candidate = out.get(row, pivot_col).abs_sq();
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row;
                }
            }
            if is_near_zero(best_value) {
                return Complex::zero();
            }

            if best_row != pivot_col {
                out.swap_rows(best_row, pivot_col);
                sign = sign * Complex::from(-1.0);
            }

            let pivot = out.get(pivot_col, pivot_col);
            det *= pivot;

            for row in (pivot_col + 1)..out.rows {
                let factor = out.get(row, pivot_col) / pivot;
                for col in pivot_col..out.cols {
                    let new_value = out.get(row, col) - factor * out.get(pivot_col, col);
                    out.set(row, col, new_value);
                }
            }
        }

        sign * det
    }

    pub fn inverse(&self) -> Result<Matrix<Complex>, MatrixError> {
        assert!(
            self.is_square(),
            "inverse is defined on square matrices only"
        );
        let size = self.rows;

        let mut left = self.clone();
        let mut right = Matrix::<Complex>::identity(size);

        for pivot_col in 0..size {
            let mut best_row = pivot_col;
            let mut best_value = left.get(best_row, pivot_col).abs_sq();
            for row in (pivot_col + 1)..size {
                let candidate = left.get(row, pivot_col).abs_sq();
                if candidate > best_value {
                    best_value = candidate;
                    best_row = row;
                }
            }
            if is_near_zero(best_value) {
                return Err(MatrixError::SingularMatrix);
            }

            left.swap_rows(best_row, pivot_col);
            right.swap_rows(best_row, pivot_col);

            let pivot = left.get(pivot_col, pivot_col);
            for col in 0..size {
                left.set(pivot_col, col, left.get(pivot_col, col) / pivot);
                right.set(pivot_col, col, right.get(pivot_col, col) / pivot);
            }

            for row in 0..size {
                if row == pivot_col {
                    continue;
                }
                let factor = left.get(row, pivot_col);
                if factor.is_near_zero() {
                    continue;
                }
                for col in 0..size {
                    let left_value = left.get(row, col) - factor * left.get(pivot_col, col);
                    let right_value = right.get(row, col) - factor * right.get(pivot_col, col);
                    left.set(row, col, left_value);
                    right.set(row, col, right_value);
                }
            }
        }

        for value in &mut right.data {
            *value = value.normalize_near_zero();
        }
        Ok(right)
    }

    pub fn rank(&self) -> usize {
        let reduced = self.row_echelon();
        let mut rank = 0usize;
        for row in 0..reduced.rows {
            let mut non_zero = false;
            for col in 0..reduced.cols {
                if !reduced.get(row, col).is_near_zero() {
                    non_zero = true;
                    break;
                }
            }
            if non_zero {
                rank += 1;
            }
        }
        rank
    }
}

impl<const M: usize, const N: usize> From<[[Complex; M]; N]> for Matrix<Complex> {
    fn from(value: [[Complex; M]; N]) -> Self {
        let rows = M;
        let cols = N;
        let mut data = Vec::with_capacity(rows * cols);
        for column in value {
            data.extend_from_slice(&column);
        }
        Self { data, rows, cols }
    }
}

impl Display for Matrix<Complex> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for col in 0..self.cols {
            write!(f, "[")?;
            for row in 0..self.rows {
                write!(f, "{}", self.get(row, col))?;
                if row + 1 != self.rows {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if col + 1 != self.cols {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Add for Matrix<Complex> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "matrix dimensions must match");
        let Matrix { data, rows, cols } = self;
        let data = data
            .into_iter()
            .zip(rhs.data)
            .map(|(lhs, rhs)| lhs + rhs)
            .collect();
        Self { data, rows, cols }
    }
}

impl Sub for Matrix<Complex> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.shape(), rhs.shape(), "matrix dimensions must match");
        let Matrix { data, rows, cols } = self;
        let data = data
            .into_iter()
            .zip(rhs.data)
            .map(|(lhs, rhs)| lhs - rhs)
            .collect();
        Self { data, rows, cols }
    }
}

impl Mul<Complex> for Matrix<Complex> {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        let Matrix { data, rows, cols } = self;
        let data = data.into_iter().map(|value| value * rhs).collect();
        Self { data, rows, cols }
    }
}

impl Mul<f32> for Matrix<Complex> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let Matrix { data, rows, cols } = self;
        let data = data.into_iter().map(|value| value * rhs).collect();
        Self { data, rows, cols }
    }
}
