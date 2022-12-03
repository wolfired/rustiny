#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;

use rustiny_computer_graphic::{X, Y, Z, W};
use rustiny_computer_graphic::{line_bresenham, Point};
use rustiny_fixed_point::FixedPoint;
use rustiny_linear_algebra::Matrix;
use rustiny_number::BaseIntegerWrapper;

fn main() -> Result<(), Box<dyn Error>> {
    let x0 = TryInto::<FixedPoint<i16, 8>>::try_into(0.0)?;
    let y0 = TryInto::<FixedPoint<i16, 8>>::try_into(10.0)?;

    let x1 = TryInto::<FixedPoint<i16, 8>>::try_into(10.0)?;
    let y1 = TryInto::<FixedPoint<i16, 8>>::try_into(0.0)?;

    line_bresenham(x0, y0, x1, y1, |x, y| {
        println!("{} {}", x, y);
    });

    let p: Point<FixedPoint<i8, 4>, 4> = [
        TryInto::<FixedPoint<i8, 4>>::try_into(Into::<BaseIntegerWrapper<_>>::into(1))?,
        TryInto::<FixedPoint<i8, 4>>::try_into(Into::<BaseIntegerWrapper<_>>::into(2))?,
        TryInto::<FixedPoint<i8, 4>>::try_into(Into::<BaseIntegerWrapper<_>>::into(3))?,
        TryInto::<FixedPoint<i8, 4>>::try_into(Into::<BaseIntegerWrapper<_>>::into(4))?,
    ].into();
    println!("{:?}", p.w());

    Ok(())
}
