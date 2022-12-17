//!
//!
//!

use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> From<FixedPoint<T, P>> for f32 {
    fn from(value: FixedPoint<T, P>) -> Self {
        value.0.to_f32() / (1 << P) as f32
    }
}

impl<T: Integer, const P: usize> From<FixedPoint<T, P>> for f64 {
    fn from(value: FixedPoint<T, P>) -> Self {
        value.0.to_f64() / (1 << P) as f64
    }
}
