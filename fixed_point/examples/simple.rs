#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;

use rustiny_fixed_point::types::FixedPoint;
use rustiny_number::ops::Abs;
use rustiny_number::ops::CheckedAdd;
use rustiny_number::ops::CheckedShl;
use rustiny_number::ops::OverflowingAdd;
use rustiny_number::ops::OverflowingDiv;
use rustiny_number::ops::OverflowingMul;
use rustiny_number::ops::OverflowingNeg;
use rustiny_number::ops::OverflowingPow;
use rustiny_number::ops::OverflowingShl;
use rustiny_number::types::Integer;

fn main() -> Result<(), Box<dyn Error>> {
    // let fp0: FixedPoint<u8, 7> = FixedPoint::<u8, 7>::MAX;
    // println!("{:?}", fp0);


    println!("{:?}", FixedPoint::<i8, 7>::MIN);
    println!("{:?}", FixedPoint::<i8, 7>::MAX);
    println!("{:?}", FixedPoint::<u8, 7>::MIN);
    println!("{:?}", FixedPoint::<u8, 7>::MAX);

    Ok(())
}
