//!
//!
//!

use std::ops::Rem;
use std::ops::RemAssign;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Rem<Self> for FixedPoint<T, P>
where
    T: Rem<T, Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

impl<T: Integer, const P: usize> Rem<&Self> for FixedPoint<T, P>
where
    for<'a> T: Rem<&'a T, Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: &Self) -> Self::Output {
        Self(self.0 % &rhs.0)
    }
}

impl<T: Integer, const P: usize> Rem<Self> for &FixedPoint<T, P>
where
    for<'a> &'a T: Rem<&'a T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn rem(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>(&self.0 % &rhs.0)
    }
}

impl<T: Integer, const P: usize> Rem<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    for<'a> &'a T: Rem<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn rem(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>(&self.0 % rhs.0)
    }
}

impl<T: Integer, const P: usize> RemAssign<Self> for FixedPoint<T, P>
where
    T: RemAssign<T>,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0
    }
}

impl<T: Integer, const P: usize> RemAssign<&Self> for FixedPoint<T, P>
where
    for<'a> T: RemAssign<&'a T>,
{
    fn rem_assign(&mut self, rhs: &Self) {
        self.0 %= &rhs.0
    }
}
