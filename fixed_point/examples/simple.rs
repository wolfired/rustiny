#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt::Debug;
use std::ops::{Shr, SubAssign};
use std::{error::Error, ops::Add};

use rustiny_fixed_point::FixedPoint;
use rustiny_number::{BaseIntegerWrapper, Number, Expend, Shrink};

fn main() -> Result<(), Box<dyn Error>> {
    let fp = TryInto::<FixedPoint<i8, 4>>::try_into(Into::<BaseIntegerWrapper<_>>::into(7));
    println!("{:?}", fp);

    // for value in 0u8..=255 {
    //     let fp = FixedPoint::<i8, 4> { value: value as i8 };
    //     println!("{:?}", fp);
    // }

    let mut x = TryInto::<FixedPoint<u8, 4>>::try_into(Into::<BaseIntegerWrapper<_>>::into(8))?;
    let y = TryInto::<FixedPoint<u8, 4>>::try_into(Into::<BaseIntegerWrapper<_>>::into(4))?;
    x /= y;
    println!("{:?}", x);

    test_asmdr(
        TryInto::<FixedPoint<i32, 8>>::try_into(Into::<BaseIntegerWrapper<_>>::into(5))?,
        TryInto::<FixedPoint<i32, 8>>::try_into(Into::<BaseIntegerWrapper<_>>::into(2))?,
    );

    test_shlr(
        TryInto::<FixedPoint<i32, 8>>::try_into(Into::<BaseIntegerWrapper<_>>::into(5))?,
        TryInto::<FixedPoint<i32, 8>>::try_into(Into::<BaseIntegerWrapper<_>>::into(2))?,
    );

    let mut fp = TryInto::<FixedPoint<u8, 4>>::try_into(2.5625)?;
    println!("pf = {:?}", fp);
    test_asmdr_assign(
        &mut fp,
        &TryInto::<FixedPoint<u8, 4>>::try_into(Into::<BaseIntegerWrapper<_>>::into(1))?,
    );
    println!("pf = {:?}", fp.num_sqrt());

    println!("{}", 9.num_sqrt());

    println!(
        "FixedPoint::<u8, 4>::NUM_MAX = {:?}",
        FixedPoint::<u8, 4>::NUM_MAX
    );
    println!(
        "FixedPoint::<u8, 4>::NUM_MIN = {:?}",
        FixedPoint::<u8, 4>::NUM_MIN
    );

    println!(
        "FixedPoint::<i8, 4>::NUM_MAX = {:?}",
        FixedPoint::<i8, 4>::NUM_MAX
    );
    println!(
        "FixedPoint::<i8, 4>::NUM_MIN = {:?}",
        FixedPoint::<i8, 4>::NUM_MIN
    );

    println!(
        "FixedPoint::<i8, 4>::NUM_0 = {:?}",
        FixedPoint::<i8, 4>::NUM_0
    );
    println!(
        "FixedPoint::<i8, 4>::NUM_1 = {:?}",
        FixedPoint::<i8, 4>::NUM_1
    );

    Ok(())
}

fn test_asmdr<T>(s: T, r: T)
where
    T: Add<T>,
    <T as Add<T>>::Output: Debug,
{
    println!("{:?}", s + r);
    // println!("{:?}", s - r);
    // println!("{:?}", s * r);
    // println!("{:?}", s / r);
    // println!("{:?}", s % r);
}

fn test_asmdr_assign<T: Debug>(s: &mut T, r: &T)
where
    for<'a> T: SubAssign<&'a T>,
{
    // *s += r;
    *s -= r;
    // *s *= r;
    // *s /= r;
    // *s %= r;
    println!("s = {:?}", s);
}

fn test_shlr<T>(s: T, r: T)
where
    T: Shr<T>,
    <T as Shr<T>>::Output: Debug,
{
    // println!("{:?}", s << r);
    println!("{:?}", s >> r);
}
