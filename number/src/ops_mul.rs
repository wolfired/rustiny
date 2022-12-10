//!
//!
//!

pub trait CheckedMul<Rhs = Self>
where
    Self: Sized,
{
    fn checked_mul(self, rhs: Rhs) -> Option<Self>;
}

macro_rules! impl_checked_mul4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedMul for $t {
                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    self.checked_mul(rhs)
                }
            }
        )*
    };
}
impl_checked_mul4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_mul4float {
    ($($t:ty), *) => {
        $(
            impl CheckedMul for $t {
                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    Some(self * rhs)
                }
            }
        )*
    };
}
impl_checked_mul4float!(f32, f64);

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
