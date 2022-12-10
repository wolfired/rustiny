//!
//!
//!

use std::fmt::Binary;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Shr;

use rustiny_number::Integer;
use rustiny_number::Tof32;
use rustiny_number::Tof64;
use rustiny_number::Zero;

use crate::FixedPoint;

#[rustfmt::skip]
impl<T: Integer, const P: usize> Debug
    for FixedPoint<T, P>
where
    T: Binary,
    T: Shr<usize, Output = T>,
    T: Tof32 + Tof64,
    T: Zero,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FixedPoint{{ {:+0chars_count_2_P$.P$}, {:+0chars_count_1$}, {:#0bits_2$b} }}",
            Into::<f64>::into(*self),
            self.0,
            self.0,
            chars_count_2_P = (if if T::SIGNED { T::BITS - 1 } else { T::BITS } as usize <= P { T::zero() } else { T::MAX >> P }).to_string().chars().count() + 2 + P,
            chars_count_1 = T::MAX.to_string().chars().count() + 1,
            bits_2 = { T::BITS as usize + 2 },
        )
    }
}

impl<T: Integer, const P: usize> Display for FixedPoint<T, P>
where
    T: Tof32 + Tof64,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.P$}", Into::<f64>::into(*self))
    }
}
