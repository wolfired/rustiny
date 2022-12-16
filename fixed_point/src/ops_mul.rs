//!
//!
//!

use std::ops::Mul;
use std::ops::MulAssign;

use rustiny_number::CheckedMul;
use rustiny_number::Integer;
use rustiny_number::OverflowingMul;
use rustiny_number::SaturatingMul;
use rustiny_number::WrappingMul;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Mul<Self> for FixedPoint<T, P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> Mul<&Self> for FixedPoint<T, P> {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self((self.0 * rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> Mul<Self> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn mul(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>((self.0 * rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> Mul<FixedPoint<T, P>> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn mul(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>((self.0 * rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> MulAssign<Self> for FixedPoint<T, P> {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 >>= P;
    }
}

impl<T: Integer, const P: usize> MulAssign<&Self> for FixedPoint<T, P> {
    fn mul_assign(&mut self, rhs: &Self) {
        self.0 *= rhs.0;
        self.0 >>= P;
    }
}

impl<T: Integer, const P: usize> CheckedMul for FixedPoint<T, P> {
    fn checked_mul(self, rhs: Self) -> Option<Self> {
        let Some(value) = self.0.checked_mul(rhs.0) else {
            return None;
        };
        Some(Self(value >> P))
    }
}

impl<T: Integer, const P: usize> OverflowingMul for FixedPoint<T, P> {
    fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_mul(rhs.0);
        (Self(value >> P), overflowed)
    }
}

impl<T: Integer, const P: usize> SaturatingMul for FixedPoint<T, P> {
    fn saturating_mul(self, rhs: Self) -> Self {
        Self(self.0.saturating_mul(rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> WrappingMul for FixedPoint<T, P> {
    fn wrapping_mul(self, rhs: Self) -> Self {
        Self(self.0.wrapping_mul(rhs.0) >> P)
    }
}
