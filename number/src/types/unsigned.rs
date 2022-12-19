//!
//!
//!

use super::Integer;

pub trait Unsigned: Integer {}

macro_rules! impl_unsigned {
    ($($t:ty), *) => {
        $(
            impl Unsigned for $t {}
        )*
    };
}
impl_unsigned!(u8, u16, u32, u64, u128, usize);

pub trait U8: Unsigned {}
impl U8 for u8 {}

pub trait U16: Unsigned {}
impl U16 for u16 {}

pub trait U32: Unsigned {}
impl U32 for u32 {}

pub trait U64: Unsigned {}
impl U64 for u64 {}

pub trait U128: Unsigned {}
impl U128 for u128 {}

pub trait USize: Unsigned {}
impl USize for usize {}
