//!
//!
//!

use std::fmt::Binary;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::ops_abs::CheckedAbs;
use crate::ops_abs::OverflowingAbs;
use crate::ops_abs::SaturatingAbs;
use crate::ops_abs::WrappingAbs;
use crate::Abs;
use crate::CheckedAdd;
use crate::CheckedDiv;
use crate::CheckedMul;
use crate::CheckedNeg;
use crate::CheckedRem;
use crate::CheckedShl;
use crate::CheckedShr;
use crate::CheckedSub;
use crate::One;
use crate::OverflowingAdd;
use crate::OverflowingDiv;
use crate::OverflowingMul;
use crate::OverflowingNeg;
use crate::OverflowingRem;
use crate::OverflowingShl;
use crate::OverflowingShr;
use crate::OverflowingSub;
use crate::SaturatingAdd;
use crate::SaturatingDiv;
use crate::SaturatingMul;
use crate::SaturatingNeg;
use crate::SaturatingSub;
use crate::Tof32;
use crate::Tof64;
use crate::WrappingAdd;
use crate::WrappingDiv;
use crate::WrappingMul;
use crate::WrappingNeg;
use crate::WrappingRem;
use crate::WrappingShl;
use crate::WrappingShr;
use crate::WrappingSub;
use crate::Zero;

pub trait Number: Clone + Default + Debug + Display + Copy + One + Zero {}

macro_rules! impl_number {
    ($($t:ty), *) => {
        $(
            impl Number for $t {}
        )*
    };
}
impl_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

pub trait Float: Number {}

macro_rules! impl_float {
    ($($t:ty), *) => {
        $(
            impl Float for $t {}
        )*
    };
}
impl_float!(f32, f64);

#[rustfmt::skip]
pub trait Integer
where
    Self: Number,
    Self: Add<Self, Output = Self> + AddAssign<Self> + CheckedAdd + OverflowingAdd + SaturatingAdd + WrappingAdd,
    Self: Sub<Self, Output = Self> + SubAssign<Self> + CheckedSub + OverflowingSub + SaturatingSub + WrappingSub,
    Self: Mul<Self, Output = Self> + MulAssign<Self> + CheckedMul + OverflowingMul + SaturatingMul + WrappingMul,
    Self: Div<Self, Output = Self> + DivAssign<Self> + CheckedDiv + OverflowingDiv + SaturatingDiv + WrappingDiv,
    Self: Rem<Self, Output = Self> + RemAssign<Self> + CheckedRem + OverflowingRem + WrappingRem,
    Self: CheckedNeg + OverflowingNeg + WrappingNeg,
    Self: CheckedShl + OverflowingShl + WrappingShl,
    Self: CheckedShr + OverflowingShr + WrappingShr,
    Self: PartialEq + Eq + PartialOrd +  Ord,
    Self: Binary,
    Self: Shl<Self, Output = Self>,
    Self: Shl<u8, Output = Self> + Shl<u16, Output = Self> + Shl<u32, Output = Self> + Shl<u64, Output = Self> + Shl<u128, Output = Self> + Shl<usize, Output = Self>,
    Self: ShlAssign<u8> + ShlAssign<u16> + ShlAssign<u32> + ShlAssign<u64> + ShlAssign<u128> + ShlAssign<usize>,
    Self: Shr<Self, Output = Self>,
    Self: Shr<u8, Output = Self> + Shr<u16, Output = Self> + Shr<u32, Output = Self> + Shr<u64, Output = Self> + Shr<u128, Output = Self> + Shr<usize, Output = Self>,
    Self: ShrAssign<u8> + ShrAssign<u16> + ShrAssign<u32> + ShrAssign<u64> + ShrAssign<u128> + ShrAssign<usize>,
    Self: TryFrom<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128> + TryFrom<usize>,
    Self: TryFrom<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64> + TryFrom<i128> + TryFrom<isize>,
    Self: TryInto<u8> + TryInto<u16> + TryInto<u32> + TryInto<u64> + TryInto<u128> + TryInto<usize>,
    Self: TryInto<i8> + TryInto<i16> + TryInto<i32> + TryInto<i64> + TryInto<i128> + TryInto<isize>,
    Self: Tof32 + Tof64,
{
    const BITS: u32;
    const MIN: Self;
    const MAX: Self;
    const SIGNED: bool;
}

macro_rules! impl_integer {
    ($($t:ty), *, $l:literal) => {
        $(
            impl Integer for $t {
                const BITS:u32 = <$t>::BITS;
                const MIN:Self = <$t>::MIN;
                const MAX:Self = <$t>::MAX;
                const SIGNED: bool = $l;
            }
        )*
    };
}
impl_integer!(u8, u16, u32, u64, u128, usize, false);
impl_integer!(i8, i16, i32, i64, i128, isize, true);

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

pub trait Unsigned: Integer {}

macro_rules! impl_unsigned {
    ($($t:ty), *) => {
        $(
            impl Unsigned for $t {}
        )*
    };
}
impl_unsigned!(u8, u16, u32, u64, u128, usize);
