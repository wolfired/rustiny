//!
//!
//!

use std::ops::Neg;

use super::Number;

pub trait Float
where
    Self: Number,
    Self: Neg<Output = Self>,
{
}

macro_rules! impl_float {
    ($($t:ty), *) => {
        $(
            impl Float for $t {}
        )*
    };
}
impl_float!(f32, f64);
