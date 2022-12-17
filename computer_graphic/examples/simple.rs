#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;

use rustiny_computer_graphic::point::Point2;
use rustiny_linear_algebra::vector::X;

fn main() -> Result<(), Box<dyn Error>> {
    let mut p0: Point2<i8> = [1, 2].into();
    let p1: Point2<i8> = [2, 3].into();
    let v0 = p1 - p0;

    println!("{}", *p0.x_ref());

    p0.0[0][0] = 0;

    println!("{}", *p0.x_ref());

    println!("{:?}\n{:?}", p0, v0);

    Ok(())
}
