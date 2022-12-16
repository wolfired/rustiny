//!
//!
//!

use std::ops::Add;
use std::ops::AddAssign;

use rustiny_number::CheckedAdd;
use rustiny_number::Integer;
use rustiny_number::OverflowingAdd;
use rustiny_number::SaturatingAdd;
use rustiny_number::WrappingAdd;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Add<Self> for FixedPoint<T, P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Integer, const P: usize> Add<&Self> for FixedPoint<T, P> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Integer, const P: usize> Add<Self> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn add(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>(self.0 + rhs.0)
    }
}

impl<T: Integer, const P: usize> Add<FixedPoint<T, P>> for &FixedPoint<T, P> {
    type Output = FixedPoint<T, P>;

    fn add(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>(self.0 + rhs.0)
    }
}

impl<T: Integer, const P: usize> AddAssign<Self> for FixedPoint<T, P> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<T: Integer, const P: usize> AddAssign<&Self> for FixedPoint<T, P> {
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += rhs.0
    }
}

impl<T: Integer, const P: usize> CheckedAdd for FixedPoint<T, P> {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        let Some(value) = self.0.checked_add(rhs.0) else {
            return None;
        };
        Some(Self(value))
    }
}

impl<T: Integer, const P: usize> OverflowingAdd for FixedPoint<T, P> {
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_add(rhs.0);
        (Self(value), overflowed)
    }
}

impl<T: Integer, const P: usize> SaturatingAdd for FixedPoint<T, P> {
    fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl<T: Integer, const P: usize> WrappingAdd for FixedPoint<T, P> {
    fn wrapping_add(self, rhs: Self) -> Self {
        Self(self.0.wrapping_add(rhs.0))
    }
}
