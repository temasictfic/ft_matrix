mod complex;
mod errors;
mod functions;
mod matrix;
mod scalar;
mod utils;
mod vector;

pub use complex::Complex;
pub use errors::MatrixError;
pub use functions::{Lerp, angle_cos, cross_product, lerp, linear_combination, projection};
pub use matrix::Matrix;
pub use scalar::Scalar;
pub use vector::Vector;
