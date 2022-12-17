//!
//!
//!

pub trait CheckedNeg
where
    Self: Sized,
{
    fn checked_neg(self) -> Option<Self>;
}

pub trait OverflowingNeg
where
    Self: Sized,
{
    fn overflowing_neg(self) -> (Self, bool);
}

pub trait WrappingNeg {
    fn wrapping_neg(self) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedNeg for $t {
                fn checked_neg(self) -> Option<Self> {
                    self.checked_neg()
                }
            }

            impl OverflowingNeg for $t {
                fn overflowing_neg(self) -> (Self, bool) {
                    self.overflowing_neg()
                }
            }

            impl WrappingNeg for $t {
                fn wrapping_neg(self) -> Self {
                    self.wrapping_neg()
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

pub trait SaturatingNeg {
    fn saturating_neg(self) -> Self;
}

macro_rules! impl4signed {
    ($($t:ty), *) => {
        $(
            impl SaturatingNeg for $t {
                fn saturating_neg(self) -> Self {
                    self.saturating_neg()
                }
            }
        )*
    };
}
impl4signed!(i8, i16, i32, i64, i128, isize);
