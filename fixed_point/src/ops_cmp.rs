//!
//!
//!

use std::cmp::Ordering;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> PartialEq<Self> for FixedPoint<T, P> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Integer, const P: usize> Eq for FixedPoint<T, P> where T: Eq {}

impl<T: Integer, const P: usize> PartialOrd<Self> for FixedPoint<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Integer, const P: usize> Ord for FixedPoint<T, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
