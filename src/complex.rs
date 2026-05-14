use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::utils::{EPSILON, abs_f32};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub const fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub const fn one() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }

    pub fn abs_sq(self) -> f32 {
        self.re.mul_add(self.re, self.im * self.im)
    }

    pub fn abs(self) -> f32 {
        self.abs_sq().powf(0.5)
    }

    pub fn is_near_zero(self) -> bool {
        abs_f32(self.re) <= EPSILON && abs_f32(self.im) <= EPSILON
    }

    pub fn normalize_near_zero(self) -> Self {
        let re = if abs_f32(self.re) <= EPSILON {
            0.0
        } else {
            self.re
        };
        let im = if abs_f32(self.im) <= EPSILON {
            0.0
        } else {
            self.im
        };
        Self::new(re, im)
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.im < 0.0 {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
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

impl Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.re * rhs, self.im * rhs)
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.re.mul_add(rhs.re, rhs.im * rhs.im);
        let re = self.re.mul_add(rhs.re, self.im * rhs.im) / denom;
        let im = self.im.mul_add(rhs.re, -(self.re * rhs.im)) / denom;
        Self::new(re, im)
    }
}

impl Div<f32> for Complex {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.re / rhs, self.im / rhs)
    }
}
