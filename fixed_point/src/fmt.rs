//!
//!
//!

use std::fmt::Binary;
use std::fmt::Debug;
use std::fmt::Display;

use rustiny_number::Integer;
use rustiny_number::Tof64;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Binary for FixedPoint<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#0wid$b}", self.0, wid = { T::BITS as usize + 2 })
    }
}

#[rustfmt::skip]
impl<T: Integer, const P: usize> Debug
    for FixedPoint<T, P>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FixedPoint{{ {:+0chars_count_2_P$.P$}, {:+0chars_count_1$}, {:b} }}",
            self.to_f64(),
            self.0,
            self,
            chars_count_2_P = (if if T::SIGNED { T::BITS - 1 } else { T::BITS } as usize <= P { T::zero() } else { T::MAX >> P }).to_string().chars().count() + 2 + P,
            chars_count_1 = T::MAX.to_string().chars().count() + 1,
        )
    }
}

impl<T: Integer, const P: usize> Display for FixedPoint<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.P$}", self.to_f64())
    }
}
