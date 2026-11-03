use ft_matrix::{Matrix, Vector, lerp};

fn main() {
    println!("{:?}", lerp(0., 1., 0.));
    // 0.0
    println!("{:?}", lerp(0., 1., 1.));
    // 1.0
    println!("{:?}", lerp(0., 1., 0.5));
    // 0.5
    println!("{:?}\n", lerp(21., 42., 0.3));
    // 27.3

    println!(
        "{}\n",
        lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
    );
    // [2.6]
    // [1.3]

    println!(
        "{}",
        lerp(
            Matrix::from([[2., 1.], [3., 4.]]),
            Matrix::from([[20., 10.], [30., 40.]]),
            0.5
        )
    );
    // [[11., 5.5]
    // [16.5, 22.]]
}
