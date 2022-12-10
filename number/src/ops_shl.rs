//!
//!
//!

pub trait CheckedShl<Rhs = Self>
where
    Self: Sized,
{
    fn checked_shl(self, rhs: Rhs) -> Option<Self>;
}

macro_rules! impl_checked_shl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedShl for $t {
                fn checked_shl(self, rhs: Self) -> Option<Self> {
                    self.checked_shl(rhs.try_into().unwrap())
                }
            }
        )*
    };
}
impl_checked_shl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_shl4float {
    ($($t:ty), *) => {
        $(
            impl CheckedShl for $t {
                fn checked_shl(self, rhs: Self) -> Option<Self> {
                    Some(self * (2.0 as $t).powi(rhs as i32))
                }
            }
        )*
    };
}
impl_checked_shl4float!(f32, f64);

pub trait OverflowingShl<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_shl(self, rhs: Rhs) -> (Self, bool);
}

pub trait WrappingShl<Rhs = Self> {
    fn wrapping_shl(self, rhs: Rhs) -> Self;
}
