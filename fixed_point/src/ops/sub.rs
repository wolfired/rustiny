//!
//!
//!

use std::ops::Sub;
use std::ops::SubAssign;

use rustiny_number::ops::CheckedSub;
use rustiny_number::ops::OverflowingSub;
use rustiny_number::ops::SaturatingSub;
use rustiny_number::ops::WrappingSub;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> Sub<Self> for FixedPoint<T, P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T: Integer, const P: usize> Sub<&Self> for FixedPoint<T, P> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T: Integer, const P: usize> Sub<Self> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn sub(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>(self.0 - rhs.0)
    }
}

impl<T: Integer, const P: usize> Sub<FixedPoint<T, P>> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn sub(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>(self.0 - rhs.0)
    }
}

impl<T: Integer, const P: usize> SubAssign<Self> for FixedPoint<T, P> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<T: Integer, const P: usize> SubAssign<&Self> for FixedPoint<T, P> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 -= rhs.0
    }
}

impl<T: Integer, const P: usize> CheckedSub for FixedPoint<T, P> {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        let Some(value) = self.0.checked_sub(rhs.0) else {
            return None;
        };
        Some(Self(value))
    }
}

impl<T: Integer, const P: usize> OverflowingSub for FixedPoint<T, P> {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_sub(rhs.0);
        (Self(value), overflowed)
    }
}

impl<T: Integer, const P: usize> SaturatingSub for FixedPoint<T, P> {
    fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl<T: Integer, const P: usize> WrappingSub for FixedPoint<T, P> {
    fn wrapping_sub(self, rhs: Self) -> Self {
        Self(self.0.wrapping_sub(rhs.0))
    }
}
