pub const EPSILON: f32 = 1e-6;

pub fn abs_f32(value: f32) -> f32 {
    if value < 0.0 { -value } else { value }
}

pub fn is_near_zero(value: f32) -> bool {
    abs_f32(value) <= EPSILON
}

pub fn normalize_near_zero(value: f32) -> f32 {
    if is_near_zero(value) { 0.0 } else { value }
}
