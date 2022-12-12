//!
//!
//!

use rustiny_linear_algebra::Vector;
use rustiny_number::Number;

pub type Color3<T> = Vector<T, 3>;
pub type Color4<T> = Vector<T, 4>;

pub trait R {
    type Output;

    fn r(self) -> Self::Output;
    fn r_ref(&self) -> &Self::Output;
    fn r_mut(&mut self) -> &mut Self::Output;
}

pub trait G {
    type Output;

    fn g(self) -> Self::Output;
    fn g_ref(&self) -> &Self::Output;
    fn g_mut(&mut self) -> &mut Self::Output;
}

pub trait B {
    type Output;

    fn b(self) -> Self::Output;
    fn b_ref(&self) -> &Self::Output;
    fn b_mut(&mut self) -> &mut Self::Output;
}

pub trait A {
    type Output;

    fn a(self) -> Self::Output;
    fn a_ref(&self) -> &Self::Output;
    fn a_mut(&mut self) -> &mut Self::Output;
}

impl<T: Number> R for Color3<T> {
    type Output = T;

    fn r(self) -> Self::Output {
        self.0[0][0]
    }

    fn r_ref(&self) -> &Self::Output {
        &self.0[0][0]
    }

    fn r_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][0]
    }
}

impl<T: Number> G for Color3<T> {
    type Output = T;

    fn g(self) -> Self::Output {
        self.0[0][1]
    }

    fn g_ref(&self) -> &Self::Output {
        &self.0[0][1]
    }

    fn g_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][1]
    }
}

impl<T: Number> B for Color3<T> {
    type Output = T;

    fn b(self) -> Self::Output {
        self.0[0][1]
    }

    fn b_ref(&self) -> &Self::Output {
        &self.0[0][1]
    }

    fn b_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][1]
    }
}

impl<T: Number> R for Color4<T> {
    type Output = T;

    fn r(self) -> Self::Output {
        self.0[0][0]
    }

    fn r_ref(&self) -> &Self::Output {
        &self.0[0][0]
    }

    fn r_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][0]
    }
}

impl<T: Number> G for Color4<T> {
    type Output = T;

    fn g(self) -> Self::Output {
        self.0[0][1]
    }

    fn g_ref(&self) -> &Self::Output {
        &self.0[0][1]
    }

    fn g_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][1]
    }
}

impl<T: Number> B for Color4<T> {
    type Output = T;

    fn b(self) -> Self::Output {
        self.0[0][2]
    }

    fn b_ref(&self) -> &Self::Output {
        &self.0[0][2]
    }

    fn b_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][2]
    }
}

impl<T: Number> A for Color4<T> {
    type Output = T;

    fn a(self) -> Self::Output {
        self.0[0][3]
    }

    fn a_ref(&self) -> &Self::Output {
        &self.0[0][3]
    }

    fn a_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][3]
    }
}
