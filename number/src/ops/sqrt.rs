//!
//!
//!

pub trait CheckedSqrt
where
    Self: Sized,
{
    fn checked_sqrt(self) -> Option<Self>;
}

macro_rules! impl_checked_sqrt4unsigned {
    ($($t:ty), *) => {
        $(
            impl CheckedSqrt for $t where Self: Sized {
                fn checked_sqrt(mut self) -> Option<Self> {
                    let mut result = 0;
                    let mut halt_bits = (<$t>::BITS - self.leading_zeros() + 1) >> 1;
                    while 0 < self && 0 < halt_bits {
                        halt_bits -= 1;
                        let middle = ((result << 1) + (1 << halt_bits)) << halt_bits;
                        if middle <= self {
                            result += 1 << halt_bits;
                            self -= middle;
                        }
                    }
                    Some(result)
                }
            }
        )*
    };
}
impl_checked_sqrt4unsigned!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_checked_sqrt4signed {
    ($($t:ty), *) => {
        $(
            impl CheckedSqrt for $t where Self: Sized {
                fn checked_sqrt(mut self) -> Option<Self> {
                    if 0 > self {
                        None
                    } else {
                        let mut result = 0;
                        let mut halt_bits = (<$t>::BITS - self.leading_zeros() + 1) >> 1;
                        while 0 < self && 0 < halt_bits {
                            halt_bits -= 1;
                            let middle = ((result << 1) + (1 << halt_bits)) << halt_bits;
                            if middle <= self {
                                result += 1 << halt_bits;
                                self -= middle;
                            }
                        }
                        Some(result)
                    }
                }
            }
        )*
    };
}
impl_checked_sqrt4signed!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_checked_sqrt4float {
    ($($t:ty), *) => {
        $(
            impl CheckedSqrt for $t where Self: Sized {
                fn checked_sqrt(self) -> Option<Self> {
                    if 0.0 > self {
                        None
                    } else if 0.0 < self {
                        Some(self.sqrt())
                    } else {
                        Some(0.0)
                    }
                }
            }
        )*
    }
}
impl_checked_sqrt4float!(f32, f64);
