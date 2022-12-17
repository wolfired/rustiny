//!
//!
//!

use rustiny_number::Number;

use crate::Matrix;

pub type Vector<T, const N: usize> = Matrix<T, 1, N>;
pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;
pub type Vector4<T> = Vector<T, 4>;

impl<T: Number, const N: usize> Vector<T, N> {
    pub fn magnitude(&self) -> T {
        let mut result = T::zero();

        for c in 0..N {
            result += self.0[0][c] * self.0[0][c];
        }

        let Some(value) = result.checked_sqrt() else {
            return T::zero();
        };

        return value;
    }
}
