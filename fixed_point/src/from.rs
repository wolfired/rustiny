//!
//!
//!

use std::cmp::Ord;
use std::error::Error;
use std::ops::Shl;
use std::ops::Shr;

use rustiny_number::Integer;
use rustiny_number::Zero;

use crate::FixedPoint;

macro_rules! impl_try_from_integer {
    ($($t:ty), *) => {
        $(
            impl<T: Integer, const P: usize> TryFrom<$t> for FixedPoint<T, P>
            where
                T: TryFrom<$t>,
                T: Ord,
                T: Shl<usize, Output = T>,
                T: Shr<usize, Output = T>,
                T: Zero,
            {
                type Error = Box<dyn Error>;

                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    let Ok(value) = value.try_into() else {
                        return Err(format!("{} 超出有效范围: [{}, {}]", value, <$t>::MIN, <$t>::MAX).into());
                    };

                    if T::zero() == value {
                        return Ok(Self(value));
                    }

                    let min = T::MIN >> P;
                    let max = T::MAX >> P;

                    if value < min || max < value {
                        return Err(format!("{} 超出有效范围: [{}, {}]", value, min, max).into());
                    }

                    Ok(Self(value << P))
                }
            }
        )*
    }
}
impl_try_from_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<T: Integer, const P: usize> TryFrom<f32> for FixedPoint<T, P>
where
    T: TryFrom<i128>,
{
    type Error = Box<dyn Error>;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let Ok(value) = ((value * (1 << P) as f32).round() as i128).try_into() else {
            return Err(format!("{} 超出有效范围: [{}, {}]", value, T::MIN, T::MAX).into());
        };
        Ok(Self(value))
    }
}

impl<T: Integer, const P: usize> TryFrom<f64> for FixedPoint<T, P>
where
    T: TryFrom<i128>,
{
    type Error = Box<dyn Error>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let Ok(value) = ((value * (1 << P) as f64).round() as i128).try_into() else {
            return Err(format!("{} 超出有效范围: [{}, {}]", value, T::MIN, T::MAX).into());
        };
        Ok(Self(value))
    }
}
