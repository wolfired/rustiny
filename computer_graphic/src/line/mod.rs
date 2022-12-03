use std::ops::{AddAssign, Neg, Shl, Sub, SubAssign};

use rustiny_number::Number;

pub fn line_bresenham<T: Number, F: FnMut(T, T)>(
    mut x0: T,
    mut y0: T,
    mut x1: T,
    mut y1: T,
    mut f: F,
) where
    T: AddAssign<T>
        + Sub<T, Output = T>
        + SubAssign<T>
        + Shl<T, Output = T>
        + Ord
        + Neg<Output = T>,
{
    let steep = (x1 - x0).num_abs() < (y1 - y0).num_abs();

    if steep {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
    }

    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    let dx = x1 - x0;
    let dx2 = dx << T::NUM_1;

    let slope = (y1 - y0).num_abs() << T::NUM_1;
    let mut slope_sum = T::NUM_0;

    let step_y = if y0 > y1 { -T::NUM_1 } else { T::NUM_1 };

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
        x += T::NUM_1;
    }
}
