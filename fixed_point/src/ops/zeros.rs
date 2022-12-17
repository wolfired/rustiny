//!
//!
//!

use rustiny_number::ops::Zero;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> Zero for FixedPoint<T, P> {
    fn zero() -> Self {
        Self(T::zero())
    }
}
