//!
//!
//!

use std::ops::Shr;
use std::ops::ShrAssign;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Shr<Self> for FixedPoint<T, P>
where
    T: Shr<T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self(self.0 >> (rhs.0 >> P))
    }
}

impl<T: Integer, const P: usize> Shr<&Self> for FixedPoint<T, P>
where
    T: Shr<T, Output = T>,
    for<'a> &'a T: Shr<usize, Output = T>,
{
    type Output = Self;

    fn shr(self, rhs: &Self) -> Self::Output {
        Self(self.0 >> (&rhs.0 >> P))
    }
}

impl<T: Integer, const P: usize> Shr<Self> for &FixedPoint<T, P>
where
    for<'a> &'a T: Shr<T, Output = T>,
    for<'a> &'a T: Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn shr(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>(&self.0 >> (&rhs.0 >> P))
    }
}

impl<T: Integer, const P: usize> Shr<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    for<'a> &'a T: Shr<T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn shr(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>(&self.0 >> (rhs.0 >> P))
    }
}

impl<T: Integer, const P: usize> ShrAssign<Self> for FixedPoint<T, P>
where
    T: ShrAssign<T>,
    T: Shr<usize, Output = T>,
{
    fn shr_assign(&mut self, rhs: Self) {
        self.0 >>= rhs.0 >> P;
    }
}

impl<T: Integer, const P: usize> ShrAssign<&Self> for FixedPoint<T, P>
where
    T: ShrAssign<T>,
    for<'a> &'a T: Shr<usize, Output = T>,
{
    fn shr_assign(&mut self, rhs: &Self) {
        self.0 >>= &rhs.0 >> P;
    }
}
