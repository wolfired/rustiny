#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;

use rustiny_fixed_point::FixedPoint;
use rustiny_linear_algebra::{Cross, CrossAssign, Matrix, Vector, Vector3, Vector4};
use rustiny_number::{CheckedShl, CheckedShr, CheckedSqrt, Integer, Number};

fn main() -> Result<(), Box<dyn Error>> {
    for v in 0..=255 {
        println!("{:?}", FixedPoint::<u8, 8>(v));
    }

    let mut fp0: FixedPoint<u8, 4> = 1.0625f32.try_into()?;
    let fp1: FixedPoint<u8, 4> = 1.try_into()?;

    fp0 -= fp1;

    println!("{:?}", fp0.checked_sqrt().unwrap());

    let m0: Vector4<FixedPoint<i16, 4>> = [
        0.try_into()?,
        1.25.try_into()?,
        2.try_into()?,
        3.try_into()?,
    ]
    .into();
    let m1: Vector4<FixedPoint<i16, 4>> = [
        0.try_into()?,
        1.25.try_into()?,
        2.try_into()?,
        3.try_into()?,
    ]
    .into();

    println!("{:?}", m0 + m1);

    let m2: Vector3<f32> = [1.0, 1.0, 1.0].into();
    println!("{}", m2.magnitude());

    let m3: Vector3<FixedPoint<i32, 8>> =
        [1.0.try_into()?, 1.0.try_into()?, 1.0.try_into()?].into();
    println!("{}", m3.magnitude());

    let x: Vector3<f32> = [1.0, 0.0, 0.0].into();
    let y: Vector3<f32> = [0.0, 1.0, 0.0].into();
    let z = x.cross(y);
    println!("{}", z);

    println!("{:?}", -129.25 as u8);
    println!("{:?}", 129.25 as i8);

    Ok(())
}
