//!
//!
//!

use std::ops::Neg;

use rustiny_number::Integer;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Neg for FixedPoint<T, P>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<T: Integer, const P: usize> Neg for &FixedPoint<T, P>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn neg(self) -> Self::Output {
        FixedPoint::<T, P>(-&self.0)
    }
}
