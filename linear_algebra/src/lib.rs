use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::slice::from_raw_parts;
use std::slice::from_raw_parts_mut;

use rustiny_number::Number;

#[derive(Debug)]
pub struct Matrix<T: Number, const R: usize, const C: usize> {
    raw: [[T; C]; R],
}

impl<T: Number, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    pub fn new() -> Self {
        Self {
            raw: [[T::NUM_0; C]; R],
        }
    }

    pub fn set(&mut self, r: usize, c: usize, val: T) {
        self.raw[r][c] = val;
    }

    pub fn get(&self, r: usize, c: usize) -> T {
        self.raw[r][c]
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        let raw = [[T::NUM_0; R]; C];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; R]; C] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            raw_transpose(result, left, R, C).unwrap();
        }

        Matrix::<T, C, R> { raw }
    }

    pub fn mul<const CC: usize>(&self, rhs: &Matrix<T, C, CC>) -> Matrix<T, R, CC> {
        let raw = [[T::NUM_0; CC]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; CC]; R] as *mut T, R * CC);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; CC]; C] as *const T, C * CC);
            raw_mul(result, left, right, R, C, CC).unwrap();
        }

        Matrix::<T, R, CC> { raw }
    }
}

impl<T: Number, const N: usize> Matrix<T, N, N> {
    pub fn new_identity() -> Self {
        let mut raw = [[T::NUM_0; N]; N];

        for i in 0..N {
            raw[i][i] = T::NUM_1;
        }

        Self { raw }
    }
}

impl<T: Number, const N: usize> Matrix<T, 1, N>
where
    T: AddAssign<T> + Mul<T, Output = T>,
{
    pub fn dot(self, rhs: Self) -> T {
        unsafe {
            let left = from_raw_parts(&self.raw as *const [[T; N]; 1] as *const T, N);
            let right = from_raw_parts(&rhs.raw as *const [[T; N]; 1] as *const T, N);
            raw_dot(left, right, N).unwrap()
        }
    }

    pub fn magnitude(self) -> T {
        unsafe {
            let left = from_raw_parts(&self.raw as *const [[T; N]; 1] as *const T, N);
            raw_magnitude(left, N).unwrap()
        }
    }
}

impl<T: Number> Matrix<T, 1, 3>
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T>,
{
    pub fn cross(self, rhs: Self) -> Self {
        let raw = [[T::NUM_0; 3]; 1];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; 3]; 1] as *mut T, 3);
            let left = from_raw_parts(&self.raw as *const [[T; 3]; 1] as *const T, 3);
            let right = from_raw_parts(&rhs.raw as *const [[T; 3]; 1] as *const T, 3);
            raw_cross(result, left, right).unwrap();
        }

        Self { raw }
    }
}

// impl fmt
impl<T: Number, const R: usize, const C: usize> Display for Matrix<T, R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..R {
            for c in 0..C {
                write!(f, "{}", self.raw[r][c])?;
                if c < C - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}
// impl fmt done

// impl from
impl<T: Number, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(raw: [[T; C]; R]) -> Self {
        Self { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> From<&[[T; C]; R]> for Matrix<T, R, C> {
    fn from(raw: &[[T; C]; R]) -> Self {
        Self { raw: *raw }
    }
}

impl<T: Number, const C: usize> From<[T; C]> for Matrix<T, 1, C> {
    fn from(raw: [T; C]) -> Self {
        Self { raw: [raw; 1] }
    }
}

impl<T: Number, const C: usize> From<&[T; C]> for Matrix<T, 1, C> {
    fn from(raw: &[T; C]) -> Self {
        Self { raw: [*raw; 1] }
    }
}
// impl from done

// impl add
impl<T: Number, const R: usize, const C: usize> Add<Self> for Matrix<T, R, C>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let raw = [[T::NUM_0; C]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; C]; R] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_add(result, left, right, R, C).unwrap();
        }

        Self { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> Add<&Self> for Matrix<T, R, C>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let raw = [[T::NUM_0; C]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; C]; R] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_add(result, left, right, R, C).unwrap();
        }

        Self { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> Add<Self> for &Matrix<T, R, C>
where
    T: Add<T, Output = T>,
{
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Self) -> Self::Output {
        let raw = [[T::NUM_0; C]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; C]; R] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_add(result, left, right, R, C).unwrap();
        }

        Matrix::<T, R, C> { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> Add<Matrix<T, R, C>> for &Matrix<T, R, C>
where
    T: Add<T, Output = T>,
{
    type Output = Matrix<T, R, C>;

    fn add(self, rhs: Matrix<T, R, C>) -> Self::Output {
        let raw = [[T::NUM_0; C]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; C]; R] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_add(result, left, right, R, C).unwrap();
        }

        Matrix::<T, R, C> { raw }
    }
}
// impl add done

