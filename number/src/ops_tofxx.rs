//!
//!
//!

pub trait Tof32 {
    fn to_f32(self) -> f32;
}

macro_rules! impl_to_f32 {
    ($($t:ty), *) => {
        $(
            impl Tof32 for $t {
                fn to_f32(self) -> f32 {
                    self as f32
                }
            }
        )*
    };
}
impl_to_f32!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait Tof64 {
    fn to_f64(self) -> f64;
}

macro_rules! impl_to_f64 {
    ($($t:ty), *) => {
        $(
            impl Tof64 for $t {
                fn to_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}
impl_to_f64!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
