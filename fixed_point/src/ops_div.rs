//!
//!
//!

use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Shl;
use std::ops::ShlAssign;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Div<Self> for FixedPoint<T, P>
where
    T: Div<T, Output = T>,
    T: Shl<usize, Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self((self.0 / rhs.0) << P)
    }
}

impl<T: Integer, const P: usize> Div<&Self> for FixedPoint<T, P>
where
    for<'a> T: Div<&'a T, Output = T>,
    T: Shl<usize, Output = T>,
{
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        Self((self.0 / &rhs.0) << P)
    }
}

impl<T: Integer, const P: usize> Div<Self> for &FixedPoint<T, P>
where
    for<'a> &'a T: Div<&'a T, Output = T>,
    T: Shl<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn div(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>((&self.0 / &rhs.0) << P)
    }
}

impl<T: Integer, const P: usize> Div<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    for<'a> &'a T: Div<T, Output = T>,
    T: Shl<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn div(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>((&self.0 / rhs.0) << P)
    }
}

impl<T: Integer, const P: usize> DivAssign<Self> for FixedPoint<T, P>
where
    T: DivAssign<T>,
    T: ShlAssign<usize>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.0 <<= P;
    }
}

impl<T: Integer, const P: usize> DivAssign<&Self> for FixedPoint<T, P>
where
    for<'a> T: DivAssign<&'a T>,
    T: ShlAssign<usize>,
{
    fn div_assign(&mut self, rhs: &Self) {
        self.0 /= &rhs.0;
        self.0 <<= P;
    }
}
