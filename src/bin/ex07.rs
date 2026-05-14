use ft_matrix::{Matrix, Vector};

fn main() {
    let m = Matrix::from([[3.0, -5.0], [6.0, 8.0]]);
    let v = Vector::from([4.0, 2.0]);
    println!("{}", m.mul_vec(&v));

    let a = Matrix::from([[3.0, -5.0], [6.0, 8.0]]);
    let b = Matrix::from([[2.0, 1.0], [4.0, 2.0]]);
    println!("{}", a.mul_mat(&b));
}
