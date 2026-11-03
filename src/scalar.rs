use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::utils::abs_f32;

/// The field `K` every vector and matrix is built over.
///
/// Implemented for `f32` (the real numbers) and `Complex` (the bonus), so every
/// exercise is written once, generically, and works for both fields.
pub trait Scalar:
    Copy
    + PartialEq
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
{
    /// Additive identity.
    fn zero() -> Self;
    /// Multiplicative identity.
    fn one() -> Self;
    /// Embeds a real number into the field.
    fn from_f32(value: f32) -> Self;
    /// `self * a + b`, using fused multiply-add.
    fn mul_add(self, a: Self, b: Self) -> Self;
    /// Complex conjugate (identity for real numbers).
    fn conj(self) -> Self;
    /// Real part.
    fn real(self) -> f32;
    /// `|self|²`. Needs no square root, so it is used for pivot comparisons.
    fn modulus_sq(self) -> f32;
    /// `|self|`.
    fn modulus(self) -> f32;
    /// Formats the value the way the subject prints it (`7.0`, not `7`).
    fn fmt_scalar(&self, f: &mut Formatter<'_>) -> fmt::Result;
}

impl Scalar for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn from_f32(value: f32) -> Self {
        value
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        // Inherent `f32::mul_add` takes precedence over this trait method.
        f32::mul_add(self, a, b)
    }

    fn conj(self) -> Self {
        self
    }

    fn real(self) -> f32 {
        self
    }

    fn modulus_sq(self) -> f32 {
        self * self
    }

    fn modulus(self) -> f32 {
        abs_f32(self)
    }

    fn fmt_scalar(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
