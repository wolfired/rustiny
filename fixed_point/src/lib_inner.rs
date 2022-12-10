//!
//!
//!

use rustiny_number::Integer;

#[derive(Clone, Default, Copy)]
pub struct FixedPoint<T: Integer, const P: usize>(pub T);
