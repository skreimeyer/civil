//! WIP
//! Solvers for systems of equations.
//! 

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MatrixError {
    details: String
}

impl MatrixError {
    fn new(msg: &str) -> MatrixError {
        MatrixError{details: msg.to_string()}
    }
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MatrixError {
    fn description(&self) -> &str {
        &self.details
    }
}


// Matrix is a NxN matrix
#[derive(Debug)]
pub struct Matrix {
    pub stride: usize,
    pub data: Vec<f64>,
}


impl Matrix {
    pub fn mul(&self, m: Matrix) -> Result<Matrix, MatrixError> {
        if m.stride * self.stride != m.data.len() {
            return Err(MatrixError::new("Matrices are of incompatible size!"));
        }
        let mut prod = Matrix{stride: m.stride, data: Vec::with_capacity(self.data.len()/self.stride * m.stride)};
        for i in (0..self.data.len()).step_by(self.stride) {
            for j in 0..m.stride {
                prod.data[i+j] = self.data[i..i+self.stride].iter()
                .zip((j..m.data.len()).step_by(m.stride).map(|k|m.data[k]))
                .map(|(a,b)| a*b)
                .sum();
            }
        }
        Ok(prod)
    }
}