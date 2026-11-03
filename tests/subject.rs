//! Every example from the subject, plus regression tests for tolerance handling
//! and the complex bonus.

use ft_matrix::{
    Complex, Matrix, MatrixError, Scalar, Vector, angle_cos, cross_product, lerp,
    linear_combination, projection,
};

const TOL: f32 = 1e-4;

fn close(a: f32, b: f32) -> bool {
    let diff = a - b;
    let scale = if b < 0.0 { -b } else { b };
    diff.modulus() <= TOL * (1.0 + scale)
}

fn assert_close(actual: f32, expected: f32) {
    assert!(close(actual, expected), "{actual} != {expected}");
}

fn assert_vec<const N: usize>(actual: &Vector<f32>, expected: [f32; N]) {
    assert_eq!(actual.size(), N, "size of {actual}");
    for (idx, &e) in expected.iter().enumerate() {
        assert!(close(actual[idx], e), "\n{actual}\n!= {expected:?}");
    }
}

fn assert_mat<const R: usize, const C: usize>(actual: &Matrix<f32>, expected: [[f32; C]; R]) {
    assert_eq!(actual.shape(), (R, C), "shape of\n{actual}");
    for (row, values) in expected.iter().enumerate() {
        for (col, &e) in values.iter().enumerate() {
            assert!(
                close(actual.get(row, col), e),
                "\n{actual}\n!= {expected:?}"
            );
        }
    }
}

fn assert_cmat(actual: &Matrix<Complex>, expected: &Matrix<Complex>) {
    assert_eq!(actual.shape(), expected.shape());
    let (rows, cols) = actual.shape();
    for row in 0..rows {
        for col in 0..cols {
            let diff = actual.get(row, col) - expected.get(row, col);
            assert!(diff.modulus() <= TOL, "\n{actual}\n!=\n{expected}");
        }
    }
}

fn c(re: f32, im: f32) -> Complex {
    Complex::new(re, im)
}

// ---------------------------------------------------------------- ex00

#[test]
fn ex00_vector_add_sub_scl() {
    let mut u = Vector::from([2., 3.]);
    u.add(&Vector::from([5., 7.]));
    assert_vec(&u, [7., 10.]);

    let mut u = Vector::from([2., 3.]);
    u.sub(&Vector::from([5., 7.]));
    assert_vec(&u, [-3., -4.]);

    let mut u = Vector::from([2., 3.]);
    u.scl(2.);
    assert_vec(&u, [4., 6.]);
}

#[test]
fn ex00_matrix_add_sub_scl() {
    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.add(&Matrix::from([[7., 4.], [-2., 2.]]));
    assert_mat(&u, [[8., 6.], [1., 6.]]);

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.sub(&Matrix::from([[7., 4.], [-2., 2.]]));
    assert_mat(&u, [[-6., -2.], [5., 2.]]);

    let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    u.scl(2.);
    assert_mat(&u, [[2., 4.], [6., 8.]]);
}

// ---------------------------------------------------------------- ex01

#[test]
fn ex01_linear_combination() {
    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    assert_vec(
        &linear_combination(&[e1, e2, e3], &[10., -2., 0.5]),
        [10., -2., 0.5],
    );

    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    assert_vec(&linear_combination(&[v1, v2], &[10., -2.]), [10., 0., 230.]);
}

// ---------------------------------------------------------------- ex02

#[test]
fn ex02_lerp() {
    assert_eq!(lerp(0., 1., 0.), 0.0);
    assert_eq!(lerp(0., 1., 1.), 1.0);
    assert_eq!(lerp(0., 1., 0.5), 0.5);
    assert_close(lerp(21., 42., 0.3), 27.3);
    assert_vec(
        &lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3),
        [2.6, 1.3],
    );
    assert_mat(
        &lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5,
        ),
        [[11., 5.5], [16.5, 22.]],
    );
}

// ---------------------------------------------------------------- ex03

#[test]
fn ex03_dot() {
    assert_eq!(Vector::from([0., 0.]).dot(&Vector::from([1., 1.])), 0.0);
    assert_eq!(Vector::from([1., 1.]).dot(&Vector::from([1., 1.])), 2.0);
    assert_eq!(Vector::from([-1., 6.]).dot(&Vector::from([3., 2.])), 9.0);
}

// ---------------------------------------------------------------- ex04

