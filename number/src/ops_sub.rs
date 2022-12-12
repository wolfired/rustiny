//!
//!
//!

pub trait CheckedSub<Rhs = Self>
where
    Self: Sized,
{
    fn checked_sub(self, rhs: Rhs) -> Option<Self>;
}

macro_rules! impl_checked_sub4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedSub for $t {
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    self.checked_sub(rhs)
                }
            }
        )*
    };
}
impl_checked_sub4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_sub4float {
    ($($t:ty), *) => {
        $(
            impl CheckedSub for $t {
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    Some(self - rhs)
                }
            }
        )*
    };
}
impl_checked_sub4float!(f32, f64);

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

macro_rules! impl_wrapping_sub4integer {
    ($($t:ty), *) => {
        $(
            impl WrappingSub for $t {
                fn wrapping_sub(self, rhs: Self) -> Self {
                    self.wrapping_sub(rhs)
                }
            }
        )*
    };
}
impl_wrapping_sub4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
