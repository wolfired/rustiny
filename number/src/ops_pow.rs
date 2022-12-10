//!
//!
//!

pub trait Pow<Rhs = Self> {
    fn pow(self, exp: Rhs) -> Self;
}

macro_rules! impl_pow4integer {
    ($($t:ty), *) => {
        $(
            impl Pow for $t {
                fn pow(self, rhs: Self) -> Self {
                    self.pow(rhs.try_into().unwrap())
                }
            }
        )*
    };
}
impl_pow4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_pow4float {
    ($($t:ty), *) => {
        $(
            impl Pow for $t {
                fn pow(self, rhs: Self) -> Self {
                    self.powi(rhs as i32)
                }
            }
        )*
    };
}
impl_pow4float!(f32, f64);

pub trait CheckedPow<Rhs = Self>
where
    Self: Sized,
{
    fn checked_pow(self, rhs: Rhs) -> Option<Self>;
}

macro_rules! impl_checked_pow4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedPow for $t {
                fn checked_pow(self, rhs: Self) -> Option<Self> {
                    self.checked_pow(rhs.try_into().unwrap())
                }
            }
        )*
    };
}
impl_checked_pow4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_pow4float {
    ($($t:ty), *) => {
        $(
            impl CheckedPow for $t {
                fn checked_pow(self, rhs: Self) -> Option<Self> {
                    Some(self.powi(rhs as i32))
                }
            }
        )*
    };
}
impl_checked_pow4float!(f32, f64);

pub trait OverflowingPow<Rhs = Self>
where
    Self: Sized,
{
    fn overflowing_pow(self, rhs: Rhs) -> (Self, bool);
}

pub trait SaturatingPow<Rhs = Self> {
    fn saturating_pow(self, rhs: Rhs) -> Self;
}

pub trait WrappingPow<Rhs = Self> {
    fn wrapping_pow(self, rhs: Rhs) -> Self;
}
