//!
//!
//!

pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_zero {
    ($($t:ty), *, $l:literal) => {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    $l
                }
            }
        )*
    };
}
impl_zero!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, 0);
impl_zero!(f32, f64, 0.0);

pub trait LeadingZeros {
    fn leading_zeros(self) -> u32;
}

macro_rules! impl_leading_zeros4integer {
    ($($t:ty), *) => {
        $(
            impl LeadingZeros for $t {
                fn leading_zeros(self) -> u32 {
                    self.leading_zeros()
                }
            }
        )*
    };
}
impl_leading_zeros4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait TrailingZeros {
    fn trailing_zeros(self) -> u32;
}

macro_rules! impl_trailing_zeros4integer {
    ($($t:ty), *) => {
        $(
            impl TrailingZeros for $t {
                fn trailing_zeros(self) -> u32 {
                    self.trailing_zeros()
                }
            }
        )*
    };
}
impl_trailing_zeros4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait CountZeros {
    fn count_zeros(self) -> u32;
}

macro_rules! impl_count_zeros4integer {
    ($($t:ty), *) => {
        $(
            impl CountZeros for $t {
                fn count_zeros(self) -> u32 {
                    self.count_zeros()
                }
            }
        )*
    };
}
impl_count_zeros4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

