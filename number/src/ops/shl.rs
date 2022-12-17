//!
//!
//!

pub trait CheckedShl<Rhs = u32>
where
    Self: Sized,
{
    fn checked_shl(self, rhs: Rhs) -> Option<Self>;
}

pub trait OverflowingShl<Rhs = u32>
where
    Self: Sized,
{
    fn overflowing_shl(self, rhs: Rhs) -> (Self, bool);
}

pub trait WrappingShl<Rhs = u32> {
    fn wrapping_shl(self, rhs: Rhs) -> Self;
}

macro_rules! impl4integer {
    ($($t:ty), *) => {
        $(
            impl CheckedShl for $t {
                fn checked_shl(self, rhs: u32) -> Option<Self> {
                    self.checked_shl(rhs)
                }
            }

            impl OverflowingShl for $t {
                fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                    self.overflowing_shl(rhs)
                }
            }

            impl WrappingShl for $t {
                fn wrapping_shl(self, rhs: u32) -> Self {
                    self.wrapping_shl(rhs)
                }
            }
        )*
    };
}
impl4integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
