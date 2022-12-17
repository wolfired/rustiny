//!
//!
//!

use std::error::Error;

use rustiny_number::ops::Zero;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

macro_rules! impl_try_from_integer {
    ($($t:ty), *) => {
        $(
            impl<T: Integer, const P: usize> TryFrom<$t> for FixedPoint<T, P>
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

impl<T: Integer, const P: usize> TryFrom<f32> for FixedPoint<T, P> {
    type Error = Box<dyn Error>;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let Ok(value) = ((value * (1 << P) as f32).round() as i128).try_into() else {
            return Err(format!("{} 超出有效范围: [{}, {}]", value, T::MIN, T::MAX).into());
        };
        Ok(Self(value))
    }
}

impl<T: Integer, const P: usize> TryFrom<f64> for FixedPoint<T, P> {
    type Error = Box<dyn Error>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let Ok(value) = ((value * (1 << P) as f64).round() as i128).try_into() else {
            return Err(format!("{} 超出有效范围: [{}, {}]", value, T::MIN, T::MAX).into());
        };
        Ok(Self(value))
    }
}

macro_rules! impl_try_into_integer {
    ($($t:ty), *) => {
        $(
            impl<T: Integer, const P: usize> TryFrom<FixedPoint<T, P>> for $t
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
