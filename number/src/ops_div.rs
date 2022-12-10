//!
//!
//!

pub trait CheckedDiv<Rhs = Self>
where
    Self: Sized,
{
    fn checked_div(self, rhs: Rhs) -> Option<Self>;
}

macro_rules! impl_checked_div4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedDiv for $t {
                fn checked_div(self, rhs: Self) -> Option<Self> {
                    self.checked_div(rhs)
                }
            }
        )*
    };
}
impl_checked_div4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_div4float {
    ($($t:ty), *) => {
        $(
            impl CheckedDiv for $t {
                fn checked_div(self, rhs: Self) -> Option<Self> {
                    if 0.0 == rhs {
                        None
                    } else {
                        Some(self * rhs)
                    }
                }
            }
        )*
    };
}
impl_checked_div4float!(f32, f64);

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
