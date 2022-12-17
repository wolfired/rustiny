//!
//!
//!

use rustiny_number::types::Number;

use crate::vector::Vector;

pub trait Dot<Rhs = Self> {
    type Output;

    fn dot(self, rhs: Rhs) -> Self::Output;
}

impl<T: Number, const N: usize> Dot<Self> for Vector<T, N> {
    type Output = T;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut result = T::zero();

        for c in 0..N {
            result += self.0[0][c] * rhs.0[0][c];
        }

        result
    }
}

impl<T: Number, const N: usize> Dot<&Self> for Vector<T, N> {
    type Output = T;

    fn dot(self, rhs: &Self) -> Self::Output {
        let mut result = T::zero();

        for c in 0..N {
            result += self.0[0][c] * rhs.0[0][c];
        }

        result
    }
}

impl<T: Number, const N: usize> Dot<Self> for &Vector<T, N> {
    type Output = T;

    fn dot(self, rhs: Self) -> Self::Output {
        let mut result = T::zero();

        for c in 0..N {
            result += self.0[0][c] * rhs.0[0][c];
        }

        result
    }
}

impl<T: Number, const N: usize> Dot<Vector<T, N>> for &Vector<T, N> {
    type Output = T;

    fn dot(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = T::zero();

        for c in 0..N {
            result += self.0[0][c] * rhs.0[0][c];
        }

        result
    }
}
