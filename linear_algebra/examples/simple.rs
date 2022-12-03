#![allow(unused_variables)]
#![allow(unused_imports)]

use std::fmt::Debug;
use std::ops::{Shr, SubAssign};
use std::{error::Error, ops::Add};

use rustiny_fixed_point::FixedPoint;
use rustiny_linear_algebra::{raw_dot, Matrix, Vector};
use rustiny_number::{BaseIntegerWrapper, Expend, Number, Shrink};

fn main() -> Result<(), Box<dyn Error>> {
    let m0: Matrix<FixedPoint<i32, 8>, 2, 2> = [
        [1.25f32.try_into()?, 2.5f32.try_into()?],
        [2.5f32.try_into()?, 1.25f32.try_into()?],
    ]
    .into();
    let m1: Matrix<FixedPoint<i32, 8>, 2, 2> = [
        [2.5f32.try_into()?, 1.25f32.try_into()?],
        [1.25f32.try_into()?, 2.5f32.try_into()?],
    ]
    .into();
    // let m2 = Matrix::<FixedPoint<i32, 8>, 2, 2>::new_identity();
    println!("{}", m0 - m1);

    let o = &mut [1, 0, 0, 4][..];
    let l = &[1, 2, 3, 4][..];
    let r = &[1, 2, 3, 4][..];
    let d = raw_dot(l, r, 4)?;
    println!("{:?}", d);

    let v0: Vector<FixedPoint<i16, 4>, 3> = [
        Into::<BaseIntegerWrapper<_>>::into(1).try_into()?,
        Into::<BaseIntegerWrapper<_>>::into(0).try_into()?,
        Into::<BaseIntegerWrapper<_>>::into(0).try_into()?,
    ]
    .into();
    let v1: Vector<FixedPoint<i16, 4>, 3> = [
        Into::<BaseIntegerWrapper<_>>::into(0).try_into()?,
        Into::<BaseIntegerWrapper<_>>::into(1).try_into()?,
        Into::<BaseIntegerWrapper<_>>::into(0).try_into()?,
    ]
    .into();
    println!("{}", v0.cross(v1));

    let aa = [3.0.try_into()?, 4.0.try_into()?, 0.0.try_into()?];
    let mm: Vector<FixedPoint<i16, 4>, 3> = aa.into();
    println!("{:?}", mm.magnitude());

    let mm: Matrix<u8, 4, 3> = [[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]].into();
    println!("{}", mm);
    println!("{}", mm.transpose());
    Ok(())
}
