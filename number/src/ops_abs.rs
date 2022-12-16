//!
//!
//!

pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

pub trait CheckedAbs
where
    Self: Sized,
{
    fn checked_abs(self) -> Option<Self>;
}

pub trait OverflowingAbs
where
    Self: Sized,
{
    fn overflowing_abs(self) -> (Self, bool);
}

pub trait SaturatingAbs {
    fn saturating_abs(self) -> Self;
}

pub trait WrappingAbs {
    fn wrapping_abs(self) -> Self;
}

macro_rules! impl4signed {
    ($($t:ty), *) => {
        $(
            impl Abs for $t {
                type Output = Self;

                fn abs(self) -> Self::Output {
                    self.abs()
                }
            }

            impl CheckedAbs for $t {
                fn checked_abs(self) -> Option<Self> {
                    self.checked_abs()
                }
            }

            impl OverflowingAbs for $t {
                fn overflowing_abs(self) -> (Self, bool) {
                    self.overflowing_abs()
                }
            }

            impl SaturatingAbs for $t {
                fn saturating_abs(self) -> Self {
                    self.saturating_abs()
                }
            }

            impl WrappingAbs for $t {
                fn wrapping_abs(self) -> Self {
                    self.wrapping_abs()
                }
            }
        )*
    };
}
impl4signed!(i8, i16, i32, i64, i128, isize);
