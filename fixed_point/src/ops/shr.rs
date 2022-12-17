//!
//!
//!

use std::ops::Shr;
use std::ops::ShrAssign;

use rustiny_number::ops::CheckedShr;
use rustiny_number::ops::OverflowingShr;
use rustiny_number::ops::WrappingShr;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

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

macro_rules! impl_block {
    ($($t:ty), *) => {
        $(
            impl<T: Integer, const P: usize> Shr<$t> for FixedPoint<T, P>
            where
                T: Shr<$t, Output = T>,
            {
                type Output = Self;

                fn shr(self, rhs: $t) -> Self::Output {
                    Self(self.0 >> rhs)
                }
            }

            impl<T: Integer, const P: usize> Shr<&$t> for FixedPoint<T, P>
            where
                for<'a> T: Shr<&'a $t, Output = T>,
            {
                type Output = Self;

                fn shr(self, rhs: &$t) -> Self::Output {
                    Self(self.0 >> rhs)
                }
            }

            impl<T: Integer, const P: usize> Shr<$t> for &FixedPoint<T, P>
            where
                for<'a> &'a T: Shr<$t, Output = T>,
            {
                type Output = FixedPoint<T, P>;

                fn shr(self, rhs: $t) -> Self::Output {
                    FixedPoint::<T, P>(&self.0 >> rhs)
                }
            }

            impl<T: Integer, const P: usize> Shr<&$t> for &FixedPoint<T, P>
            where
                for<'a> &'a T: Shr<&'a $t, Output = T>,
            {
                type Output = FixedPoint<T, P>;

                fn shr(self, rhs: &$t) -> Self::Output {
                    FixedPoint::<T, P>(&self.0 >> rhs)
                }
            }

            impl<T: Integer, const P: usize> ShrAssign<$t> for FixedPoint<T, P>
            where
                T: ShrAssign<$t>,
            {
                fn shr_assign(&mut self, rhs: $t) {
                    self.0 >>= rhs;
                }
            }

            impl<T: Integer, const P: usize> ShrAssign<&$t> for FixedPoint<T, P>
            where
                for<'a> T: ShrAssign<&'a $t>,
            {
                fn shr_assign(&mut self, rhs: &$t) {
                    self.0 >>= rhs;
                }
            }
        )*
    };
}
impl_block!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<T: Integer, const P: usize> CheckedShr for FixedPoint<T, P> {
    fn checked_shr(self, rhs: u32) -> Option<Self> {
        let Some(value) = self.0.checked_shr(rhs) else {
            return None;
        };
        Some(Self(value))
    }
}

impl<T: Integer, const P: usize> OverflowingShr for FixedPoint<T, P> {
    fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
        let (value, overflowed) = self.0.overflowing_shr(rhs);
        (Self(value), overflowed)
    }
}

impl<T: Integer, const P: usize> WrappingShr for FixedPoint<T, P> {
    fn wrapping_shr(self, rhs: u32) -> Self {
        Self(self.0.wrapping_shr(rhs))
    }
}
