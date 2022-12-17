//!
//!
//!

use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::ops::CheckedSqrt;
use crate::ops::One;
use crate::ops::Zero;

pub trait Number
where
    Self: Clone + Default + Debug + Display + Copy + One + Zero,
    Self: Add<Self, Output = Self> + AddAssign<Self>,
    Self: Sub<Self, Output = Self> + SubAssign<Self>,
    Self: Mul<Self, Output = Self> + MulAssign<Self>,
    Self: Div<Self, Output = Self> + DivAssign<Self>,
    Self: Rem<Self, Output = Self> + RemAssign<Self>,
    Self: CheckedSqrt,
{
}

macro_rules! impl_number {
    ($($t:ty), *) => {
        $(
            impl Number for $t {}
        )*
    };
}
impl_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
