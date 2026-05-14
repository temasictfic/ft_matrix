use ft_matrix::Matrix;

fn main() {
    let u = Matrix::from([[2.0, -5.0, 0.0], [4.0, 3.0, 7.0], [-2.0, 3.0, 4.0]]);
    println!("{}", u.trace());
}
