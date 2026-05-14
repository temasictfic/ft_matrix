use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixError {
    SingularMatrix,
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::SingularMatrix => write!(f, "matrix is singular"),
        }
    }
}

impl Error for MatrixError {}
