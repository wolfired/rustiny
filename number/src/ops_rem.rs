//!
//!
//!

pub trait CheckedRem<Rhs = Self>
where
    Self: Sized,
{
    fn checked_rem(self, rhs: Rhs) -> Option<Self>;
}

macro_rules! impl_checked_rem4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedRem for $t {
                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    self.checked_rem(rhs)
                }
            }
        )*
    };
}
impl_checked_rem4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_rem4float {
    ($($t:ty), *) => {
        $(
            impl CheckedRem for $t {
                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    if 0.0 == rhs {
                        None
                    } else {
                        Some(self % rhs)
                    }
                }
            }
        )*
    };
}
impl_checked_rem4float!(f32, f64);

pub trait OverflowingRem<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_rem(self, rhs: Rhs) -> (Self, bool);
}

pub trait WrappingRem<Rhs = Self> {
    fn wrapping_rem(self, rhs: Rhs) -> Self;
}
