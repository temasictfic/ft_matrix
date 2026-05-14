use ft_matrix::{Matrix, Vector};

fn main() {
    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);
    u.add(&v);
    println!("{}", u);

    let mut a = Matrix::from([[1.0, 2.0], [3.0, 4.0]]);
    let b = Matrix::from([[7.0, 4.0], [-2.0, 2.0]]);
    a.add(&b);
    println!("{}", a);
}
