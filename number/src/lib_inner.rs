//!
//!
//!

use std::fmt::Debug;
use std::fmt::Display;

pub trait Number: Clone + Default + Debug + Display + Copy {}

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

pub trait Integer: Number {
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

pub trait Signed: Integer {}

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
