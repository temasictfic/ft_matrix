use ft_matrix::{Matrix, MatrixError};

fn show(result: Result<Matrix<f32>, MatrixError>) {
    match result {
        Ok(inverse) => println!("{}\n", inverse),
        Err(err) => println!("error: {}\n", err),
    }
}

fn main() {
    show(Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]).inverse());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]

    show(Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]).inverse());
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]

    show(Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]).inverse());
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]

    // Singular matrix: must return an error.
    show(Matrix::from([[1., 2.], [2., 4.]]).inverse());
    // error: matrix is singular
}
