//!
//!
//!

pub trait CheckedMul<Rhs = Self>
where
    Self: Sized,
{
    fn checked_mul(self, rhs: Rhs) -> Option<Self>;
}

pub trait OverflowingMul<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_mul(self, rhs: Rhs) -> (Self, bool);
}

pub trait SaturatingMul<Rhs = Self> {
    fn saturating_mul(self, rhs: Rhs) -> Self;
}

pub trait WrappingMul<Rhs = Self> {
    fn wrapping_mul(self, rhs: Rhs) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedMul for $t {
                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    self.checked_mul(rhs)
                }
            }

            impl OverflowingMul for $t {
                fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                    self.overflowing_mul(rhs)
                }
            }

            impl SaturatingMul for $t {
                fn saturating_mul(self, rhs: Self) -> Self {
                    self.saturating_mul(rhs)
                }
            }

            impl WrappingMul for $t {
                fn wrapping_mul(self, rhs: Self) -> Self {
                    self.wrapping_mul(rhs)
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
