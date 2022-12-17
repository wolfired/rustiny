//!
//!
//!

use rustiny_number::ops::One;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> One for FixedPoint<T, P> {
    fn one() -> Self {
        Self(T::one() << P)
    }
}
