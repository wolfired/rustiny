#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;

use rustiny_fixed_point::FixedPoint;
use rustiny_linear_algebra::{
    Cross, CrossAssign, Matrix, Vector, Vector2, Vector3, Vector4, W, X, Y, Z,
};
use rustiny_number::{CheckedShl, CheckedShr, CheckedSqrt, Integer, Number};

fn main() -> Result<(), Box<dyn Error>> {
    let mut v0: Vector4<u8> = [0, 0, 0, 0].into();
    let x = v0.x_mut();
    *x = 1;
    let y = v0.y_mut();
    *y = 2;
    let z = v0.z_mut();
    *z = 3;
    let w = v0.w_mut();
    *w = 4;

    println!("{:?}", v0);

    Ok(())
}
