#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::{error::Error, fmt::Debug};

fn main() -> Result<(), Box<dyn Error>> {
    let v = 2u32.overflowing_div(0);

    println!("{:?}", v);

    Ok(())
}
