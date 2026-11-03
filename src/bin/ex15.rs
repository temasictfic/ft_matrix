//! Bonus: every exercise again, with K = Complex. The library code is the same
//! generic code used for f32; only the scalar type changes.

use ft_matrix::{Complex, Matrix, Vector, angle_cos, cross_product, lerp, linear_combination};

fn c(re: f32, im: f32) -> Complex {
    Complex::new(re, im)
}

fn main() {
    let v = Vector::from([c(1.0, 2.0), c(-3.0, 0.5), c(0.0, -1.0)]);
    let w = Vector::from([c(-1.0, 1.0), c(2.0, -2.0), c(0.5, 0.5)]);

    // ex00
    let mut sum = v.clone();
    sum.add(&w);
    println!("ex00 add:\n{}", sum);
    let mut diff = v.clone();
    diff.sub(&w);
    println!("ex00 sub:\n{}", diff);
    let mut scaled = v.clone();
    scaled.scl(c(0.0, 1.0));
    println!("ex00 scl by i:\n{}\n", scaled);

    // ex01
    let lc = linear_combination(&[v.clone(), w.clone()], &[c(0.5, 0.25), c(-1.0, 0.5)]);
    println!("ex01 linear combination:\n{}\n", lc);

    // ex02
    println!("ex02 lerp scalar: {}", lerp(c(0.0, 0.0), c(2.0, 4.0), 0.25));
    println!("ex02 lerp vector:\n{}\n", lerp(v.clone(), w.clone(), 0.5));

    // ex03
    println!("ex03 dot (bilinear): {}", v.dot(&w));
    println!("ex03 hermitian dot: {}", v.hermitian_dot(&w));
    println!("ex03 <v, v> is real: {}\n", v.hermitian_dot(&v));

    // ex04
    println!(
        "ex04 norms: {:?}, {:?}, {:?}\n",
        v.norm_1(),
        v.norm(),
        v.norm_inf()
    );

    // ex05
    println!("ex05 angle cos(v, w): {:?}", angle_cos(&v, &w));
    let iv = Vector::from([c(0.0, 1.0), c(-0.5, -3.0), c(1.0, 0.0)]); // i * v
    println!("ex05 angle cos(v, i*v): {:?}\n", angle_cos(&v, &iv));

    // ex06
    println!("ex06 cross:\n{}\n", cross_product(&v, &w));

    let a = Matrix::from([
        [c(2.0, 0.0), c(1.0, -1.0), c(0.0, 1.0)],
        [c(0.0, 1.0), c(3.0, 0.0), c(-2.0, 0.0)],
        [c(1.0, 0.0), c(0.0, -1.0), c(1.0, 0.0)],
    ]);
    let b = Matrix::from([
        [c(1.0, 0.0), c(0.0, 1.0), c(2.0, 0.0)],
        [c(0.0, -1.0), c(1.0, 0.0), c(0.0, 0.0)],
        [c(1.0, 1.0), c(0.0, 0.0), c(1.0, -1.0)],
    ]);

    // ex07
    println!("ex07 mul_vec:\n{}", a.mul_vec(&w));
    println!("ex07 mul_mat:\n{}\n", a.mul_mat(&b));

    // ex08 / ex09
    println!("ex08 trace: {}", a.trace());
    println!("ex09 transpose:\n{}\n", a.transpose());

    // ex10
    println!("ex10 row echelon:\n{}", a.row_echelon());
    // Second row = i * first row, so the rows are linearly dependent over C.
    let singular = Matrix::from([[c(1.0, 1.0), c(2.0, 0.0)], [c(-1.0, 1.0), c(0.0, 2.0)]]);
    println!("ex10 row echelon (singular):\n{}\n", singular.row_echelon());

    // ex11
    println!("ex11 determinant: {}", a.determinant());
    println!("ex11 determinant (singular): {}\n", singular.determinant());

    // ex12
    let inv = a.inverse().expect("matrix should be invertible");
    println!("ex12 inverse:\n{}", inv);
    println!("ex12 inverse * a:\n{}", inv.mul_mat(&a));
    match singular.inverse() {
        Ok(m) => println!("ex12 unexpected inverse:\n{}\n", m),
        Err(err) => println!("ex12 singular: {}\n", err),
    }

    // ex13
    println!("ex13 rank: {}", a.rank());
    println!("ex13 rank (singular): {}", singular.rank());
    println!(
        "ex13 rank (zero): {}",
        Matrix::<Complex>::zeros(2, 3).rank()
    );
}
