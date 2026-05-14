use ft_matrix::Vector;

fn main() {
    let u = Vector::from([-1.0, 6.0]);
    let v = Vector::from([3.0, 2.0]);
    println!("{}", u.dot(&v));
}
