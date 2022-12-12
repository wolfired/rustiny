use rustiny_number::Number;

use crate::Vector2;
use crate::Vector3;
use crate::Vector4;

pub trait X {
    type Output;

    fn x(self) -> Self::Output;
    fn x_ref(&self) -> &Self::Output;
    fn x_mut(&mut self) -> &mut Self::Output;
}

pub trait Y {
    type Output;

    fn y(self) -> Self::Output;
    fn y_ref(&self) -> &Self::Output;
    fn y_mut(&mut self) -> &mut Self::Output;
}

pub trait Z {
    type Output;

    fn z(self) -> Self::Output;
    fn z_ref(&self) -> &Self::Output;
    fn z_mut(&mut self) -> &mut Self::Output;
}

pub trait W {
    type Output;

    fn w(self) -> Self::Output;
    fn w_ref(&self) -> &Self::Output;
    fn w_mut(&mut self) -> &mut Self::Output;
}

impl<T: Number> X for Vector2<T> {
    type Output = T;

    fn x(self) -> Self::Output {
        self.0[0][0]
    }

    fn x_ref(&self) -> &Self::Output {
        &self.0[0][0]
    }

    fn x_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][0]
    }
}

impl<T: Number> Y for Vector2<T> {
    type Output = T;

    fn y(self) -> Self::Output {
        self.0[0][1]
    }

    fn y_ref(&self) -> &Self::Output {
        &self.0[0][1]
    }

    fn y_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][1]
    }
}

impl<T: Number> X for Vector3<T> {
    type Output = T;

    fn x(self) -> Self::Output {
        self.0[0][0]
    }

    fn x_ref(&self) -> &Self::Output {
        &self.0[0][0]
    }

    fn x_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][0]
    }
}

impl<T: Number> Y for Vector3<T> {
    type Output = T;

    fn y(self) -> Self::Output {
        self.0[0][1]
    }

    fn y_ref(&self) -> &Self::Output {
        &self.0[0][1]
    }

    fn y_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][1]
    }
}

impl<T: Number> Z for Vector3<T> {
    type Output = T;

    fn z(self) -> Self::Output {
        self.0[0][2]
    }

    fn z_ref(&self) -> &Self::Output {
        &self.0[0][2]
    }

    fn z_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][2]
    }
}

impl<T: Number> X for Vector4<T> {
    type Output = T;

    fn x(self) -> Self::Output {
        self.0[0][0]
    }

    fn x_ref(&self) -> &Self::Output {
        &self.0[0][0]
    }

    fn x_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][0]
    }
}

impl<T: Number> Y for Vector4<T> {
    type Output = T;

    fn y(self) -> Self::Output {
        self.0[0][1]
    }

    fn y_ref(&self) -> &Self::Output {
        &self.0[0][1]
    }

    fn y_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][1]
    }
}

impl<T: Number> Z for Vector4<T> {
    type Output = T;

    fn z(self) -> Self::Output {
        self.0[0][2]
    }

    fn z_ref(&self) -> &Self::Output {
        &self.0[0][2]
    }

    fn z_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][2]
    }
}

impl<T: Number> W for Vector4<T> {
    type Output = T;

    fn w(self) -> Self::Output {
        self.0[0][3]
    }

    fn w_ref(&self) -> &Self::Output {
        &self.0[0][3]
    }

    fn w_mut(&mut self) -> &mut Self::Output {
        &mut self.0[0][3]
    }
}
