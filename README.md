# ft_matrix

A small linear algebra library in Rust, written for the 42 project **Enter the Matrix**.
It covers vectors and matrices, from addition up to inverses, ranks and a 3D projection matrix.
The same code works over two fields: the real numbers (`f32`) and the complex numbers
(`Complex`, the bonus).

It uses no math library. The only math functions it calls are the ones each exercise allows
(fused multiply-add, `pow`, `max`, `tan`).

**New to the math?** Open [`docs/math.html`](docs/math.html) in a browser. It explains every
operation in this repo in plain language, with diagrams, worked examples and two small
interactive demos. It works offline.

## Quick start

```sh
cargo build
cargo run --bin ex00      # any of ex00 … ex15
cargo test                # every example from the subject, plus edge cases
```

Each `exNN` binary runs every example from the subject for that exercise. The expected output
is written as a comment under each example.

## Usage

```rust
use ft_matrix::{Matrix, Vector, angle_cos, lerp, linear_combination};

let mut u = Vector::from([2., 3.]);
u.add(&Vector::from([5., 7.]));            // u = [7, 10]

let a = Matrix::from([[3., -5.],           // each inner array is one ROW
                      [6.,  8.]]);
let b = Matrix::from([[2., 1.], [4., 2.]]);
println!("{}", a.mul_mat(&b));              // [-14.0, -7.0]
                                            // [44.0, 22.0]

let inv = a.inverse()?;                     // Err(MatrixError::SingularMatrix) if singular
let r = a.rank();
let mid = lerp(u.clone(), Vector::from([0., 0.]), 0.5);
```

The same calls work with `Complex`:

```rust
use ft_matrix::{Complex, Matrix};

let c = |re, im| Complex::new(re, im);
let m = Matrix::from([[c(1., 1.), c(2., 0.)],
                      [c(0., 2.), c(1., -1.)]]);
println!("{}", m.determinant());            // computed by the same generic code as f32
```

## Exercises

| Ex | Topic | API | Complexity |
|----|-------|-----|------------|
| 00 | Add, subtract, scale | `Vector::{add, sub, scl}`, `Matrix::{add, sub, scl}` | O(n) |
| 01 | Linear combination | `linear_combination(&[Vector], &[K])` | O(n) |
| 02 | Linear interpolation | `lerp(u, v, t)` for scalars, vectors, matrices | O(n) |
| 03 | Dot product | `Vector::dot`, `Vector::hermitian_dot` | O(n) |
| 04 | Norms | `Vector::{norm_1, norm, norm_inf}` | O(n) |
| 05 | Cosine | `angle_cos(&u, &v)` | O(n) |
| 06 | Cross product | `cross_product(&u, &v)` | O(1) |
| 07 | Matrix multiplication | `Matrix::{mul_vec, mul_mat}` | O(nm), O(nmp) |
| 08 | Trace | `Matrix::trace` | O(n) |
| 09 | Transpose | `Matrix::transpose` | O(nm) |
| 10 | Row-echelon form | `Matrix::row_echelon` (reduced form) | O(n³) |
| 11 | Determinant | `Matrix::determinant` (any size) | O(n³) |
| 12 | Inverse | `Matrix::inverse -> Result<Matrix, MatrixError>` | O(n³) |
| 13 | Rank | `Matrix::rank` | O(n³) |
| 14 | Projection matrix (bonus) | `projection(fov, ratio, near, far)` | O(1) |
| 15 | Complex vector spaces (bonus) | everything above with `K = Complex` | — |

## Project layout

```
src/
  scalar.rs     Scalar trait: the field K (zero, one, fma, conj, modulus, …), impl for f32
  complex.rs    Complex number type, implements Scalar
  vector.rs     Vector<K>: ex00, ex03, ex04, reshape to matrix
  matrix.rs     Matrix<K>: ex00, ex07–ex13, Gaussian elimination helpers
  functions.rs  linear_combination, lerp (+ Lerp trait), angle_cos, cross_product, projection
  utils.rs      hand-written abs and max, relative zero tolerance
  errors.rs     MatrixError
  bin/exNN.rs   one runnable demo per exercise
tests/
  subject.rs    all subject examples + tolerance, projection and complex regression tests
  ex14_display.rs  runs the ex14 binary and checks the proj file it writes
display.sh      runs ex14 and opens the display program (with the Linux black-window fix)
docs/
  math.html     illustrated explanation of all the math
```

## Design notes

