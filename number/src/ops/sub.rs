//!
//!
//!

pub trait CheckedSub<Rhs = Self>
where
    Self: Sized,
{
    fn checked_sub(self, rhs: Rhs) -> Option<Self>;
}

pub trait OverflowingSub<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_sub(self, rhs: Rhs) -> (Self, bool);
}

pub trait SaturatingSub<Rhs = Self> {
    fn saturating_sub(self, rhs: Rhs) -> Self;
}

pub trait WrappingSub<Rhs = Self> {
    fn wrapping_sub(self, rhs: Rhs) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedSub for $t {
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    self.checked_sub(rhs)
                }
            }

            impl OverflowingSub for $t {
                fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                    self.overflowing_sub(rhs)
                }
            }

            impl SaturatingSub for $t {
                fn saturating_sub(self, rhs: Self) -> Self {
                    self.saturating_sub(rhs)
                }
            }

            impl WrappingSub for $t {
                fn wrapping_sub(self, rhs: Self) -> Self {
                    self.wrapping_sub(rhs)
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
