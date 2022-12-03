use std::fmt::Debug;
use std::fmt::Display;

pub trait BaseNumber: Clone + Default + Debug + Display + Copy {}

macro_rules! impl_base_number {
    ($($t:ty),*) => {
        $(
            impl BaseNumber for $t {}
        )*
    }
}
impl_base_number!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

pub struct BaseNumberWrapper<T: BaseNumber>(T);

impl<T: BaseNumber> From<T> for BaseNumberWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

macro_rules! impl_from_number_wrapper {
    ($($t:ty),*) => {
        $(
            impl From<BaseNumberWrapper<$t>> for $t {
                fn from(value: BaseNumberWrapper<$t>) -> Self {
                    value.0
                }
            }
        )*
    }
}
impl_from_number_wrapper!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

pub trait BaseFloat: BaseNumber {}

macro_rules! impl_base_float {
    ($($t:ty),*) => {
        $(
            impl BaseFloat for $t {}
        )*
    }
}
impl_base_float!(f32, f64);

pub struct BaseFloatWrapper<T: BaseFloat>(T);

impl<T: BaseFloat> From<T> for BaseFloatWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

macro_rules! impl_from_float_wrapper {
    ($($t:ty),*) => {
        $(
            impl From<BaseFloatWrapper<$t>> for $t {
                fn from(value: BaseFloatWrapper<$t>) -> Self {
                    value.0
                }
            }
        )*
    }
}
impl_from_float_wrapper!(f32, f64);

pub trait Expend<T> {
    fn expend(self) -> T;
}

pub trait Shrink<T> {
    fn shrink(self) -> T;
}

pub trait BaseInteger: BaseNumber {}

macro_rules! impl_base_integer {
    ($(($t0:ty, $t1:ty)),*) => {
        $(
            impl BaseInteger for $t0 {}

            impl Expend<$t1> for $t0 {
                fn expend(self) -> $t1 {
                    self as $t1
                }
            }

            impl Shrink<$t0> for $t1 {
                fn shrink(self) -> $t0 {
                    self as $t0
                }
            }
        )*
    };
}
impl_base_integer!(
    (u8, u16),
    (i8, i16),
    (u16, u32),
    (i16, i32),
    (u32, u64),
    (i32, i64),
    (u64, u128),
    (i64, i128),
    (u128, u128),
    (i128, i128),
    (usize, u128),
    (isize, i128)
);

pub struct BaseIntegerWrapper<T: BaseInteger>(T);

impl<T: BaseInteger> From<T> for BaseIntegerWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

macro_rules! impl_from_integer_wrapper {
    ($($t:ty),*) => {
        $(
            impl From<BaseIntegerWrapper<$t>> for $t {
                fn from(value: BaseIntegerWrapper<$t>) -> Self {
                    value.0
                }
            }
        )*
    }
}
impl_from_integer_wrapper!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

pub trait Number: Clone + Default + Debug + Display + Copy {
    const TYPE_STR: &'static str;
    const NUM_MAX: Self;
    const NUM_MIN: Self;
    const NUM_0: Self;
    const NUM_1: Self;
    const TOTAL_BITS: u32;
    const VALID_BITS: u32;

    fn num_abs(self) -> Self;
    fn num_sqrt(self) -> Self;
}

macro_rules! impl_number_u {
    ($t:ty, $e0:expr) => {
        impl Number for $t {
            const TYPE_STR: &'static str = $e0;
            const NUM_MAX: Self = <$t>::MAX;
            const NUM_MIN: Self = <$t>::MIN;
            const NUM_0: Self = 0 as $t;
            const NUM_1: Self = 1 as $t;
            const TOTAL_BITS: u32 = <$t>::BITS;
            const VALID_BITS: u32 = <$t>::BITS;

            fn num_abs(self) -> Self {
                self
            }

            fn num_sqrt(mut self) -> Self {
                let mut result = 0;

                let mut halt_bits = (Self::TOTAL_BITS - self.leading_zeros() + 1) >> 1;

                while 0 < self && 0 < halt_bits {
                    halt_bits -= 1;
                    let middle = ((result << 1) + (1 << halt_bits)) << halt_bits;
                    if middle <= self {
                        result += 1 << halt_bits;
                        self -= middle;
                    }
                }

                result
            }
        }
    };
}
impl_number_u!(u8, "u8");
impl_number_u!(u16, "u16");
impl_number_u!(u32, "u32");
impl_number_u!(u64, "u64");
impl_number_u!(u128, "u128");
impl_number_u!(usize, "usize");

macro_rules! impl_number_i {
    ($t:ty, $e0:expr) => {
        impl Number for $t {
            const TYPE_STR: &'static str = $e0;
            const NUM_MAX: Self = <$t>::MAX;
            const NUM_MIN: Self = <$t>::MIN;
            const NUM_0: Self = 0 as $t;
            const NUM_1: Self = 1 as $t;
            const TOTAL_BITS: u32 = <$t>::BITS;
            const VALID_BITS: u32 = <$t>::BITS - 1;

            fn num_abs(self) -> Self {
                self.abs()
            }
            fn num_sqrt(mut self) -> Self {
                let mut result = 0;

                let mut halt_bits = (Self::TOTAL_BITS - self.leading_zeros() + 1) >> 1;

                while 0 < self && 0 < halt_bits {
                    halt_bits -= 1;
                    let middle = ((result << 1) + (1 << halt_bits)) << halt_bits;
                    if middle <= self {
                        result += 1 << halt_bits;
                        self -= middle;
                    }
                }

                result
            }
        }
    };
}
impl_number_i!(i8, "i8");
impl_number_i!(i16, "i16");
impl_number_i!(i32, "i32");
impl_number_i!(i64, "i64");
impl_number_i!(i128, "i128");
impl_number_i!(isize, "isize");

macro_rules! impl_number_f {
    ($t:ty, $e0:expr, $e1:expr, $e2:expr) => {
        impl Number for $t {
            const TYPE_STR: &'static str = $e0;
            const NUM_MAX: Self = <$t>::MAX;
            const NUM_MIN: Self = <$t>::MIN;
            const NUM_0: Self = 0 as $t;
            const NUM_1: Self = 1 as $t;
            const TOTAL_BITS: u32 = $e1;
            const VALID_BITS: u32 = $e2;

            fn num_abs(self) -> Self {
                self.abs()
            }
            fn num_sqrt(self) -> Self {
                self.sqrt()
            }
        }
    };
}
impl_number_f!(f32, "f32", 32, 31);
impl_number_f!(f64, "f64", 64, 63);

pub trait Float: Number {
    fn to_i128(self) -> i128;
}

macro_rules! impl_float {
    ($($t:ty),*) => {
        $(
            impl Float for $t {
                fn to_i128(self) -> i128 { self.round() as i128 }
            }
        )*
    }
}
impl_float!(f32, f64);

pub trait Integer: Number {
    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;
}

macro_rules! impl_integer {
    ($($t:ty),*) => {
        $(
            impl Integer for $t {
                fn to_f32(self) -> f32 { self as f32 }
                fn to_f64(self) -> f64 { self as f64 }
            }
        )*
    }
}
impl_integer!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
