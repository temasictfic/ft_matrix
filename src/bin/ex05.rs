use ft_matrix::{Vector, angle_cos};

fn main() {
    let u = Vector::from([1.0, 2.0, 3.0]);
    let v = Vector::from([4.0, 5.0, 6.0]);
    println!("{}", angle_cos(&u, &v));
}
