//!
//!
//!

pub trait CheckedNeg
where
    Self: Sized,
{
    fn checked_neg(self) -> Option<Self>;
}

macro_rules! impl_checked_neg4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedNeg for $t {
                fn checked_neg(self) -> Option<Self> {
                    self.checked_neg()
                }
            }
        )*
    };
}
impl_checked_neg4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_neg4float {
    ($($t:ty), *) => {
        $(
            impl CheckedNeg for $t {
                fn checked_neg(self) -> Option<Self> {
                    if 0.0 == self {
                        Some(0.0)
                    } else {
                        Some(-self)
                    }
                }
            }
        )*
    };
}
impl_checked_neg4float!(f32, f64);

pub trait OverflowingNeg
where
    Self: Sized,
{
    fn overflowing_neg(self) -> (Self, bool);
}

pub trait SaturatingNeg {
    fn saturating_neg(self) -> Self;
}

pub trait WrappingNeg {
    fn wrapping_neg(self) -> Self;
}

macro_rules! impl_wrapping_neg4integer {
    ($($t:ty), *) => {
        $(
            impl WrappingNeg for $t {
                fn wrapping_neg(self) -> Self {
                    self.wrapping_neg()
                }
            }
        )*
    };
}
impl_wrapping_neg4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
