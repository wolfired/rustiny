//!
//!
//!

use std::ops::Rem;
use std::ops::RemAssign;

use rustiny_number::ops::CheckedRem;
use rustiny_number::ops::OverflowingRem;
use rustiny_number::ops::WrappingRem;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> Rem<Self> for FixedPoint<T, P> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<T: Integer, const P: usize> Rem<&Self> for FixedPoint<T, P> {
    type Output = Self;

    fn rem(self, rhs: &Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<T: Integer, const P: usize> Rem<Self> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn rem(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>(self.0 % rhs.0)
    }
}

impl<T: Integer, const P: usize> Rem<FixedPoint<T, P>> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn rem(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>(self.0 % rhs.0)
    }
}

impl<T: Integer, const P: usize> RemAssign<Self> for FixedPoint<T, P> {
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0
    }
}

impl<T: Integer, const P: usize> RemAssign<&Self> for FixedPoint<T, P> {
    fn rem_assign(&mut self, rhs: &Self) {
        self.0 %= rhs.0
    }
}

impl<T: Integer, const P: usize> CheckedRem for FixedPoint<T, P> {
    fn checked_rem(self, rhs: Self) -> Option<Self> {
        let Some(value) = self.0.checked_rem(rhs.0) else {
            return None;
        };
        Some(Self(value))
    }
}

impl<T: Integer, const P: usize> OverflowingRem for FixedPoint<T, P> {
    fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_rem(rhs.0);
        (Self(value), overflowed)
    }
}

impl<T: Integer, const P: usize> WrappingRem for FixedPoint<T, P> {
    fn wrapping_rem(self, rhs: Self) -> Self {
        Self(self.0.wrapping_rem(rhs.0))
    }
}
