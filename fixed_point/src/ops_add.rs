//!
//!
//!

use std::ops::Add;
use std::ops::AddAssign;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Add<Self> for FixedPoint<T, P>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T: Integer, const P: usize> Add<&Self> for FixedPoint<T, P>
where
    for<'a> T: Add<&'a T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + &rhs.0)
    }
}

impl<T: Integer, const P: usize> Add<Self> for &FixedPoint<T, P>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn add(self, rhs: Self) -> Self::Output {
        FixedPoint::<T, P>(&self.0 + &rhs.0)
    }
}

impl<T: Integer, const P: usize> Add<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    for<'a> &'a T: Add<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn add(self, rhs: FixedPoint<T, P>) -> Self::Output {
        FixedPoint::<T, P>(&self.0 + rhs.0)
    }
}

impl<T: Integer, const P: usize> AddAssign<Self> for FixedPoint<T, P>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl<T: Integer, const P: usize> AddAssign<&Self> for FixedPoint<T, P>
where
    for<'a> T: AddAssign<&'a T>,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.0 += &rhs.0
    }
}
