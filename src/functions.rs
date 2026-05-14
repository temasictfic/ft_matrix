use std::ops::{Add, Mul, Sub};

use crate::complex::Complex;
use crate::matrix::Matrix;
use crate::utils::is_near_zero;
use crate::vector::Vector;

pub fn linear_combination(u: &[Vector<f32>], coefs: &[f32]) -> Vector<f32> {
    assert_eq!(
        u.len(),
        coefs.len(),
        "vector and coefficient counts must match"
    );
    if u.is_empty() {
        return Vector::new(Vec::new());
    }
    let size = u[0].size();
    for vector in u {
        assert_eq!(vector.size(), size, "all vectors must have the same size");
    }

    let mut out = vec![0.0; size];
    for (vector, &coef) in u.iter().zip(coefs.iter()) {
        for idx in 0..size {
            out[idx] = vector[idx].mul_add(coef, out[idx]);
        }
    }
    Vector::from(out)
}

pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Sub<Output = V> + Add<Output = V> + Mul<f32, Output = V> + Clone,
{
    u.clone() + (v - u) * t
}

pub fn angle_cos(u: &Vector<f32>, v: &Vector<f32>) -> f32 {
    let norm_product = u.norm() * v.norm();
    assert!(
        !is_near_zero(norm_product),
        "angle cosine undefined for zero vectors"
    );
    u.dot(v) / norm_product
}

pub fn cross_product(u: &Vector<f32>, v: &Vector<f32>) -> Vector<f32> {
    assert_eq!(u.size(), 3, "cross product is defined for 3D vectors");
    assert_eq!(v.size(), 3, "cross product is defined for 3D vectors");
    Vector::from(vec![
        u[1].mul_add(v[2], -(u[2] * v[1])),
        u[2].mul_add(v[0], -(u[0] * v[2])),
        u[0].mul_add(v[1], -(u[1] * v[0])),
    ])
}

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let focal = 1.0 / (fov * 0.5).tan();
    let depth = far - near;

    let mut out = Matrix::<f32>::zeros(4, 4);
    out.set(0, 0, focal / ratio);
    out.set(1, 1, focal);
    out.set(2, 2, far / depth);
    out.set(2, 3, 1.0);
    out.set(3, 2, -(far * near) / depth);
    out
}

pub fn linear_combination_complex(u: &[Vector<Complex>], coefs: &[Complex]) -> Vector<Complex> {
    assert_eq!(
        u.len(),
        coefs.len(),
        "vector and coefficient counts must match"
    );
    if u.is_empty() {
        return Vector::new(Vec::new());
    }
    let size = u[0].size();
    for vector in u {
        assert_eq!(vector.size(), size, "all vectors must have the same size");
    }

    let mut out = vec![Complex::zero(); size];
    for (vector, &coef) in u.iter().zip(coefs.iter()) {
        for idx in 0..size {
            out[idx] += vector[idx] * coef;
        }
    }
    Vector::from(out)
}

pub fn angle_cos_complex(u: &Vector<Complex>, v: &Vector<Complex>) -> Complex {
    let norm_product = u.norm() * v.norm();
    assert!(
        !is_near_zero(norm_product),
        "angle cosine undefined for zero vectors"
    );
    u.hermitian_dot(v) / Complex::from(norm_product)
}

pub fn cross_product_complex(u: &Vector<Complex>, v: &Vector<Complex>) -> Vector<Complex> {
    assert_eq!(u.size(), 3, "cross product is defined for 3D vectors");
    assert_eq!(v.size(), 3, "cross product is defined for 3D vectors");
    Vector::from(vec![
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ])
}
