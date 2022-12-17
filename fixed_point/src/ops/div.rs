//!
//!
//!

use std::ops::Div;
use std::ops::DivAssign;

use rustiny_number::ops::CheckedDiv;
use rustiny_number::ops::OverflowingDiv;
use rustiny_number::ops::SaturatingDiv;
use rustiny_number::ops::WrappingDiv;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> Div<Self> for FixedPoint<T, P> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self((self.0 / rhs.0) << P)
    }
}

impl<T: Integer, const P: usize> Div<&Self> for FixedPoint<T, P> {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        Self((self.0 / rhs.0) << P)
    }
}

impl<T: Integer, const P: usize> Div<Self> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn div(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>((self.0 / rhs.0) << P)
    }
}

impl<T: Integer, const P: usize> Div<FixedPoint<T, P>> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn div(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>((self.0 / rhs.0) << P)
    }
}

impl<T: Integer, const P: usize> DivAssign<Self> for FixedPoint<T, P> {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.0 <<= P;
    }
}

impl<T: Integer, const P: usize> DivAssign<&Self> for FixedPoint<T, P> {
    fn div_assign(&mut self, rhs: &Self) {
        self.0 /= rhs.0;
        self.0 <<= P;
    }
}

impl<T: Integer, const P: usize> CheckedDiv for FixedPoint<T, P> {
    fn checked_div(self, rhs: Self) -> Option<Self> {
        let Some(value) = self.0.checked_div(rhs.0) else {
            return None;
        };
        Some(Self(value))
    }
}

impl<T: Integer, const P: usize> OverflowingDiv for FixedPoint<T, P> {
    fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_div(rhs.0);
        (Self(value), overflowed)
    }
}

impl<T: Integer, const P: usize> SaturatingDiv for FixedPoint<T, P> {
    fn saturating_div(self, rhs: Self) -> Self {
        Self(self.0.saturating_div(rhs.0))
    }
}

impl<T: Integer, const P: usize> WrappingDiv for FixedPoint<T, P> {
    fn wrapping_div(self, rhs: Self) -> Self {
        Self(self.0.wrapping_div(rhs.0))
    }
}
