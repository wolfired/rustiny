//!
//!
//!

pub trait One {
    fn one() -> Self;
}

macro_rules! impl_one {
    ($($t:ty), *, $l:literal) => {
        $(
            impl One for $t {
                fn one() -> Self {
                    $l
                }
            }
        )*
    };
}
impl_one!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, 1);
impl_one!(f32, f64, 1.0);

pub trait LeadingOnes {
    fn leading_ones(self) -> u32;
}

macro_rules! impl_leading_ones4integer {
    ($($t:ty), *) => {
        $(
            impl LeadingOnes for $t {
                fn leading_ones(self) -> u32 {
                    self.leading_ones()
                }
            }
        )*
    };
}
impl_leading_ones4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait TrailingOnes {
    fn trailing_ones(self) -> u32;
}

macro_rules! impl_trailing_ones4integer {
    ($($t:ty), *) => {
        $(
            impl TrailingOnes for $t {
                fn trailing_ones(self) -> u32 {
                    self.trailing_ones()
                }
            }
        )*
    };
}
impl_trailing_ones4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait CountOnes {
    fn count_ones(self) -> u32;
}

macro_rules! impl_count_ones4integer {
    ($($t:ty), *) => {
        $(
            impl CountOnes for $t {
                fn count_ones(self) -> u32 {
                    self.count_ones()
                }
            }
        )*
    };
}
impl_count_ones4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