#[test]
fn ex04_norms() {
    let u: Vector<f32> = Vector::from([0., 0., 0.]);
    assert_eq!((u.norm_1(), u.norm(), u.norm_inf()), (0.0, 0.0, 0.0));

    let u: Vector<f32> = Vector::from([1., 2., 3.]);
    assert_close(u.norm_1(), 6.0);
    assert_close(u.norm(), 3.741_657_4);
    assert_close(u.norm_inf(), 3.0);

    let u: Vector<f32> = Vector::from([-1., -2.]);
    assert_close(u.norm_1(), 3.0);
    assert_close(u.norm(), 2.236_068);
    assert_close(u.norm_inf(), 2.0);
}

// ---------------------------------------------------------------- ex05

#[test]
fn ex05_angle_cos() {
    let cases: [([f32; 2], [f32; 2], f32); 4] = [
        ([1., 0.], [1., 0.], 1.0),
        ([1., 0.], [0., 1.], 0.0),
        ([-1., 1.], [1., -1.], -1.0),
        ([2., 1.], [4., 2.], 1.0),
    ];
    for (u, v, expected) in cases {
        let cos = angle_cos(&Vector::from(u), &Vector::from(v));
        assert_close(cos, expected);
        assert!((-1.0..=1.0).contains(&cos), "{cos} outside [-1, 1]");
    }
    assert_close(
        angle_cos(&Vector::from([1., 2., 3.]), &Vector::from([4., 5., 6.])),
        0.974_631_85,
    );
}

// ---------------------------------------------------------------- ex06

#[test]
fn ex06_cross_product() {
    let cross = |u: [f32; 3], v: [f32; 3]| cross_product(&Vector::from(u), &Vector::from(v));
    assert_vec(&cross([0., 0., 1.], [1., 0., 0.]), [0., 1., 0.]);
    assert_vec(&cross([1., 2., 3.], [4., 5., 6.]), [-3., 6., -3.]);
    assert_vec(&cross([4., 2., -3.], [-2., -5., 16.]), [17., -58., -16.]);
}

// ---------------------------------------------------------------- ex07

#[test]
fn ex07_mul_vec() {
    let v = Vector::from([4., 2.]);
    assert_vec(&Matrix::from([[1., 0.], [0., 1.]]).mul_vec(&v), [4., 2.]);
    assert_vec(&Matrix::from([[2., 0.], [0., 2.]]).mul_vec(&v), [8., 4.]);
    assert_vec(&Matrix::from([[2., -2.], [-2., 2.]]).mul_vec(&v), [4., -4.]);
    // Not symmetric, so it catches a row/column mix-up.
    assert_vec(&Matrix::from([[3., -5.], [6., 8.]]).mul_vec(&v), [2., 40.]);
}

#[test]
fn ex07_mul_mat() {
    let id = Matrix::from([[1., 0.], [0., 1.]]);
    assert_mat(&id.mul_mat(&id), [[1., 0.], [0., 1.]]);
    assert_mat(
        &id.mul_mat(&Matrix::from([[2., 1.], [4., 2.]])),
        [[2., 1.], [4., 2.]],
    );
    assert_mat(
        &Matrix::from([[3., -5.], [6., 8.]]).mul_mat(&Matrix::from([[2., 1.], [4., 2.]])),
        [[-14., -7.], [44., 22.]],
    );
    // Non-square: (2x3)(3x1) = 2x1
    assert_mat(
        &Matrix::from([[1., 2., 3.], [4., 5., 6.]]).mul_mat(&Matrix::from([[1.], [0.], [-1.]])),
        [[-2.], [-2.]],
    );
}

// ---------------------------------------------------------------- ex08 / ex09

#[test]
fn ex08_trace() {
    assert_eq!(Matrix::from([[1., 0.], [0., 1.]]).trace(), 2.0);
    assert_eq!(
        Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]).trace(),
        9.0
    );
    assert_eq!(
        Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]).trace(),
        -21.0
    );
}

#[test]
fn ex09_transpose() {
    assert_mat(
        &Matrix::from([[1., 2., 3.], [4., 5., 6.]]).transpose(),
        [[1., 4.], [2., 5.], [3., 6.]],
    );
}

// ---------------------------------------------------------------- ex10

