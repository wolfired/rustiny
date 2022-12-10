//!
//!
//!

use std::ops::Sub;
use std::ops::SubAssign;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Sub<Self> for FixedPoint<T, P>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T: Integer, const P: usize> Sub<&Self> for FixedPoint<T, P>
where
    for<'a> T: Sub<&'a T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - &rhs.0)
    }
}

impl<T: Integer, const P: usize> Sub<Self> for &FixedPoint<T, P>
where
    for<'a> &'a T: Sub<&'a T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn sub(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>(&self.0 - &rhs.0)
    }
}

impl<T: Integer, const P: usize> Sub<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    for<'a> &'a T: Sub<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn sub(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>(&self.0 - rhs.0)
    }
}

impl<T: Integer, const P: usize> SubAssign<Self> for FixedPoint<T, P>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl<T: Integer, const P: usize> SubAssign<&Self> for FixedPoint<T, P>
where
    for<'a> T: SubAssign<&'a T>,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.0 -= &rhs.0
    }
}
