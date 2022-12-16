//!
//!
//!

use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;

use rustiny_number::CheckedShl;
use rustiny_number::Integer;
use rustiny_number::OverflowingShl;
use rustiny_number::WrappingShl;

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

macro_rules! impl_block {
    ($($t:ty), *) => {
        $(
            impl<T: Integer, const P: usize> Shl<$t> for FixedPoint<T, P>
            where
                T: Shl<$t, Output = T>,
            {
                type Output = Self;

                fn shl(self, rhs: $t) -> Self::Output {
                    Self(self.0 << rhs)
                }
            }

            impl<T: Integer, const P: usize> Shl<&$t> for FixedPoint<T, P>
            where
                for<'a> T: Shl<&'a $t, Output = T>,
            {
                type Output = Self;

                fn shl(self, rhs: &$t) -> Self::Output {
                    Self(self.0 << rhs)
                }
            }

            impl<T: Integer, const P: usize> Shl<$t> for &FixedPoint<T, P>
            where
                for<'a> &'a T: Shl<$t, Output = T>,
            {
                type Output = FixedPoint<T, P>;

                fn shl(self, rhs: $t) -> Self::Output {
                    FixedPoint::<T, P>(&self.0 << rhs)
                }
            }

            impl<T: Integer, const P: usize> Shl<&$t> for &FixedPoint<T, P>
            where
                for<'a> &'a T: Shl<&'a $t, Output = T>,
            {
                type Output = FixedPoint<T, P>;

                fn shl(self, rhs: &$t) -> Self::Output {
                    FixedPoint::<T, P>(&self.0 << rhs)
                }
            }

            impl<T: Integer, const P: usize> ShlAssign<$t> for FixedPoint<T, P>
            where
                T: ShlAssign<$t>,
            {
                fn shl_assign(&mut self, rhs: $t) {
                    self.0 <<= rhs;
                }
            }

            impl<T: Integer, const P: usize> ShlAssign<&$t> for FixedPoint<T, P>
            where
                for<'a> T: ShlAssign<&'a $t>,
            {
                fn shl_assign(&mut self, rhs: &$t) {
                    self.0 <<= rhs;
                }
            }
        )*
    };
}
impl_block!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<T: Integer, const P: usize> CheckedShl for FixedPoint<T, P> {
    fn checked_shl(self, rhs: u32) -> Option<Self> {
        let Some(value) = self.0.checked_shl(rhs) else {
            return None;
        };
        Some(Self(value))
    }
}

impl<T: Integer, const P: usize> OverflowingShl for FixedPoint<T, P> {
    fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_shl(rhs);
        (Self(value), overflowed)
    }
}

impl<T: Integer, const P: usize> WrappingShl for FixedPoint<T, P> {
    fn wrapping_shl(self, rhs: u32) -> Self {
        Self(self.0.wrapping_shl(rhs))
    }
}