// impl add_assign
impl<T: Number, const R: usize, const C: usize> AddAssign<Self> for Matrix<T, R, C>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        unsafe {
            let left = from_raw_parts_mut(&self.raw as *const [[T; C]; R] as *mut T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_add_assign(left, right, R, C).unwrap();
        }
    }
}

impl<T: Number, const R: usize, const C: usize> AddAssign<&Self> for Matrix<T, R, C>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: &Self) {
        unsafe {
            let left = from_raw_parts_mut(&self.raw as *const [[T; C]; R] as *mut T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_add_assign(left, right, R, C).unwrap();
        }
    }
}
// impl add_assign done

// impl sub
impl<T: Number, const R: usize, const C: usize> Sub<Self> for Matrix<T, R, C>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let raw = [[T::NUM_0; C]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; C]; R] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_sub(result, left, right, R, C).unwrap();
        }

        Self { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> Sub<&Self> for Matrix<T, R, C>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        let raw = [[T::NUM_0; C]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; C]; R] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_sub(result, left, right, R, C).unwrap();
        }

        Self { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> Sub<Self> for &Matrix<T, R, C>
where
    T: Sub<T, Output = T>,
{
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: Self) -> Self::Output {
        let raw = [[T::NUM_0; C]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; C]; R] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_sub(result, left, right, R, C).unwrap();
        }

        Matrix::<T, R, C> { raw }
    }
}

impl<T: Number, const R: usize, const C: usize> Sub<Matrix<T, R, C>> for &Matrix<T, R, C>
where
    T: Sub<T, Output = T>,
{
    type Output = Matrix<T, R, C>;

    fn sub(self, rhs: Matrix<T, R, C>) -> Self::Output {
        let raw = [[T::NUM_0; C]; R];

        unsafe {
            let result = from_raw_parts_mut(&raw as *const [[T; C]; R] as *mut T, R * C);
            let left = from_raw_parts(&self.raw as *const [[T; C]; R] as *const T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_sub(result, left, right, R, C).unwrap();
        }

        Matrix::<T, R, C> { raw }
    }
}
// impl sub done

// impl sub_assign
impl<T: Number, const R: usize, const C: usize> SubAssign<Self> for Matrix<T, R, C>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        unsafe {
            let left = from_raw_parts_mut(&self.raw as *const [[T; C]; R] as *mut T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_sub_assign(left, right, R, C).unwrap();
        }
    }
}

impl<T: Number, const R: usize, const C: usize> SubAssign<&Self> for Matrix<T, R, C>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: &Self) {
        unsafe {
            let left = from_raw_parts_mut(&self.raw as *const [[T; C]; R] as *mut T, R * C);
            let right = from_raw_parts(&rhs.raw as *const [[T; C]; R] as *const T, R * C);
            raw_sub_assign(left, right, R, C).unwrap();
        }
    }
}
// impl sub_assign done

// impl raw add
pub fn raw_add<T: Number>(
    result: &mut [T],
    left: &[T],
    right: &[T],
    row: usize,
    col: usize,
) -> Result<(), Box<dyn Error>>
where
    T: Add<T, Output = T>,
{
    let count = row * col;
    if result.len() < count || left.len() < count || right.len() < count {
        return Err("长度错误".into());
    }

    for r in 0..row {
        for c in 0..col {
            result[r * col + c] = left[r * col + c] + right[r * col + c];
        }
    }

    Ok(())
}
// impl raw add done

// impl raw add_assing
pub fn raw_add_assign<T: Number>(
    left: &mut [T],
    right: &[T],
    row: usize,
    col: usize,
) -> Result<(), Box<dyn Error>>
where
    T: AddAssign<T>,
{
    let count = row * col;
    if left.len() < count || right.len() < count {
        return Err("长度错误".into());
    }

    for r in 0..row {
        for c in 0..col {
            left[r * col + c] += right[r * col + c];
        }
    }

    Ok(())
}
// impl raw add_assing done

// impl raw sub
pub fn raw_sub<T: Number>(
    result: &mut [T],
    left: &[T],
    right: &[T],
    row: usize,
    col: usize,
) -> Result<(), Box<dyn Error>>
where
    T: Sub<T, Output = T>,
{
    let count = row * col;
    if result.len() < count || left.len() < count || right.len() < count {
        return Err("长度错误".into());
    }

    for r in 0..row {
        for c in 0..col {
            result[r * col + c] = left[r * col + c] - right[r * col + c];
        }
    }

    Ok(())
}
// impl raw sub done

