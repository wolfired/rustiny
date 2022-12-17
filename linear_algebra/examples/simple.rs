#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;

use rustiny_fixed_point::FixedPoint;
use rustiny_linear_algebra::{
    Cross, CrossAssign, Matrix, Vector, Vector2, Vector3, Vector4, W, X, Y, Z,
};
use rustiny_number::{CheckedShl, CheckedShr, CheckedSqrt, Integer, Number};

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", (-2.0f32).sqrt());

    Ok(())
}
