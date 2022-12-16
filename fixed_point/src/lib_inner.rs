//!
//!
//!

use rustiny_number::Integer;
use rustiny_number::Number;
use rustiny_number::Signed;

#[derive(Clone, Default, Copy)]
pub struct FixedPoint<T: Integer, const P: usize>(pub T);

impl<T: Integer, const P: usize> Number for FixedPoint<T, P> {}

impl<T: Integer, const P: usize> Integer for FixedPoint<T, P> {
    const BITS: u32 = T::BITS;
    const MIN: Self = Self(T::MIN);
    const MAX: Self = Self(T::MAX);
    const SIGNED: bool = T::SIGNED;
}

impl<T: Signed, const P: usize> Signed for FixedPoint<T, P> {}
