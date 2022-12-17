//!
//!
//!

use std::fmt::Binary;

use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> Binary for FixedPoint<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#0wid$b}", self.0, wid = { T::BITS as usize + 2 })
    }
}
