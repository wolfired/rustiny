//!
//!
//!

pub trait CheckedShr<Rhs = u32>
where
    Self: Sized,
{
    fn checked_shr(self, rhs: Rhs) -> Option<Self>;
}

pub trait OverflowingShr<Rhs = u32>
where
    Self: Sized,
{
    fn overflowing_shr(self, rhs: Rhs) -> (Self, bool);
}

pub trait WrappingShr<Rhs = u32> {
    fn wrapping_shr(self, rhs: Rhs) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedShr for $t {
                fn checked_shr(self, rhs: u32) -> Option<Self> {
                    self.checked_shr(rhs)
                }
            }

            impl OverflowingShr for $t {
                fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                    self.overflowing_shr(rhs)
                }
            }

            impl WrappingShr for $t {
                fn wrapping_shr(self, rhs: u32) -> Self {
                    self.wrapping_shr(rhs)
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
