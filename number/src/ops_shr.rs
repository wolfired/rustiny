//!
//!
//!

pub trait CheckedShr<Rhs = Self>
where
    Self: Sized,
{
    fn checked_shr(self, rhs: Rhs) -> Option<Self>;
}

macro_rules! impl_checked_shr4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedShr for $t {
                fn checked_shr(self, rhs: Self) -> Option<Self> {
                    self.checked_shr(rhs.try_into().unwrap())
                }
            }
        )*
    };
}
impl_checked_shr4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_shr4float {
    ($($t:ty), *) => {
        $(
            impl CheckedShr for $t {
                fn checked_shr(self, rhs: Self) -> Option<Self> {
                    Some(self / (2.0 as $t).powi(rhs as i32))
                }
            }
        )*
    };
}
impl_checked_shr4float!(f32, f64);

pub trait OverflowingShr<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_shr(self, rhs: Rhs) -> (Self, bool);
}

pub trait WrappingShr<Rhs = Self> {
    fn wrapping_shr(self, rhs: Rhs) -> Self;
}
