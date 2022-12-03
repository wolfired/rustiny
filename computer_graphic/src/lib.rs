use std::ops::{AddAssign, Mul};

use rustiny_linear_algebra::Matrix;
use rustiny_number::Number;

mod line;
pub use line::line_bresenham;

pub trait X<T> {
    fn x(&self) -> T;
}

pub trait Y<T> {
    fn y(&self) -> T;
}

pub trait Z<T> {
    fn z(&self) -> T;
}

pub trait W<T> {
    fn w(&self) -> T;
}

pub type Point<T, const N: usize> = Matrix<T, 1, N>;
pub type Color<T, const N: usize> = Matrix<T, 1, N>;

impl<T: Number> X<T> for Point<T, 3>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    fn x(&self) -> T {
        self.get(0, 0)
    }
}

impl<T: Number> Y<T> for Point<T, 3>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    fn y(&self) -> T {
        self.get(0, 1)
    }
}

impl<T: Number> Z<T> for Point<T, 3>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    fn z(&self) -> T {
        self.get(0, 2)
    }
}

impl<T: Number> X<T> for Point<T, 4>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    fn x(&self) -> T {
        self.get(0, 0)
    }
}

impl<T: Number> Y<T> for Point<T, 4>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    fn y(&self) -> T {
        self.get(0, 1)
    }
}

impl<T: Number> Z<T> for Point<T, 4>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    fn z(&self) -> T {
        self.get(0, 2)
    }
}

impl<T: Number> W<T> for Point<T, 4>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    fn w(&self) -> T {
        self.get(0, 2)
    }
}