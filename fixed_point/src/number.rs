//!
//!
//!

use std::fmt::Binary;
use std::ops::Shl;
use std::ops::Shr;

use rustiny_number::CheckedSqrt;
use rustiny_number::Integer;
use rustiny_number::Number;
use rustiny_number::One;
use rustiny_number::Tof32;
use rustiny_number::Tof64;
use rustiny_number::Zero;

use crate::FixedPoint;

impl<T: Integer, const P: usize> One for FixedPoint<T, P>
where
    T: Shl<usize, Output = T>,
    T: One,
{
    fn one() -> Self {
        Self(T::one() << P)
    }
}

impl<T: Integer, const P: usize> Zero for FixedPoint<T, P>
where
    T: Zero,
{
    fn zero() -> Self {
        Self(T::zero())
    }
}

impl<T: Integer, const P: usize> CheckedSqrt for FixedPoint<T, P>
where
    T: TryFrom<u128> + TryFrom<i128>,
    T: TryInto<u128> + TryInto<i128>,
{
    fn checked_sqrt(self) -> Option<Self> {
        if T::SIGNED {
            let Ok(value) = TryInto::<i128>::try_into(self.0) else {
                return None
            };
            let Some(value) = (value << P).checked_sqrt() else {
                return None
            };
            let Ok(value) = TryInto::<T>::try_into(value) else {
                return None
            };
            return Some(Self(value));
        } else {
            let Ok(value) = TryInto::<u128>::try_into(self.0) else {
                return None
            };
            let Some(value) = (value << P).checked_sqrt() else {
                return None
            };
            let Ok(value) = TryInto::<T>::try_into(value) else {
                return None
            };
            return Some(Self(value));
        }
    }
}

impl<T: Integer, const P: usize> Number for FixedPoint<T, P>
where
    T: Binary,
    T: Shr<usize, Output = T>,
    T: Zero,
    T: Tof32 + Tof64,
{
}

impl<T: Integer, const P: usize> Integer for FixedPoint<T, P>
where
    T: Binary,
    T: Shr<usize, Output = T>,
    T: Zero,
    T: Tof32 + Tof64,
{
    const BITS: u32 = T::BITS;

    const MIN: Self = Self(T::MIN);

    const MAX: Self = Self(T::MAX);

    const SIGNED: bool = T::SIGNED;
}
