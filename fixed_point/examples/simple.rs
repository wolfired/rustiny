#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;

use rustiny_fixed_point::FixedPoint;
use rustiny_number::Abs;
use rustiny_number::CheckedAdd;
use rustiny_number::CheckedShl;
use rustiny_number::Integer;
use rustiny_number::OverflowingAdd;
use rustiny_number::OverflowingDiv;
use rustiny_number::OverflowingMul;
use rustiny_number::OverflowingNeg;
use rustiny_number::OverflowingPow;
use rustiny_number::OverflowingShl;

fn main() -> Result<(), Box<dyn Error>> {
    let fp0: FixedPoint<i8, 4> = (-0.5f32).try_into()?;
    println!("{:?}", fp0);
    println!("{:?}", fp0.abs());
    Ok(())
}
