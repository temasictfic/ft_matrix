use std::f32::consts::PI;
use std::fs;

use ft_matrix::projection;

/// Usage: cargo run --bin ex14 -- [fov_degrees] [ratio] [near] [far]
///
/// Writes the matrix to `./proj` in the format the display program expects
/// (column-major: each line is one column), then prints it.
fn main() {
    let args: Vec<f32> = std::env::args()
        .skip(1)
        .map(|arg| arg.parse().expect("arguments must be numbers"))
        .collect();
    let arg = |idx: usize, default: f32| args.get(idx).copied().unwrap_or(default);

    let fov_degrees = arg(0, 70.0);
    let ratio = arg(1, 16.0 / 9.0);
    let near = arg(2, 0.1);
    let far = arg(3, 100.0);

    let proj = projection(fov_degrees * PI / 180.0, ratio, near, far);
    println!("projection (math layout, one row per line):\n{}\n", proj);

    let (rows, cols) = proj.shape();
    let text: String = (0..cols)
        .map(|col| {
            let line: Vec<String> = (0..rows)
                .map(|row| format!("{:?}", proj.get(row, col)))
                .collect();
            line.join(", ") + "\n"
        })
        .collect();

    fs::write("proj", &text).expect("cannot write ./proj");
    print!("written to ./proj (column-major):\n{}", text);
}
