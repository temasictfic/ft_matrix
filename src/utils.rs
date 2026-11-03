/// `|x|`, written by hand: the subject forbids the standard library's math functions.
pub fn abs_f32(value: f32) -> f32 {
    if value < 0.0 { -value } else { value }
}

/// `max(a, b)`, written by hand: ex10–ex13 allow no math function besides
/// fused multiply-add, and they rely on the zero tolerance below.
pub fn max_f32(a: f32, b: f32) -> f32 {
    if a < b { b } else { a }
}

/// Squared threshold below which a value counts as zero during Gaussian elimination.
///
/// This is deliberately *not* `f32::EPSILON` used as an absolute bound.
/// `f32::EPSILON` (~1.19e-7) is machine epsilon: the gap between 1.0 and the
/// next f32, i.e. a *relative* precision. Rounding error left over by elimination
/// grows with the magnitude of the entries and the number of steps, so a fixed
/// absolute threshold is wrong at both ends of the scale: `1e-7 * I` is perfectly
/// invertible, while leftovers like `3e-6` on entries around 30 are just noise.
///
/// The threshold is therefore `dim * f32::EPSILON * max|a_ij|` (the rule LAPACK
/// and NumPy use for rank decisions). It is squared so complex moduli can be
/// compared without taking a square root.
pub fn zero_tolerance_sq(dim: usize, max_modulus_sq: f32) -> f32 {
    let scale = dim as f32 * f32::EPSILON;
    scale * scale * max_modulus_sq
}
