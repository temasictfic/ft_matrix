use ft_matrix::{Vector, linear_combination};

fn main() {
    let e1 = Vector::from([1.0, 0.0, 0.0]);
    let e2 = Vector::from([0.0, 1.0, 0.0]);
    let e3 = Vector::from([0.0, 0.0, 1.0]);
    let out = linear_combination(&[e1, e2, e3], &[10.0, -2.0, 0.5]);
    println!("{}", out);
}
