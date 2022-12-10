//!
//!
//!

use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Shl<Self> for FixedPoint<T, P>
where
    T: Shl<T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self(self.0 << (rhs.0 >> P))
    }
}

impl<T: Integer, const P: usize> Shl<&Self> for FixedPoint<T, P>
where
    T: Shl<T, Output = T>,
    for<'a> &'a T: Shr<usize, Output = T>,
{
    type Output = Self;

    fn shl(self, rhs: &Self) -> Self::Output {
        Self(self.0 << (&rhs.0 >> P))
    }
}

impl<T: Integer, const P: usize> Shl<Self> for &FixedPoint<T, P>
where
    for<'a> &'a T: Shl<T, Output = T>,
    for<'a> &'a T: Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn shl(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>(&self.0 << (&rhs.0 >> P))
    }
}

impl<T: Integer, const P: usize> Shl<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    for<'a> &'a T: Shl<T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn shl(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>(&self.0 << (rhs.0 >> P))
    }
}

impl<T: Integer, const P: usize> ShlAssign<Self> for FixedPoint<T, P>
where
    T: ShlAssign<T>,
    T: Shr<usize, Output = T>,
{
    fn shl_assign(&mut self, rhs: Self) {
        self.0 <<= rhs.0 >> P;
    }
}

impl<T: Integer, const P: usize> ShlAssign<&Self> for FixedPoint<T, P>
where
    T: ShlAssign<T>,
    for<'a> &'a T: Shr<usize, Output = T>,
{
    fn shl_assign(&mut self, rhs: &Self) {
        self.0 <<= &rhs.0 >> P;
    }
}
