use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::scalar::Scalar;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub const fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }
}

impl Scalar for Complex {
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    fn one() -> Self {
        Self::new(1.0, 0.0)
    }

    fn from_f32(value: f32) -> Self {
        Self::new(value, 0.0)
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        // (x + iy)(u + iv) + (p + iq) = (xu - yv + p) + i(xv + yu + q)
        let re = self.re.mul_add(a.re, (-self.im).mul_add(a.im, b.re));
        let im = self.re.mul_add(a.im, self.im.mul_add(a.re, b.im));
        Self::new(re, im)
    }

    fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }

    fn real(self) -> f32 {
        self.re
    }

    fn modulus_sq(self) -> f32 {
        self.re.mul_add(self.re, self.im * self.im)
    }

    fn modulus(self) -> f32 {
        self.modulus_sq().powf(0.5)
    }

    fn fmt_scalar(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl From<f32> for Complex {
    fn from(value: f32) -> Self {
        Self::new(value, 0.0)
    }
}

impl From<(f32, f32)> for Complex {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{:?}-{:?}i", self.re, -self.im)
        } else {
            write!(f, "{:?}+{:?}i", self.re, self.im)
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let re = self.re.mul_add(rhs.re, -(self.im * rhs.im));
        let im = self.re.mul_add(rhs.im, self.im * rhs.re);
        Self::new(re, im)
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.modulus_sq();
        let re = self.re.mul_add(rhs.re, self.im * rhs.im) / denom;
        let im = self.im.mul_add(rhs.re, -(self.re * rhs.im)) / denom;
        Self::new(re, im)
    }
}
