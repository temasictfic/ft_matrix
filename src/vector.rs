use std::fmt::{self, Display, Formatter};
use std::ops::{Index, IndexMut};

use crate::functions::Lerp;
use crate::matrix::Matrix;
use crate::scalar::Scalar;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<K> {
    data: Vec<K>,
}

impl<K: Scalar> Vector<K> {
    pub fn new(data: Vec<K>) -> Self {
        Self { data }
    }

    pub fn zeros(size: usize) -> Self {
        Self::new(vec![K::zero(); size])
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn as_slice(&self) -> &[K] {
        &self.data
    }

    /// Reshapes into a `rows` x `cols` matrix, filling it column by column.
    pub fn to_matrix(&self, rows: usize, cols: usize) -> Matrix<K> {
        Matrix::from_column_major(self.data.clone(), rows, cols)
    }

    // ex00
    pub fn add(&mut self, v: &Vector<K>) {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        for (lhs, &rhs) in self.data.iter_mut().zip(&v.data) {
            *lhs += rhs;
        }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        for (lhs, &rhs) in self.data.iter_mut().zip(&v.data) {
            *lhs -= rhs;
        }
    }

    pub fn scl(&mut self, a: K) {
        for value in &mut self.data {
            *value *= a;
        }
    }

    // ex03
    /// Bilinear dot product `Σ uᵢvᵢ` (the subject's prototype).
    pub fn dot(&self, v: &Vector<K>) -> K {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        self.data
            .iter()
            .zip(&v.data)
            .fold(K::zero(), |acc, (&lhs, &rhs)| lhs.mul_add(rhs, acc))
    }

    /// Inner product `Σ conj(uᵢ)vᵢ`. Same as `dot` over the reals; over the
    /// complex numbers it is the sesquilinear product norms and angles come from.
    pub fn hermitian_dot(&self, v: &Vector<K>) -> K {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        self.data
            .iter()
            .zip(&v.data)
            .fold(K::zero(), |acc, (&lhs, &rhs)| lhs.conj().mul_add(rhs, acc))
    }

    // ex04
    pub fn norm_1(&self) -> f32 {
        self.data.iter().map(|value| value.modulus()).sum()
    }

    pub fn norm(&self) -> f32 {
        self.hermitian_dot(self).real().powf(0.5)
    }

    pub fn norm_inf(&self) -> f32 {
        self.data
            .iter()
            .fold(0.0f32, |best, value| best.max(value.modulus()))
    }
}

impl<K: Scalar> Lerp for Vector<K> {
    fn lerp_to(self, other: Self, t: f32) -> Self {
        assert_eq!(self.size(), other.size(), "vector dimensions must match");
        let data = self
            .data
            .into_iter()
            .zip(other.data)
            .map(|(u, v)| u.lerp_to(v, t))
            .collect();
        Self { data }
    }
}

impl<K: Scalar, const N: usize> From<[K; N]> for Vector<K> {
    fn from(value: [K; N]) -> Self {
        Self::new(value.to_vec())
    }
}

impl<K: Scalar> From<Vec<K>> for Vector<K> {
    fn from(value: Vec<K>) -> Self {
        Self::new(value)
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<K: Scalar> Display for Vector<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (idx, value) in self.data.iter().enumerate() {
            if idx != 0 {
                writeln!(f)?;
            }
            write!(f, "[")?;
            value.fmt_scalar(f)?;
            write!(f, "]")?;
        }
        Ok(())
    }
}