#[test]
fn ex10_row_echelon() {
    assert_mat(
        &Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]).row_echelon(),
        [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    );
    assert_mat(
        &Matrix::from([[1., 2.], [3., 4.]]).row_echelon(),
        [[1., 0.], [0., 1.]],
    );
    assert_mat(
        &Matrix::from([[1., 2.], [2., 4.]]).row_echelon(),
        [[1., 2.], [0., 0.]],
    );
    assert_mat(
        &Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ])
        .row_echelon(),
        [
            [1.0, 0.625, 0.0, 0.0, -12.166_667],
            [0.0, 0.0, 1.0, 0.0, -3.666_666_7],
            [0.0, 0.0, 0.0, 1.0, 29.5],
        ],
    );
}

// ---------------------------------------------------------------- ex11

#[test]
fn ex11_determinant() {
    assert_close(Matrix::from([[1., -1.], [-1., 1.]]).determinant(), 0.0);
    assert_close(
        Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]).determinant(),
        8.0,
    );
    assert_close(
        Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]).determinant(),
        -174.0,
    );
    assert_close(
        Matrix::from([
            [8., 5., -2., 4.],
            [4., 2.5, 20., 4.],
            [8., 5., 1., 4.],
            [28., -4., 17., 1.],
        ])
        .determinant(),
        1032.0,
    );
}

#[test]
fn ex11_determinant_row_swap_sign() {
    assert_close(Matrix::from([[0., 1.], [1., 0.]]).determinant(), -1.0);
}

#[test]
fn ex11_determinant_beyond_4x4() {
    let mut m = Matrix::<f32>::identity(5);
    m.scl(2.);
    assert_close(m.determinant(), 32.0);
}

// ---------------------------------------------------------------- ex12

#[test]
fn ex12_inverse() {
    assert_mat(
        &Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
            .inverse()
            .unwrap(),
        [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    );
    assert_mat(
        &Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]])
            .inverse()
            .unwrap(),
        [[0.5, 0., 0.], [0., 0.5, 0.], [0., 0., 0.5]],
    );
    assert_mat(
        &Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]])
            .inverse()
            .unwrap(),
        [
            [0.649_425_3, 0.097_701_15, -0.655_172_4],
            [-0.781_609_2, -0.126_436_78, 0.965_517_2],
            [0.143_678_16, 0.074_712_64, -0.206_896_55],
        ],
    );
}

#[test]
fn ex12_singular_is_an_error() {
    assert_eq!(
        Matrix::from([[1., 2.], [2., 4.]]).inverse(),
        Err(MatrixError::SingularMatrix)
    );
    assert_eq!(
        Matrix::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]).inverse(),
        Err(MatrixError::SingularMatrix)
    );
}

// ---------------------------------------------------------------- ex13

#[test]
fn ex13_rank() {
    assert_eq!(
        Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]).rank(),
        3
    );
    assert_eq!(
        Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]).rank(),
        2
    );
    assert_eq!(
        Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]).rank(),
        3
    );
    assert_eq!(Matrix::<f32>::zeros(3, 2).rank(), 0);
}

// ---------------------------------------------------------------- tolerance

/// Singular matrices whose elimination leaves rounding noise instead of exact zeros.
#[test]
fn tolerance_detects_singular_matrices_at_any_scale() {
    for scale in [1e-6f32, 1e-3, 0.1, 1.0, 1e3, 1e6] {
        let mut m = Matrix::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
        m.scl(scale);
        assert_eq!(m.rank(), 2, "scale {scale}");
        assert_eq!(
            m.inverse(),
            Err(MatrixError::SingularMatrix),
            "scale {scale}"
        );
        assert_eq!(m.determinant(), 0.0, "scale {scale}");

        let mut m = Matrix::from([[0.1, 0.2, 0.3], [0.4, 0.5, 0.6], [0.7, 0.8, 0.9]]);
        m.scl(scale);
        assert_eq!(m.rank(), 2, "scale {scale}");
    }
}

/// A fixed absolute threshold (like the old 1e-6) would call these singular.
#[test]
fn tolerance_keeps_small_invertible_matrices() {
    let tiny = Matrix::from([[1e-7, 0.], [0., 1e-7]]);
    assert_eq!(tiny.rank(), 2);
    let inv = tiny.inverse().expect("1e-7 * I is invertible");
    assert_close(inv.get(0, 0) / 1e7, 1.0);
}

