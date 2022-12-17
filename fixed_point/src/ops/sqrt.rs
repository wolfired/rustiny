use rustiny_number::ops::CheckedSqrt;
use rustiny_number::types::Integer;

use crate::types::FixedPoint;

impl<T: Integer, const P: usize> CheckedSqrt for FixedPoint<T, P>
where
    T: TryFrom<u128> + TryFrom<i128>,
    T: TryInto<u128> + TryInto<i128>,
{
    fn checked_sqrt(self) -> Option<Self> {
        if T::SIGNED {
            let Ok(value) = TryInto::<i128>::try_into(self.0) else {
                return None
            };
            let Some(value) = (value << P).checked_sqrt() else {
                return None
            };
            let Ok(value) = TryInto::<T>::try_into(value) else {
                return None
            };
            return Some(Self(value));
        } else {
            let Ok(value) = TryInto::<u128>::try_into(self.0) else {
                return None
            };
            let Some(value) = (value << P).checked_sqrt() else {
                return None
            };
            let Ok(value) = TryInto::<T>::try_into(value) else {
                return None
            };
            return Some(Self(value));
        }
    }
}
