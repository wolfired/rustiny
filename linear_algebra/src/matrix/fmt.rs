//!
//!
//!

use std::fmt::Display;

use rustiny_number::Number;

use crate::Matrix;

impl<T: Number, const R: usize, const C: usize> Display for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..R {
            for c in 0..C {
                write!(f, "{}", self.0[r][c])?;
                if c < C - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