/// Flushing noise must not erase genuinely small results.
#[test]
fn tolerance_keeps_small_inverse_entries() {
    let huge = Matrix::from([[1e7, 0.], [0., 1e7]]);
    let inv = huge.inverse().unwrap();
    assert_close(inv.get(0, 0) * 1e7, 1.0);
}

// ---------------------------------------------------------------- ex14

#[test]
fn ex14_projection_maps_near_to_0_and_far_to_1() {
    let (near, far) = (0.5, 50.0);
    let p = projection(std::f32::consts::FRAC_PI_2, 1.0, near, far);
    let ndc_z = |z: f32| {
        let clip = p.mul_vec(&Vector::from([0., 0., z, 1.]));
        clip[2] / clip[3]
    };
    // Right-handed: the camera looks down -z.
    assert_close(ndc_z(-near), 0.0);
    assert_close(ndc_z(-far), 1.0);
    // fov = 90°, ratio = 1  =>  focal length 1
    assert_close(p.get(0, 0), 1.0);
    assert_close(p.get(1, 1), 1.0);
}

// ---------------------------------------------------------------- reshape

#[test]
fn reshape_round_trip() {
    let v = Vector::from([1., 2., 3., 4., 5., 6.]);
    let m = v.to_matrix(2, 3);
    assert_mat(&m, [[1., 3., 5.], [2., 4., 6.]]);
    assert_eq!(m.to_vector(), v);
}

// ---------------------------------------------------------------- ex15 (complex)

#[test]
fn ex15_complex_vectors() {
    let v = Vector::from([c(1., 2.), c(-3., 0.5)]);
    let w = Vector::from([c(-1., 1.), c(2., -2.)]);

    let mut sum = v.clone();
    sum.add(&w);
    assert_eq!(sum, Vector::from([c(0., 3.), c(-1., -1.5)]));

    // <v, v> is real and equals ‖v‖²
    let vv = v.hermitian_dot(&v);
    assert_close(vv.im, 0.0);
    assert_close(vv.re, v.norm() * v.norm());

    // Multiplying by i keeps the length
    let mut iv = v.clone();
    iv.scl(c(0., 1.));
    assert_close(iv.norm(), v.norm());

    assert_close(angle_cos(&v, &v), 1.0);
    assert_eq!(
        cross_product(
            &Vector::from([c(1., 0.), c(0., 0.), c(0., 0.)]),
            &Vector::from([c(0., 0.), c(0., 1.), c(0., 0.)]),
        ),
        Vector::from([c(0., 0.), c(0., 0.), c(0., 1.)])
    );
    assert_eq!(lerp(c(0., 0.), c(2., 4.), 0.5), c(1., 2.));
    assert_eq!(
        linear_combination(std::slice::from_ref(&v), &[c(0., 1.)]),
        Vector::from([c(-2., 1.), c(-0.5, -3.)])
    );
}

#[test]
fn ex15_complex_matrices() {
    let a = Matrix::from([
        [c(2., 0.), c(1., -1.), c(0., 1.)],
        [c(0., 1.), c(3., 0.), c(-2., 0.)],
        [c(1., 0.), c(0., -1.), c(1., 0.)],
    ]);
    let det = a.determinant();
    assert!((det - c(3., -5.)).modulus() < TOL, "{det}");
    assert_eq!(a.trace(), c(6., 0.));
    assert_eq!(a.rank(), 3);

    let inv = a.inverse().unwrap();
    assert_cmat(&inv.mul_mat(&a), &Matrix::identity(3));
    assert_cmat(&a.row_echelon(), &Matrix::identity(3));

    // Second row = i * first row
    let singular = Matrix::from([[c(1., 1.), c(2., 0.)], [c(-1., 1.), c(0., 2.)]]);
    assert_eq!(singular.rank(), 1);
    assert!(singular.determinant().modulus() < TOL);
    assert_eq!(singular.inverse(), Err(MatrixError::SingularMatrix));
}

/// Regression: complex pivots used to be compared as |z|² against 1e-6, so any
/// pivot with |z| <= 1e-3 was treated as zero.
#[test]
fn ex15_complex_tolerance_matches_real() {
    let real = Matrix::from([[5e-4, 0.], [0., 5e-4]]);
    let complex = Matrix::from([[c(5e-4, 0.), c(0., 0.)], [c(0., 0.), c(5e-4, 0.)]]);
    assert!(real.inverse().is_ok());
    assert!(complex.inverse().is_ok());
    assert_eq!(real.rank(), complex.rank());
}
