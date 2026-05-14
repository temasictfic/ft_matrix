mod complex;
mod errors;
mod functions;
mod matrix;
mod utils;
mod vector;

pub use complex::Complex;
pub use errors::MatrixError;
pub use functions::{
    angle_cos, angle_cos_complex, cross_product, cross_product_complex, lerp, linear_combination,
    linear_combination_complex, projection,
};
pub use matrix::Matrix;
pub use vector::Vector;
