//!
//!
//!

use std::cmp::Ord;
use std::error::Error;
use std::fmt::Display;
use std::ops::Shl;
use std::ops::ShrAssign;
use std::ops::SubAssign;

use rustiny_number::CheckedAdd;
use rustiny_number::Integer;
use rustiny_number::One;
use rustiny_number::Tof32;
use rustiny_number::Tof64;
use rustiny_number::Zero;

use crate::FixedPoint;

macro_rules! impl_try_into_integer {
    ($($t:ty), *) => {
        $(
            impl<T: Integer, const P: usize> TryFrom<FixedPoint<T, P>> for $t
            where
                T: TryInto<$t>,
                T: Shl<usize, Output = T>,
                T: ShrAssign<usize>,
                T: Display,
                T: Ord,
                T: SubAssign<T>,
                T: One + Zero,
                T: CheckedAdd,
            {
                type Error = Box<dyn Error>;

                fn try_from(value: FixedPoint<T, P>) -> Result<Self, Self::Error> {
                    if T::SIGNED {
                        if (T::BITS - 1) as usize <= P {
                            return Ok(<$t>::zero());
                        }
                    } else {
                        if T::BITS as usize <= P {
                            return Ok(<$t>::zero());
                        }
                    }

                    if 0 == P {
                        let Ok(value) = value.0.try_into() else {
                            return Err(format!("{} 超出有效范围: [{}, {}]", value.0, <$t>::MIN, <$t>::MAX).into());
                        };
                        return Ok(value);
                    }

                    let zero_point_five = T::one() << (P - 1);

                    let Some(mut value_added_zpf) = value.0.checked_add(zero_point_five) else {
                        return Err(format!("{} 超出有效范围: [{}, {}]", value.0, <$t>::MIN, <$t>::MAX).into());
                    };

                    if T::SIGNED {
                        if T::zero() > value.0 {
                            value_added_zpf -= T::one();
                        }
                    }

                    value_added_zpf >>= P;

                    let Ok(value) = (value_added_zpf).try_into() else {
                        return Err(format!("{} 超出有效范围: [{}, {}]", value_added_zpf, <$t>::MIN, <$t>::MAX).into());
                    };

                    return Ok(value);
                }
            }
        )*
    }
}
impl_try_into_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl<T: Integer, const P: usize> From<FixedPoint<T, P>> for f32
where
    T: Tof32,
{
    fn from(value: FixedPoint<T, P>) -> Self {
        value.0.to_f32() / (1 << P) as f32
    }
}

impl<T: Integer, const P: usize> From<FixedPoint<T, P>> for f64
where
    T: Tof64,
{
    fn from(value: FixedPoint<T, P>) -> Self {
        value.0.to_f64() / (1 << P) as f64
    }
}
