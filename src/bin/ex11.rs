use ft_matrix::Matrix;

fn main() {
    let u = Matrix::from([
        [8.0, 5.0, -2.0, 4.0],
        [4.0, 2.5, 20.0, 4.0],
        [8.0, 5.0, 1.0, 4.0],
        [28.0, -4.0, 17.0, 1.0],
    ]);
    println!("{}", u.determinant());
}
