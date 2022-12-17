//!
//!
//!

use std::fmt::Binary;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;

use crate::convert::Tof32;
use crate::convert::Tof64;

use crate::ops::CheckedAdd;
use crate::ops::CheckedDiv;
use crate::ops::CheckedMul;
use crate::ops::CheckedNeg;
use crate::ops::CheckedRem;
use crate::ops::CheckedShl;
use crate::ops::CheckedShr;
use crate::ops::CheckedSub;
use crate::ops::OverflowingAdd;
use crate::ops::OverflowingDiv;
use crate::ops::OverflowingMul;
use crate::ops::OverflowingNeg;
use crate::ops::OverflowingRem;
use crate::ops::OverflowingShl;
use crate::ops::OverflowingShr;
use crate::ops::OverflowingSub;
use crate::ops::SaturatingAdd;
use crate::ops::SaturatingDiv;
use crate::ops::SaturatingMul;
use crate::ops::SaturatingSub;
use crate::ops::WrappingAdd;
use crate::ops::WrappingDiv;
use crate::ops::WrappingMul;
use crate::ops::WrappingNeg;
use crate::ops::WrappingRem;
use crate::ops::WrappingShl;
use crate::ops::WrappingShr;
use crate::ops::WrappingSub;

use super::Number;

#[rustfmt::skip]
pub trait Integer
where
    Self: Number,
    Self: CheckedAdd + OverflowingAdd + SaturatingAdd + WrappingAdd,
    Self: CheckedSub + OverflowingSub + SaturatingSub + WrappingSub,
    Self: CheckedMul + OverflowingMul + SaturatingMul + WrappingMul,
    Self: CheckedDiv + OverflowingDiv + SaturatingDiv + WrappingDiv,
    Self: CheckedRem + OverflowingRem + WrappingRem,
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
