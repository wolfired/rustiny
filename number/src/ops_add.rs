//!
//!
//!

pub trait CheckedAdd<Rhs = Self>
where
    Self: Sized,
{
    fn checked_add(self, rhs: Rhs) -> Option<Self>;
}

macro_rules! impl_checked_add4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedAdd for $t {
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    self.checked_add(rhs)
                }
            }
        )*
    };
}
impl_checked_add4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_add4float {
    ($($t:ty), *) => {
        $(
            impl CheckedAdd for $t {
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    Some(self + rhs)
                }
            }
        )*
    };
}
impl_checked_add4float!(f32, f64);

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
