//!
//!
//!

use std::fmt::Debug;

use rustiny_number::convert::Tof64;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

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