// impl raw sub_assing
pub fn raw_sub_assign<T: Number>(
    left: &mut [T],
    right: &[T],
    row: usize,
    col: usize,
) -> Result<(), Box<dyn Error>>
where
    T: SubAssign<T>,
{
    let count = row * col;
    if left.len() < count || right.len() < count {
        return Err("长度错误".into());
    }

    for r in 0..row {
        for c in 0..col {
            left[r * col + c] -= right[r * col + c];
        }
    }

    Ok(())
}
// impl raw add_assing done

// impl raw mul
pub fn raw_mul<T: Number>(
    result: &mut [T],
    left: &[T],
    right: &[T],
    row: usize,
    mid: usize,
    col: usize,
) -> Result<(), Box<dyn Error>>
where
    T: Mul<T, Output = T> + AddAssign<T>,
{
    if result.len() < row * col || left.len() < row * mid || right.len() < mid * col {
        return Err("长度错误".into());
    }

    for r in 0..row {
        for m in 0..mid {
            for c in 0..col {
                result[r * mid + c] += left[r * mid + m] * right[m * col + c];
            }
        }
    }

    Ok(())
}
// impl raw mul done

// impl raw mul scalar
pub fn raw_mul_scalar<T: Number>(
    result: &mut [T],
    left: &[T],
    right: T,
    row: usize,
    col: usize,
) -> Result<(), Box<dyn Error>>
where
    T: Mul<T, Output = T>,
{
    let count = row * col;
    if result.len() < count || left.len() < count {
        return Err("长度错误".into());
    }

    for r in 0..row {
        for c in 0..col {
            result[r * col + c] = left[r * col + c] * right;
        }
    }

    Ok(())
}
// impl raw mul scalar done

// impl raw mul scalar assign
pub fn raw_mul_scalar_assign<T: Number>(
    left: &mut [T],
    right: T,
    row: usize,
    col: usize,
) -> Result<(), Box<dyn Error>>
where
    T: MulAssign<T>,
{
    let count = row * col;
    if left.len() < count {
        return Err("长度错误".into());
    }

    for r in 0..row {
        for c in 0..col {
            left[r * col + c] *= right;
        }
    }

    Ok(())
}
// impl raw mul scalar assign done

// impl raw dot
pub fn raw_dot<T: Number>(left: &[T], right: &[T], col: usize) -> Result<T, Box<dyn Error>>
where
    T: AddAssign<T> + Mul<T, Output = T>,
{
    if left.len() < col || right.len() < col {
        return Err("长度错误".into());
    }

    let mut result = T::NUM_0;

    for c in 0..col {
        result += left[c] * right[c];
    }

    Ok(result)
}
// impl raw dot done

// impl raw cross
pub fn raw_cross<T: Number>(result: &mut [T], left: &[T], right: &[T]) -> Result<(), Box<dyn Error>>
where
    T: Sub<T, Output = T> + Mul<T, Output = T>,
{
    if result.len() < 3 || left.len() < 3 || right.len() < 3 {
        return Err("长度错误".into());
    }

    result[0] = left[1] * right[2] - left[2] * right[1];
    result[1] = left[2] * right[0] - left[0] * right[2];
    result[2] = left[0] * right[1] - left[1] * right[0];

    Ok(())
}
// impl raw cross done

// impl raw transpose
pub fn raw_transpose<T: Number>(
    result: &mut [T],
    left: &[T],
    row: usize,
    col: usize,
) -> Result<(), Box<dyn Error>> {
    let count = row * col;
    if result.len() < count || left.len() < count {
        return Err("长度错误".into());
    }

    for r in 0..row {
        for c in 0..col {
            result[c * row + r] = left[r * col + c];
        }
    }

    Ok(())
}
// impl raw transpose done

// impl raw magnitude
pub fn raw_magnitude<T: Number>(left: &[T], col: usize) -> Result<T, Box<dyn Error>>
where
    T: AddAssign<T> + Mul<T, Output = T>,
{
    if left.len() < col {
        return Err("长度错误".into());
    }

    let mut result = T::NUM_0;

    for c in 0..col {
        result += left[c] * left[c];
    }

    Ok(result.num_sqrt())
}
// impl raw magnitude done

pub type Vector<T, const N: usize> = Matrix<T, 1, N>;
