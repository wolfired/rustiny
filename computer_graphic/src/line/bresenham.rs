//!
//!
//!

use rustiny_linear_algebra::vector::X;
use rustiny_linear_algebra::vector::Y;
use rustiny_number::types::Signed;

use crate::point::Point2;

pub fn line<T: Signed, F: FnMut(T, T)>(mut p0: Point2<T>, mut p1: Point2<T>, mut f: F) {
    let steep = (p1.x() - p0.x()).abs() < (p1.y() - p0.y()).abs();

    if steep {
        (*p0.x_mut(), *p0.y_mut()) = (p0.y(), p0.x());
        (*p1.x_mut(), *p1.y_mut()) = (p1.y(), p1.x());
    }

    if p0.x() > p1.x() {
        (*p0.x_mut(), *p1.x_mut()) = (p1.x(), p0.x());
        (*p0.y_mut(), *p1.y_mut()) = (p1.y(), p0.y());
    }

    let dx = p1.x() - p0.x();
    let dx2 = dx << T::one();

    let slope = (p1.y() - p0.y()).abs() << T::one();
    let mut slope_sum = T::zero();

    let step_y = if p0.y() > p1.y() { -T::one() } else { T::one() };

    let mut x = p0.x();
    let mut y = p0.y();

    while x <= p1.x() {
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
