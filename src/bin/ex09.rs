use ft_matrix::Matrix;

fn main() {
    // The subject gives no example for this exercise.
    let u = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    println!("{}\n", u.transpose());
    // [1.0, 4.0]
    // [2.0, 5.0]
    // [3.0, 6.0]

    let u = Matrix::from([[1., 2.], [3., 4.]]);
    println!("{}", u.transpose());
    // [1.0, 3.0]
    // [2.0, 4.0]
}
