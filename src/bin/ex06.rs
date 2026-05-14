use ft_matrix::{Vector, cross_product};

fn main() {
    let u = Vector::from([1.0, 2.0, 3.0]);
    let v = Vector::from([4.0, 5.0, 6.0]);
    println!("{}", cross_product(&u, &v));
}
