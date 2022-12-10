//!
//!
//!

use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Shr;
use std::ops::ShrAssign;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Mul<Self> for FixedPoint<T, P>
where
    T: Mul<T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> Mul<&Self> for FixedPoint<T, P>
where
    for<'a> T: Mul<&'a T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self((self.0 * &rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> Mul<Self> for &FixedPoint<T, P>
where
    for<'a> &'a T: Mul<&'a T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn mul(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>((&self.0 * &rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> Mul<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    for<'a> &'a T: Mul<T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn mul(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>((&self.0 * rhs.0) >> P)
    }
}

impl<T: Integer, const P: usize> MulAssign<Self> for FixedPoint<T, P>
where
    T: MulAssign<T>,
    T: ShrAssign<usize>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 >>= P;
    }
}

impl<T: Integer, const P: usize> MulAssign<&Self> for FixedPoint<T, P>
where
    for<'a> T: MulAssign<&'a T>,
    T: ShrAssign<usize>,
{
    fn mul_assign(&mut self, rhs: &Self) {
        self.0 *= &rhs.0;
        self.0 >>= P;
    }
}
