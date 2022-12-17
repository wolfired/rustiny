//!
//!
//!

use std::fmt::Display;

use rustiny_number::convert::Tof64;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> Display for FixedPoint<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.P$}", self.to_f64())
    }
}
