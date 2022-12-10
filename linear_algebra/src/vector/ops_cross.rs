//!
//!
//!

use std::ops::Mul;
use std::ops::Sub;

use rustiny_number::Number;
use rustiny_number::Zero;

use crate::Matrix;
use crate::Vector3;

pub trait Cross<Rhs = Self> {
    type Output;

    fn cross(self, rhs: Rhs) -> Self::Output;
}

impl<T: Number> Cross<Self> for Vector3<T>
where
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Zero,
{
    type Output = Self;

    fn cross(self, rhs: Self) -> Self::Output {
        let mut raw = [[T::zero(); 3]; 1];

        raw[0][0] = self.0[0][1] * rhs.0[0][2] - self.0[0][2] * rhs.0[0][1];
        raw[0][1] = self.0[0][2] * rhs.0[0][0] - self.0[0][0] * rhs.0[0][2];
        raw[0][2] = self.0[0][0] * rhs.0[0][1] - self.0[0][1] * rhs.0[0][0];

        Self(raw)
    }
}

impl<T: Number> Cross<&Self> for Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> T: Mul<&'a T, Output = T>,
    T: Zero,
{
    type Output = Self;

    fn cross(self, rhs: &Self) -> Self::Output {
        let mut raw = [[T::zero(); 3]; 1];

        raw[0][0] = self.0[0][1] * &rhs.0[0][2] - self.0[0][2] * &rhs.0[0][1];
        raw[0][1] = self.0[0][2] * &rhs.0[0][0] - self.0[0][0] * &rhs.0[0][2];
        raw[0][2] = self.0[0][0] * &rhs.0[0][1] - self.0[0][1] * &rhs.0[0][0];

        Self(raw)
    }
}

impl<T: Number> Cross<Self> for &Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
    T: Zero,
{
    type Output = Vector3<T>;

    fn cross(self, rhs: Self) -> Self::Output {
        let mut raw = [[T::zero(); 3]; 1];

        raw[0][0] = &self.0[0][1] * &rhs.0[0][2] - &self.0[0][2] * &rhs.0[0][1];
        raw[0][1] = &self.0[0][2] * &rhs.0[0][0] - &self.0[0][0] * &rhs.0[0][2];
        raw[0][2] = &self.0[0][0] * &rhs.0[0][1] - &self.0[0][1] * &rhs.0[0][0];

        Matrix::<T, 1, 3>(raw)
    }
}

impl<T: Number> Cross<Vector3<T>> for &Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<T, Output = T>,
    T: Zero,
{
    type Output = Vector3<T>;

    fn cross(self, rhs: Vector3<T>) -> Self::Output {
        let mut raw = [[T::zero(); 3]; 1];

        raw[0][0] = &self.0[0][1] * rhs.0[0][2] - &self.0[0][2] * rhs.0[0][1];
        raw[0][1] = &self.0[0][2] * rhs.0[0][0] - &self.0[0][0] * rhs.0[0][2];
        raw[0][2] = &self.0[0][0] * rhs.0[0][1] - &self.0[0][1] * rhs.0[0][0];

        Matrix::<T, 1, 3>(raw)
    }
}

pub trait CrossAssign<Rhs = Self> {
    fn cross_assign(&mut self, rhs: Rhs);
}

impl<T: Number> CrossAssign<Self> for Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<T, Output = T>,
    T: Zero,
{
    fn cross_assign(&mut self, rhs: Self) {
        let mut raw = [[T::zero(); 3]; 1];

        raw[0][0] = &self.0[0][1] * rhs.0[0][2] - &self.0[0][2] * rhs.0[0][1];
        raw[0][1] = &self.0[0][2] * rhs.0[0][0] - &self.0[0][0] * rhs.0[0][2];
        raw[0][2] = &self.0[0][0] * rhs.0[0][1] - &self.0[0][1] * rhs.0[0][0];

        self.0 = raw;
    }
}

impl<T: Number> CrossAssign<&Self> for Vector3<T>
where
    T: Sub<T, Output = T>,
    for<'a> &'a T: Mul<&'a T, Output = T>,
    T: Zero,
{
    fn cross_assign(&mut self, rhs: &Self) {
        let mut raw = [[T::zero(); 3]; 1];

        raw[0][0] = &self.0[0][1] * &rhs.0[0][2] - &self.0[0][2] * &rhs.0[0][1];
        raw[0][1] = &self.0[0][2] * &rhs.0[0][0] - &self.0[0][0] * &rhs.0[0][2];
        raw[0][2] = &self.0[0][0] * &rhs.0[0][1] - &self.0[0][1] * &rhs.0[0][0];

        self.0 = raw;
    }
}
