//! End-to-end check of the `proj` file that `ex14` writes for the display program.
//!
//! The subject: "The display software expects the input matrix to be in
//! Column-major order. Its Normalized Device Coordinates are in the ranges
//! [−1, 1] for the X and Y axes, and [0, 1] for the Z axis."

use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use ft_matrix::projection;

/// Runs `ex14` with `args` in a fresh directory and returns the parsed `proj`
/// file as written: one `Vec` per line.
fn run_ex14(name: &str, args: &[&str]) -> Vec<Vec<f32>> {
    let dir: PathBuf = env::temp_dir().join(format!("ft_matrix_{name}_{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();

    let status = Command::new(env!("CARGO_BIN_EXE_ex14"))
        .args(args)
        .current_dir(&dir)
        .output()
        .expect("cannot run ex14");
    assert!(status.status.success(), "ex14 failed: {status:?}");

    let text = fs::read_to_string(dir.join("proj")).expect("ex14 did not write ./proj");
    fs::remove_dir_all(&dir).unwrap();

    // Same format as the subject's example: `1., 0., 0., 0.` with ", " separators.
    assert!(text.ends_with('\n'));
    text.lines()
        .map(|line| {
            line.split(", ")
                .map(|value| value.parse().unwrap_or_else(|_| panic!("bad number {value:?}")))
                .collect()
        })
        .collect()
}

#[test]
fn ex14_proj_file_is_column_major() {
    let lines = run_ex14("colmajor", &["90", "1.5", "0.5", "50"]);
    let expected = projection(90f32.to_radians(), 1.5, 0.5, 50.);

    assert_eq!(lines.len(), 4, "{lines:?}");
    for (col, line) in lines.iter().enumerate() {
        assert_eq!(line.len(), 4, "{lines:?}");
        for (row, &value) in line.iter().enumerate() {
            // Line `col` holds column `col`, i.e. the transpose of the math layout.
            let want = expected.get(row, col);
            assert!((value - want).abs() <= 1e-6, "line {col} item {row}: {value} != {want}");
        }
    }
    // The -1 that produces w = -z sits at row 3, column 2: the end of line 2.
    assert_eq!(lines[2][3], -1.0);
    assert_eq!(lines[3][3], 0.0);
}

#[test]
fn ex14_proj_file_default_ratio_fits_window() {
    // The display window is 1000x1000, so the default ratio must be 1:
    // the x and y focal lengths are equal.
    let lines = run_ex14("default", &[]);
    assert_eq!(lines[0][0], lines[1][1]);
}
