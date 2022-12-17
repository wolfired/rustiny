//!
//!
//!

pub trait CheckedRem<Rhs = Self>
where
    Self: Sized,
{
    fn checked_rem(self, rhs: Rhs) -> Option<Self>;
}

pub trait OverflowingRem<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_rem(self, rhs: Rhs) -> (Self, bool);
}

pub trait WrappingRem<Rhs = Self> {
    fn wrapping_rem(self, rhs: Rhs) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedRem for $t {
                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    self.checked_rem(rhs)
                }
            }

            impl OverflowingRem for $t {
                fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                    self.overflowing_rem(rhs)
                }
            }

            impl WrappingRem for $t {
                fn wrapping_rem(self, rhs: Self) -> Self {
                    self.wrapping_rem(rhs)
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
