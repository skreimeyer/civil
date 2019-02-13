//! Calculus utility to be used in other parts of the library.
//! # THERE IS ZERO ERROR HANDLING HERE #
//! # Everything is going to be heavily refactored before 1.0 #

/// These constants are used for Gauss-Legendre Quadrature calculations. There
/// could definitely be a more generalized implementation, I think. The
/// differance is that writing in these constants prevents having to calculate
/// them each time the GLQ function is called. I would not foresee a realistic
/// need for integrating 10th order polynomials in typical engineering work.
/// This should be adequate for now.
pub const ABSCISSA1: [f32;1] = [0.0_f32];
pub const WEIGHTS1: [f32;1] = [2.0_f32];
pub const ABSCISSA2: [f32;2] = [
-0.5773502691896257_f32,
0.5773502691896257_f32
];
pub const WEIGHTS2: [f32;2] = [
1.0_f32,
1.0_f32
];
pub const ABSCISSA3: [f32;3] = [
-0.7745966692414834_f32,
0.0_f32,
0.7745966692414834_f32
];
pub const WEIGHTS3: [f32;3] = [
0.5555555555555556_f32,
0.8888888888888888_f32,
0.5555555555555556_f32
];
pub const ABSCISSA4: [f32;4] = [
-0.8611363115940526_f32,
-0.3399810435848563_f32,
0.3399810435848563_f32,
0.8611363115940526_f32
];
pub const WEIGHTS4: [f32;4] = [
0.3478548451374538_f32,
0.6521451548625461_f32,
0.6521451548625461_f32,
0.3478548451374538_f32
];
pub const ABSCISSA5: [f32;5] = [
-0.9061798459386640_f32,
-0.5384693101056831_f32,
0.0_f32,
0.5384693101056831_f32,
0.9061798459386640_f32
];
pub const WEIGHTS5: [f32;5] = [
0.2369268850561891_f32,
0.4786286704993665_f32,
0.5688888888888889_f32,
0.4786286704993665_f32,
0.2369268850561891_f32
];

// Here comes the error handling . . .
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct OrderError;

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Maximum polynomial order exceeded!")
    }
}

impl Error for OrderError {
    fn description(&self) -> &str {
        "Maximum polynomial order exceeded!"
    }
}

/// Gauss-Legendre quadrature
pub fn integrate(f:fn(f32) -> f32,n:i32,a:f32,b:f32) -> f32 {
    match n {
        0 => f(0.0_f32),
        1 => {
            let transformed_function = f((b-a)/2.0 * ABSCISSA1[0] + (b+a/2.0));
            (b-a)/2.0 * WEIGHTS1[0] * transformed_function
        },
        2|3 => {
            let mut result = 0.0;
            for k in 0..2 {
                let transformed_function = f((b-a)/2.0 * ABSCISSA2[k] + (b+a/2.0));
                result += (b-a)/2.0 * WEIGHTS2[k] * transformed_function;
            }
            return result;
        },
        4|5 => {
            let mut result = 0.0;
            for k in 0..3 {
                let transformed_function = f((b-a)/2.0 * ABSCISSA3[k] + (b+a/2.0));
                result += (b-a)/2.0 * WEIGHTS3[k] * transformed_function;
            }
            return result;
        },
        6|7 => {
            let mut result = 0.0;
            for k in 0..4 {
                let transformed_function = f((b-a)/2.0 * ABSCISSA4[k] + (b+a/2.0));
                result += (b-a)/2.0 * WEIGHTS4[k] * transformed_function;
            }
            return result;
        },
        8|9 => {
            let mut result = 0.0;
            for k in 0..5 {
                let transformed_function = f((b-a)/2.0 * ABSCISSA5[k] + (b+a/2.0));
                result += (b-a)/2.0 * WEIGHTS5[k] * transformed_function;
            }
            return result;
        },
        _ => 0.0,
    }
}
