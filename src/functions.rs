use crate::matrix::Matrix;
use crate::scalar::Scalar;
use crate::vector::Vector;

// ex01
/// `Σ coefsᵢ·uᵢ`, O(n) in the total number of coordinates.
pub fn linear_combination<K: Scalar>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    assert_eq!(
        u.len(),
        coefs.len(),
        "vector and coefficient counts must match"
    );
    let Some(first) = u.first() else {
        return Vector::new(Vec::new());
    };
    let size = first.size();
    assert!(
        u.iter().all(|vector| vector.size() == size),
        "all vectors must have the same size"
    );

    let mut out = Vector::zeros(size);
    for (vector, &coef) in u.iter().zip(coefs) {
        for idx in 0..size {
            out[idx] = vector[idx].mul_add(coef, out[idx]);
        }
    }
    out
}

// ex02
/// Types that can be linearly interpolated: scalars, vectors and matrices.
pub trait Lerp: Sized {
    fn lerp_to(self, other: Self, t: f32) -> Self;
}

/// `u + (v - u)·t`, as a single fused multiply-add.
impl<K: Scalar> Lerp for K {
    fn lerp_to(self, other: Self, t: f32) -> Self {
        (other - self).mul_add(K::from_f32(t), self)
    }
}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V {
    u.lerp_to(v, t)
}

// ex05
/// Cosine of the angle between `u` and `v`: `Re⟨u, v⟩ / (‖u‖·‖v‖)`.
///
/// Over the reals this is the usual `u·v / (‖u‖·‖v‖)`. Over the complex numbers
/// it is the cosine of the real angle between the vectors seen as vectors of ℝ²ⁿ,
/// which is why it is always a real number in [-1, 1].
// Hand-written clamp: ex05 allows no math function besides fused multiply-add.
#[allow(clippy::manual_clamp)]
pub fn angle_cos<K: Scalar>(u: &Vector<K>, v: &Vector<K>) -> f32 {
    let norm_product = u.norm() * v.norm();
    assert!(
        norm_product > 0.0,
        "angle cosine undefined for zero vectors"
    );
    let cos = u.hermitian_dot(v).real() / norm_product;
    // Rounding can push colinear vectors slightly outside [-1, 1] (e.g. -1.0000001).
    if cos > 1.0 {
        1.0
    } else if cos < -1.0 {
        -1.0
    } else {
        cos
    }
}

// ex06
pub fn cross_product<K: Scalar>(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {
    assert_eq!(u.size(), 3, "cross product is defined for 3D vectors");
    assert_eq!(v.size(), 3, "cross product is defined for 3D vectors");
    Vector::from([
        u[1].mul_add(v[2], -(u[2] * v[1])),
        u[2].mul_add(v[0], -(u[0] * v[2])),
        u[0].mul_add(v[1], -(u[1] * v[0])),
    ])
}

// ex14
/// Right-handed perspective projection (the camera looks down -z), mapping depth to
/// NDC z ∈ [0, 1]: the near plane goes to 0 and the far plane to 1.
///
/// `fov` is the vertical field of view in radians, `ratio` is width / height.
pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let focal = 1.0 / (fov * 0.5).tan();
    let depth = near - far;

    Matrix::from([
        [focal / ratio, 0.0, 0.0, 0.0],
        [0.0, focal, 0.0, 0.0],
        [0.0, 0.0, far / depth, (far * near) / depth],
        [0.0, 0.0, -1.0, 0.0],
    ])
}