**Generic over the field.** All algorithms are written once as `impl<K: Scalar>`. `Scalar`
lists exactly what a field must provide: `+ − × ÷`, `zero`, `one`, `mul_add`, `conj`, and
`modulus`. Supporting a new field, such as a finite field, means implementing `Scalar` for it.
Nothing else changes.

**Matrix layout.** `Matrix::from([[..], [..]])` reads each inner array as a **row**, and
`Display` prints one row per line. This matches the subject's examples. Internally the data is
stored **column-major** (`index = col * rows + row`), so reshaping between a vector and a
matrix is free.

**Accumulating vs. pure.** `add`, `sub` and `scl` modify `self` (the accumulator style).
Everything else returns a new value and leaves its inputs alone. This follows the subject's
prototypes.

**Two dot products.** `dot` is the bilinear `Σ uᵢvᵢ` from the prototype. `hermitian_dot` is
`Σ conj(uᵢ)vᵢ`, the true inner product over ℂ. Norms and `angle_cos` use `hermitian_dot`. For
real vectors the two are identical.

**Numerical tolerance.** Elimination decides "is this pivot zero?" with a *relative* threshold,
`max(rows, cols) × f32::EPSILON × max|aᵢⱼ|`. This is the rule LAPACK and NumPy use for rank.
A fixed threshold such as `1e-6` breaks at both ends of the scale: it calls `1e-7·I`
singular, and it treats rounding noise on large entries as real data. Complex values are
compared through `|z|²`, so no square root is needed.

**Exact pivots.** In row-echelon form, each pivot is set to exactly `1` and each eliminated
entry to exactly `0`. Afterwards, any remaining noise below the tolerance is flushed to zero.

## Exercise 14: projection matrix and the display program

`projection` builds a right-handed perspective matrix: the camera looks down −z, and depth is
mapped to NDC z ∈ [0, 1] (near → 0, far → 1). `fov` is in radians.

```sh
./display.sh                     # fov 70°, ratio 1, near 0.1, far 100, then open the display
./display.sh 90 1 0.5 50         # fov_degrees ratio near far
cargo run --bin ex14 -- 90 1 0.5 50   # only write ./proj
```

`ex14` writes `./proj` in the format the display program expects: column-major, one column
per line, comma-separated. It writes the file directly because redirecting with `>` in
PowerShell would save it as UTF-16. The default ratio is 1 because the display window is a fixed
1000×1000 (ratio = w / h). Any other ratio stretches the logo sideways.

`display.sh` extracts `display_linux.tar.gz` if needed, runs `ex14` inside `matrix_display/`,
and starts `./display`. Run the display through the script, not on its own. The display steers
the camera with the mouse: on each cursor event it adds `(x − 500, y − 500) × 0.01` to yaw and
pitch, then warps the cursor back to (500, 500). On a desktop Linux session this can leave the
window **completely black**, because the camera ends up facing the floor:

- **HiDPI scaling** (for example GNOME at 125%): the warp target is in logical pixels, but
  events are reported in physical pixels. Every warp then reads as a 125 px jump, and the
  camera spins forever. The script sets `WINIT_X11_SCALE_FACTOR=1`.
- **The first cursor event** reports wherever the mouse happens to be, which is one large
  jump. The script opens the window once to find where it is placed, then puts the cursor at
  its centre before starting the real run. This needs `python3` and `xwininfo` (`x11-utils`).

The matrix itself has nothing to do with the black window. If the view is still off, move the
mouse to look around: the camera starts at (0, 0, 10) facing the logo at the origin.

To check the matrix, change `near` and `far` and watch which parts of the model get clipped.
For example, `./display.sh 70 1 9.95 10.05` keeps only the depths between 9.95 and 10.05. The
logo is 0.2 thick and centred 10 units away, so its front face (9.9) and back face (10.1)
disappear and only thin slices of its side walls remain.

## Testing

```sh
cargo test
cargo clippy --all-targets
```

The tests in `tests/subject.rs` cover:

- every example in the subject, compared with a tolerance;
- non-symmetric matrices, which catch row/column mix-ups;
- singular matrices at scales from `1e-6` to `1e6`, and tiny or huge invertible matrices;
- the projection: near → 0 and far → 1, frustum edges → NDC ±1, `w = −z`, how each parameter
  acts, and (in `tests/ex14_display.rs`) the column-major `proj` file that `ex14` writes;
- complex vectors and matrices: `A⁻¹A = I`, singular detection, and complex pivots using the
  same tolerance as real ones.
