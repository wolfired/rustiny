//!
//!
//!

use rustiny_number::Integer;
use rustiny_number::Tof32;
use rustiny_number::Tof64;

use crate::FixedPoint;

impl<T: Integer, const P: usize> Tof32 for FixedPoint<T, P>
where
    Self: Into<f32>,
{
    fn to_f32(self) -> f32 {
        self.into()
    }
}

impl<T: Integer, const P: usize> Tof64 for FixedPoint<T, P>
where
    Self: Into<f64>,
{
    fn to_f64(self) -> f64 {
        self.into()
    }
}
