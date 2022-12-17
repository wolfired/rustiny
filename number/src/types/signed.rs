//!
//!
//!

use std::ops::Neg;

use crate::ops::Abs;
use crate::ops::CheckedAbs;
use crate::ops::OverflowingAbs;
use crate::ops::SaturatingAbs;
use crate::ops::SaturatingNeg;
use crate::ops::WrappingAbs;

use super::Integer;

pub trait Signed
where
    Self: Integer,
    Self: Neg<Output = Self> + SaturatingNeg,
    Self: Abs<Output = Self> + CheckedAbs + OverflowingAbs + SaturatingAbs + WrappingAbs,
{
}

macro_rules! impl_signed {
    ($($t:ty),*) => {
        $(
            impl Signed for $t {}
        )*
    };
}
impl_signed!(i8, i16, i32, i64, i128, isize);
