use ft_matrix::projection;

fn main() {
    let p = projection(70.0f32.to_radians(), 16.0 / 9.0, 0.1, 100.0);
    println!("{}", p);
}
