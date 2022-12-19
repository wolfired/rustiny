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

pub trait I8: Signed {}
impl I8 for i8 {}

pub trait I16: Signed {}
impl I16 for i16 {}

pub trait I32: Signed {}
impl I32 for i32 {}

pub trait I64: Signed {}
impl I64 for i64 {}

pub trait I128: Signed {}
impl I128 for i128 {}

pub trait ISize: Signed {}
impl ISize for isize {}
