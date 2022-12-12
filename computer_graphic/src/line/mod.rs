use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::Shl;
use std::ops::Sub;
use std::ops::SubAssign;

use rustiny_number::Integer;
use rustiny_number::One;
use rustiny_number::Zero;

pub fn line_bresenham<T: Integer, F: FnMut(T, T)>(
    mut x0: T,
    mut y0: T,
    mut x1: T,
    mut y1: T,
    mut f: F,
) where
    T: AddAssign<T>,
    T: Sub<T, Output = T>,
    T: SubAssign<T>,
    T: Shl<T, Output = T>,
    T: Ord,
    T: Neg<Output = T>,
    T: Zero,
    T: One,
{
    let steep = if x1 > x0 { x1 - x0 } else { x0 - x1 } < if y1 > y0 { y1 - y0 } else { y0 - y1 };

    if steep {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
    }

    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let dx = x1 - x0;
    let dx2 = dx << T::one();

    let slope = if y1 > y0 { y1 - y0 } else { y0 - y1 } << T::one();
    let mut slope_sum = T::zero();

    let step_y = if y0 > y1 { -T::one() } else { T::one() };

    let mut x = x0;
    let mut y = y0;

    while x <= x1 {
        if steep {
            f(y, x);
        } else {
            f(x, y);
        }

        slope_sum += slope;

        if dx < slope_sum {
            y += step_y;
            slope_sum -= dx2;
        }

        x += T::one();
    }
}
