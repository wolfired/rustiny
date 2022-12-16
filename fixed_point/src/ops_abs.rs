//!
//!
//!

use rustiny_number::Abs;
use rustiny_number::CheckedAbs;
use rustiny_number::OverflowingAbs;
use rustiny_number::SaturatingAbs;
use rustiny_number::Signed;
use rustiny_number::WrappingAbs;

use crate::FixedPoint;

impl<T: Signed, const P: usize> Abs for FixedPoint<T, P> {
    type Output = Self;

    fn abs(self) -> Self::Output {
        Self(self.0.abs())
    }
}

impl<T: Signed, const P: usize> Abs for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn abs(self) -> Self::Output {
        FixedPoint::<T, P>(self.0.abs())
    }
}

impl<T: Signed, const P: usize> CheckedAbs for FixedPoint<T, P> {
    fn checked_abs(self) -> Option<Self> {
        let Some(value) = self.0.checked_abs() else {
            return None;
        };
        Some(Self(value))
    }
}

impl<T: Signed, const P: usize> OverflowingAbs for FixedPoint<T, P> {
    fn overflowing_abs(self) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_abs();
        (Self(value), overflowed)
    }
}

impl<T: Signed, const P: usize> SaturatingAbs for FixedPoint<T, P> {
    fn saturating_abs(self) -> Self {
        Self(self.0.saturating_abs())
    }
}

impl<T: Signed, const P: usize> WrappingAbs for FixedPoint<T, P> {
    fn wrapping_abs(self) -> Self {
        Self(self.0.wrapping_abs())
    }
}
