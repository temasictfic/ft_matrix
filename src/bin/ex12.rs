use ft_matrix::Matrix;

fn main() {
    let u = Matrix::from([[8.0, 5.0, -2.0], [4.0, 7.0, 20.0], [7.0, 6.0, 1.0]]);
    match u.inverse() {
        Ok(inv) => println!("{}", inv),
        Err(err) => eprintln!("{}", err),
    }
}
