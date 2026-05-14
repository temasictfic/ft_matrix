use ft_matrix::Vector;

fn main() {
    let u = Vector::from([-1.0, -2.0]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
}
