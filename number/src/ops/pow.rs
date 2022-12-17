//!
//!
//!

pub trait Pow<Rhs = u32> {
    fn pow(self, exp: Rhs) -> Self;
}

pub trait CheckedPow<Rhs = u32>
where
    Self: Sized,
{
    fn checked_pow(self, exp: Rhs) -> Option<Self>;
}

pub trait OverflowingPow<Rhs = u32>
where
    Self: Sized,
{
    fn overflowing_pow(self, exp: Rhs) -> (Self, bool);
}

pub trait SaturatingPow<Rhs = u32> {
    fn saturating_pow(self, exp: Rhs) -> Self;
}

pub trait WrappingPow<Rhs = u32> {
    fn wrapping_pow(self, exp: Rhs) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl Pow for $t {
                fn pow(self, exp: u32) -> Self {
                    self.pow(exp)
                }
            }

            impl CheckedPow for $t {
                fn checked_pow(self, exp: u32) -> Option<Self> {
                    self.checked_pow(exp)
                }
            }

            impl OverflowingPow for $t {
                fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                    self.overflowing_pow(exp)
                }
            }

            impl SaturatingPow for $t {
                fn saturating_pow(self, exp: u32) -> Self {
                    self.saturating_pow(exp)
                }
            }

            impl WrappingPow for $t {
                fn wrapping_pow(self, exp: u32) -> Self {
                    self.wrapping_pow(exp)
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
