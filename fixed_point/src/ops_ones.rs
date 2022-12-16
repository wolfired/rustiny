//!
//!
//!

use rustiny_number::Integer;
use rustiny_number::One;

use crate::FixedPoint;

impl<T: Integer, const P: usize> One for FixedPoint<T, P> {
    fn one() -> Self {
        Self(T::one() << P)
    }
}
