//!
//!
//!

pub trait CheckedAdd<Rhs = Self>
where
    Self: Sized,
{
    fn checked_add(self, rhs: Rhs) -> Option<Self>;
}

pub trait OverflowingAdd<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_add(self, rhs: Rhs) -> (Self, bool);
}

pub trait SaturatingAdd<Rhs = Self> {
    fn saturating_add(self, rhs: Rhs) -> Self;
}

pub trait WrappingAdd<Rhs = Self> {
    fn wrapping_add(self, rhs: Rhs) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedAdd for $t {
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    self.checked_add(rhs)
                }
            }

            impl OverflowingAdd for $t {
                fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                    self.overflowing_add(rhs)
                }
            }

            impl SaturatingAdd for $t {
                fn saturating_add(self, rhs: Self) -> Self {
                    self.saturating_add(rhs)
                }
            }

            impl WrappingAdd for $t {
                fn wrapping_add(self, rhs: Self) -> Self {
                    self.wrapping_add(rhs)
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
