//!
//!
//!

pub trait CheckedDiv<Rhs = Self>
where
    Self: Sized,
{
    fn checked_div(self, rhs: Rhs) -> Option<Self>;
}

pub trait OverflowingDiv<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_div(self, rhs: Rhs) -> (Self, bool);
}

pub trait SaturatingDiv<Rhs = Self> {
    fn saturating_div(self, rhs: Rhs) -> Self;
}

pub trait WrappingDiv<Rhs = Self> {
    fn wrapping_div(self, rhs: Rhs) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedDiv for $t {
                fn checked_div(self, rhs: Self) -> Option<Self> {
                    self.checked_div(rhs)
                }
            }

            impl OverflowingDiv for $t {
                fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                    self.overflowing_div(rhs)
                }
            }

            impl SaturatingDiv for $t {
                fn saturating_div(self, rhs: Self) -> Self {
                    self.saturating_div(rhs)
                }
            }

            impl WrappingDiv for $t {
                fn wrapping_div(self, rhs: Self) -> Self {
                    self.wrapping_div(rhs)
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
