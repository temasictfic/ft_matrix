use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::complex::Complex;
use crate::utils::abs_f32;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector<K> {
    data: Vec<K>,
}

impl<K> Vector<K> {
    pub fn new(data: Vec<K>) -> Self {
        Self { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn as_slice(&self) -> &[K] {
        &self.data
    }
}

impl<const N: usize> From<[f32; N]> for Vector<f32> {
    fn from(value: [f32; N]) -> Self {
        Self {
            data: value.to_vec(),
        }
    }
}

impl From<Vec<f32>> for Vector<f32> {
    fn from(value: Vec<f32>) -> Self {
        Self { data: value }
    }
}

impl<const N: usize> From<[Complex; N]> for Vector<Complex> {
    fn from(value: [Complex; N]) -> Self {
        Self {
            data: value.to_vec(),
        }
    }
}

impl From<Vec<Complex>> for Vector<Complex> {
    fn from(value: Vec<Complex>) -> Self {
        Self { data: value }
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

impl Display for Vector<f32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, value) in self.data.iter().enumerate() {
            write!(f, "[{}]", value)?;
            if idx + 1 != self.data.len() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Display for Vector<Complex> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, value) in self.data.iter().enumerate() {
            write!(f, "[{}]", value)?;
            if idx + 1 != self.data.len() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Vector<f32> {
    pub fn add(&mut self, v: &Vector<f32>) {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        for (lhs, rhs) in self.data.iter_mut().zip(v.data.iter()) {
            *lhs += *rhs;
        }
    }

    pub fn sub(&mut self, v: &Vector<f32>) {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        for (lhs, rhs) in self.data.iter_mut().zip(v.data.iter()) {
            *lhs -= *rhs;
        }
    }

    pub fn scl(&mut self, a: f32) {
        for value in &mut self.data {
            *value *= a;
        }
    }

    pub fn dot(&self, v: &Vector<f32>) -> f32 {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        let mut acc = 0.0;
        for (&lhs, &rhs) in self.data.iter().zip(v.data.iter()) {
            acc = lhs.mul_add(rhs, acc);
        }
        acc
    }

    pub fn norm_1(&self) -> f32 {
        let mut sum = 0.0;
        for &value in &self.data {
            sum += abs_f32(value);
        }
        sum
    }

    pub fn norm(&self) -> f32 {
        let mut squared = 0.0;
        for &value in &self.data {
            squared = value.mul_add(value, squared);
        }
        squared.powf(0.5)
    }

    pub fn norm_inf(&self) -> f32 {
        let mut best: f32 = 0.0;
        for &value in &self.data {
            best = best.max(abs_f32(value));
        }
        best
    }
}

impl Vector<Complex> {
    pub fn add(&mut self, v: &Vector<Complex>) {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        for (lhs, rhs) in self.data.iter_mut().zip(v.data.iter()) {
            *lhs += *rhs;
        }
    }

    pub fn sub(&mut self, v: &Vector<Complex>) {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        for (lhs, rhs) in self.data.iter_mut().zip(v.data.iter()) {
            *lhs -= *rhs;
        }
    }

    pub fn scl(&mut self, a: Complex) {
        for value in &mut self.data {
            *value *= a;
        }
    }

    pub fn dot(&self, v: &Vector<Complex>) -> Complex {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        let mut acc = Complex::zero();
        for (&lhs, &rhs) in self.data.iter().zip(v.data.iter()) {
            acc += lhs * rhs;
        }
        acc
    }

    pub fn hermitian_dot(&self, v: &Vector<Complex>) -> Complex {
        assert_eq!(self.size(), v.size(), "vector dimensions must match");
        let mut acc = Complex::zero();
        for (&lhs, &rhs) in self.data.iter().zip(v.data.iter()) {
            acc += lhs.conj() * rhs;
        }
        acc
    }

    pub fn norm_1(&self) -> f32 {
        let mut sum = 0.0;
        for &value in &self.data {
            sum += value.abs();
        }
        sum
    }

    pub fn norm(&self) -> f32 {
        let mut squared = 0.0;
        for &value in &self.data {
            squared += value.abs_sq();
        }
        squared.powf(0.5)
    }

    pub fn norm_inf(&self) -> f32 {
        let mut best: f32 = 0.0;
        for &value in &self.data {
            best = best.max(value.abs());
        }
        best
    }
}

impl Add for Vector<f32> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size(), "vector dimensions must match");
        let data = self
            .data
            .into_iter()
            .zip(rhs.data)
            .map(|(lhs, rhs)| lhs + rhs)
            .collect();
        Self { data }
    }
}

impl Sub for Vector<f32> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size(), "vector dimensions must match");
        let data = self
            .data
            .into_iter()
            .zip(rhs.data)
            .map(|(lhs, rhs)| lhs - rhs)
            .collect();
        Self { data }
    }
}

impl Mul<f32> for Vector<f32> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let data = self.data.into_iter().map(|value| value * rhs).collect();
        Self { data }
    }
}

impl Add for Vector<Complex> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size(), "vector dimensions must match");
        let data = self
            .data
            .into_iter()
            .zip(rhs.data)
            .map(|(lhs, rhs)| lhs + rhs)
            .collect();
        Self { data }
    }
}

impl Sub for Vector<Complex> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size(), "vector dimensions must match");
        let data = self
            .data
            .into_iter()
            .zip(rhs.data)
            .map(|(lhs, rhs)| lhs - rhs)
            .collect();
        Self { data }
    }
}

impl Mul<Complex> for Vector<Complex> {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        let data = self.data.into_iter().map(|value| value * rhs).collect();
        Self { data }
    }
}

impl Mul<f32> for Vector<Complex> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let data = self.data.into_iter().map(|value| value * rhs).collect();
        Self { data }
    }
}
