//!
//!
//!

use std::ops::Neg;

use rustiny_number::ops::CheckedNeg;
use rustiny_number::ops::OverflowingNeg;
use rustiny_number::ops::SaturatingNeg;
use rustiny_number::ops::WrappingNeg;
use rustiny_number::types::Integer;
use rustiny_number::types::Signed;

use crate::types::FixedPoint;

impl<T: Signed, const P: usize> Neg for FixedPoint<T, P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<T: Signed, const P: usize> Neg for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn neg(self) -> Self::Output {
        FixedPoint::<T, P>(-self.0)
    }
}

impl<T: Integer, const P: usize> CheckedNeg for FixedPoint<T, P> {
    fn checked_neg(self) -> Option<Self> {
        let Some(value) = self.0.checked_neg() else {
            return None;
        };
        Some(Self(value))
    }
}

impl<T: Integer, const P: usize> OverflowingNeg for FixedPoint<T, P> {
    fn overflowing_neg(self) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_neg();
        (Self(value), overflowed)
    }
}

impl<T: Signed, const P: usize> SaturatingNeg for FixedPoint<T, P> {
    fn saturating_neg(self) -> Self {
        Self(self.0.saturating_neg())
    }
}

impl<T: Integer, const P: usize> WrappingNeg for FixedPoint<T, P> {
    fn wrapping_neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }
}
