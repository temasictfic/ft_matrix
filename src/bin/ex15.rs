use ft_matrix::{
    Complex, Matrix, Vector, angle_cos_complex, cross_product_complex, lerp,
    linear_combination_complex,
};

fn c(re: f32, im: f32) -> Complex {
    Complex::new(re, im)
}

fn main() {
    let mut v = Vector::from([c(1.0, 2.0), c(-3.0, 0.5), c(0.0, -1.0)]);
    let w = Vector::from([c(-1.0, 1.0), c(2.0, -2.0), c(0.5, 0.5)]);
    v.add(&w);
    println!("ex00 add:\n{}", v);
    println!("ex03 dot:\n{}", v.dot(&w));
    println!(
        "ex04 norms:\n{}, {}, {}",
        v.norm_1(),
        v.norm(),
        v.norm_inf()
    );
    println!("ex05 angle cos:\n{}", angle_cos_complex(&v, &w));

    let lc = linear_combination_complex(&[v.clone(), w.clone()], &[c(0.5, 0.25), c(-1.0, 0.5)]);
    println!("ex01 linear combination:\n{}", lc);

    let cp = cross_product_complex(&v, &w);
    println!("ex06 cross:\n{}", cp);

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

    println!("ex07 mul_vec:\n{}", a.mul_vec(&w));
    println!("ex07 mul_mat:\n{}", a.mul_mat(&b));
    println!("ex08 trace:\n{}", a.trace());
    println!("ex09 transpose:\n{}", a.transpose());
    println!("ex10 row echelon:\n{}", a.row_echelon());
    println!("ex11 determinant:\n{}", a.determinant());
    println!(
        "ex12 inverse:\n{}",
        a.inverse().expect("matrix should be invertible")
    );
    println!("ex13 rank:\n{}", a.rank());

    let mid = lerp(v, w, 0.5);
    println!("ex02 lerp:\n{}", mid);
}
