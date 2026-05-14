use ft_matrix::{Matrix, Vector, lerp};

fn main() {
    println!("{}", lerp(0.0f32, 1.0, 0.3));
    println!(
        "{}",
        lerp(Vector::from([2.0, 1.0]), Vector::from([4.0, 2.0]), 0.3)
    );
    println!(
        "{}",
        lerp(
            Matrix::from([[2.0, 1.0], [3.0, 4.0]]),
            Matrix::from([[20.0, 10.0], [30.0, 40.0]]),
            0.5,
        )
    );
}
