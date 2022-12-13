use std::error::Error;

use rustiny_bits::UnsignedBits;
use rustiny_bits::Bits;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", 0b_0000_0010u8.is_power_of_two());
    println!("{}", (3i8).is_bits_set());
    println!("{}", 0b_0000_0001u8.is_power_of_two_minus_one());
    Ok(())
}
