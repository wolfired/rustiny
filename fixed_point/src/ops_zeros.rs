//!
//!
//!

use rustiny_number::Integer;
use rustiny_number::Zero;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Zero for FixedPoint<T, P> {
    fn zero() -> Self {
        Self(T::zero())
    }
}
