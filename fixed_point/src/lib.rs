//! 定点数运算推导:
//!
//! 整数部分位数: m <br/>
//! 小数部分位数: n <br/>
//! 浮点数: a, b <br/>
//! 定点数: A, B <br/>
//!
//! $$ a \cdot 2^n = A $$
//! $$ b \cdot 2^n = B $$
//! $$ (a + b) \cdot 2^n = a \cdot 2^n + b \cdot 2^n = A + B $$
//! $$ (a - b) \cdot 2^n = a \cdot 2^n - b \cdot 2^n = A - B $$
//! $$ (a \cdot b) \cdot 2^n = \frac {a \cdot 2^n \cdot b \cdot 2^n} {2^n} = \frac {A \cdot B} {2^n} = (A \cdot B) \verb|>>| n $$
//! $$ \frac {a} {b} \cdot 2^n = \frac {a \cdot 2^n} {b \cdot 2^n} \cdot 2^n = \frac {A} {B} \cdot 2^n = (\frac {A} {B}) \verb|<<| n $$
//! $$ (a \verb|<<| b) \cdot 2^n = a \cdot 2^b \cdot 2^n = a \cdot 2^n \cdot 2^{\frac {b \cdot 2^n} {2^n}} = A \cdot 2^{\frac B {2^n}} = A \verb|<<| (B \verb|>>| n) $$
//! $$ (a \verb|>>| b) \cdot 2^n = \frac a {2^b} \cdot 2^n = \frac {a \cdot 2^n} {2^{\frac {b \cdot 2^n} {2^n}}} = \frac A {2^{\frac B {2^n}}} = A \verb|>>| (B \verb|>>| n) $$
//! $$ \sqrt{a} \cdot 2^n = a^{\frac 1 2} \cdot 2^n = a^{\frac 1 2} \cdot (2^n \cdot 2^n)^{\frac 1 2} = (a \cdot 2^n \cdot 2^n)^{\frac 1 2} = (A \cdot 2^n)^{\frac 1 2} = \sqrt{A \verb|<<| n} $$
//!

use std::convert::TryFrom;
use std::convert::TryInto;
use std::error::Error;
use std::fmt::Binary;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use rustiny_number::BaseInteger;
use rustiny_number::BaseIntegerWrapper;
use rustiny_number::Expend;
use rustiny_number::Float;
use rustiny_number::Integer;
use rustiny_number::Number;
use rustiny_number::Shrink;

#[derive(Clone, Default, Copy)]
pub struct FixedPoint<T: BaseInteger, const P: usize> {
    pub value: T,
}

// impl number
macro_rules! impl_number {
    ($($t:ty), *) => {
        $(
            impl<const P: usize> Number for FixedPoint<$t, P> {
                const TYPE_STR: &'static str = "FixedPoint";
                const NUM_MAX: Self = Self {
                    value: <$t>::NUM_MAX,
                };
                const NUM_MIN: Self = Self {
                    value: <$t>::NUM_MIN,
                };
                const NUM_0: Self = Self {
                    value: <$t>::NUM_0,
                };
                const NUM_1: Self = Self {
                    value: <$t>::NUM_1 << P,
                };
                const TOTAL_BITS: u32 = <$t>::TOTAL_BITS;
                const VALID_BITS: u32 = <$t>::VALID_BITS;

                fn num_abs(self) -> Self {
                    Self {
                        value: self.value.num_abs(),
                    }
                }

                fn num_sqrt(self) -> Self {
                    Self {
                        value: (self.value.expend() << P).num_sqrt().shrink(),
                    }
                }
            }
        )*
    };
}
impl_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
// impl number done

#[rustfmt::skip]
impl<T: BaseInteger, const P: usize> Debug
    for FixedPoint<T, P>
where
    T: Binary + Shr<usize, Output = T> + Integer,
    FixedPoint<T, P>: Number,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_bits = T::TOTAL_BITS as usize;
        let valid_bits = T::VALID_BITS as usize;

        write!(
            f,
            "FixedPoint<{}, {}>{{ {:+0iwidth$.P$}, {:+0twidth$}, {:#0total_bits$b} }}",
            T::TYPE_STR,
            P,
            Into::<f64>::into(*self),
            self.value,
            self.value,
            iwidth = (if valid_bits <= P { T::NUM_0 } else { T::NUM_MAX >> P })
                .to_string()
                .chars()
                .count()
                + 2
                + P,
            twidth = T::NUM_MAX.to_string().chars().count() + 1,
            total_bits = { total_bits + 2 },
        )
    }
}

impl<T: BaseInteger, const P: usize> Display for FixedPoint<T, P>
where
    T: Integer,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<f64>::into(*self))
    }
}

impl<F: BaseInteger, T: BaseInteger, const P: usize> TryFrom<BaseIntegerWrapper<F>>
    for FixedPoint<T, P>
where
    F: From<BaseIntegerWrapper<F>>,
    T: TryFrom<F> + Shl<u32, Output = T> + Shr<u32, Output = T> + Ord + Integer,
{
    type Error = Box<dyn Error>;

    fn try_from(value: BaseIntegerWrapper<F>) -> Result<Self, Self::Error> {
        let value = Into::<F>::into(value);
        let Ok(value) = value.try_into() else {
            return Err(format!("{} 超出有效范围: [{}, {}]", value, T::NUM_MIN, T::NUM_MAX).into());
        };

        if T::NUM_0 == value {
            return Ok(Self { value });
        }

        let p_bits = P.try_into()?;
        let i_min = T::NUM_MIN >> p_bits;
        let i_max = T::NUM_MAX >> p_bits;

        if value < i_min || i_max < value {
            return Err(format!("{} 超出有效范围: [{}, {}]", value, i_min, i_max).into());
        }

        Ok(Self {
            value: value << p_bits,
        })
    }
}

impl<T: BaseInteger, const P: usize> TryFrom<f32> for FixedPoint<T, P>
where
    T: TryFrom<i128> + Integer,
    FixedPoint<T, P>: Number,
{
    type Error = Box<dyn Error>;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let Ok(value) = (value * (1 << P) as f32).to_i128().try_into() else {
            return Err(format!("{} 超出有效范围: [{}, {}], ", value, Into::<f64>::into(Self::NUM_MIN), Into::<f64>::into(Self::NUM_MAX)).into());
        };
        Ok(Self { value })
    }
}

impl<T: BaseInteger, const P: usize> TryFrom<f64> for FixedPoint<T, P>
where
    T: TryFrom<i128> + Integer,
    FixedPoint<T, P>: Number,
{
    type Error = Box<dyn Error>;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let Ok(value) = (value * (1 << P) as f64).to_i128().try_into() else {
            return Err(format!("{} 超出有效范围: [{}, {}], ", value, Into::<f64>::into(Self::NUM_MIN), Into::<f64>::into(Self::NUM_MAX)).into());
        };
        Ok(Self { value })
    }
}

macro_rules! impl_x_tryfrom_fpux {
    ($t0:ty, $($t1:ty),*) => {
        $(
            impl<const P: usize> TryFrom<FixedPoint<$t0, P>> for $t1 {
                type Error = Box<dyn Error>;

                fn try_from(value: FixedPoint<$t0, P>) -> Result<Self, Self::Error> {
                    let v_bits = <$t0>::VALID_BITS;
                    let p_bits = P.try_into()?;

                    if v_bits <= p_bits {
                        return Ok(0);
                    }

                    if 0 == p_bits {
                        return Ok(value.value.try_into()?);
                    }

                    let zero_point_five = 1 << (p_bits - 1);

                    let Some(value_add_zpf) = value.value.checked_add(zero_point_five) else {
                        return Err("overflow".into());
                    };

                    return Ok((value_add_zpf >> p_bits).try_into()?);
                }
            }
        )*
    }
}
impl_x_tryfrom_fpux!(u8, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpux!(u16, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpux!(u32, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpux!(u64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpux!(u128, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpux!(usize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

macro_rules! impl_x_tryfrom_fpix {
    ($t0:ty, $($t1:ty),*) => {
        $(
            impl<const P: usize> TryFrom<FixedPoint<$t0, P>> for $t1 {
                type Error = Box<dyn Error>;

                fn try_from(value: FixedPoint<$t0, P>) -> Result<Self, Self::Error> {
                    let v_bits = <$t0>::VALID_BITS;
                    let p_bits = P.try_into()?;

                    if v_bits <= p_bits {
                        return Ok(0);
                    }

                    if 0 == p_bits {
                        return Ok(value.value.try_into()?);
                    }

                    let zero_point_five = 1 << (p_bits - 1);

                    let Some(value_add_zpf) = value.value.checked_add(zero_point_five) else {
                        return Err("overflow".into());
                    };

                    if 0 > value.value {
                        return Ok(((value_add_zpf - 1) >> p_bits).try_into()?);
                    }

                    return Ok((value_add_zpf >> p_bits).try_into()?);
                }
            }
        )*
    }
}
impl_x_tryfrom_fpix!(i8, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpix!(i16, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpix!(i32, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpix!(i64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpix!(i128, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_x_tryfrom_fpix!(isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl<T: BaseInteger, const P: usize> From<FixedPoint<T, P>> for f32
where
    T: Integer,
{
    fn from(value: FixedPoint<T, P>) -> Self {
        value.value.to_f32() / (1 << P) as f32
    }
}

impl<T: BaseInteger, const P: usize> From<&FixedPoint<T, P>> for f32
where
    T: Integer,
{
    fn from(value: &FixedPoint<T, P>) -> Self {
        value.value.to_f32() / (1 << P) as f32
    }
}

impl<T: BaseInteger, const P: usize> From<FixedPoint<T, P>> for f64
where
    T: Integer,
{
    fn from(value: FixedPoint<T, P>) -> Self {
        value.value.to_f64() / (1 << P) as f64
    }
}

impl<T: BaseInteger, const P: usize> From<&FixedPoint<T, P>> for f64
where
    T: Integer,
{
    fn from(value: &FixedPoint<T, P>) -> Self {
        value.value.to_f64() / (1 << P) as f64
    }
}

// impl add
impl<T: BaseInteger, const P: usize> Add<Self> for FixedPoint<T, P>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Add<&Self> for FixedPoint<T, P>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Add<Self> for &FixedPoint<T, P>
where
    T: Add<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Add<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    T: Add<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn add(self, rhs: FixedPoint<T, P>) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}
// impl add done

// impl add_assign
impl<T: BaseInteger, const P: usize> AddAssign<Self> for FixedPoint<T, P>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl<T: BaseInteger, const P: usize> AddAssign<&Self> for FixedPoint<T, P>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.value += rhs.value;
    }
}
// impl add_assign done

// impl sub
impl<T: BaseInteger, const P: usize> Sub<Self> for FixedPoint<T, P>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Sub<&Self> for FixedPoint<T, P>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Sub<Self> for &FixedPoint<T, P>
where
    T: Sub<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Sub<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    T: Sub<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn sub(self, rhs: FixedPoint<T, P>) -> Self::Output {
        Self::Output {
            value: self.value - rhs.value,
        }
    }
}
// impl sub done

// impl sub_assign
impl<T: BaseInteger, const P: usize> SubAssign<Self> for FixedPoint<T, P>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl<T: BaseInteger, const P: usize> SubAssign<&Self> for FixedPoint<T, P>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.value -= rhs.value;
    }
}
// impl sub_assign done

// impl mul
impl<T: BaseInteger, const P: usize> Mul<Self> for FixedPoint<T, P>
where
    T: Mul<T, Output = T>,
    T: Shr<usize, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: (self.value * rhs.value) >> P,
        }
    }
}

impl<T: BaseInteger, const P: usize> Mul<&Self> for FixedPoint<T, P>
where
    T: Mul<T, Output = T> + Shr<usize, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        Self::Output {
            value: (self.value * rhs.value) >> P,
        }
    }
}

impl<T: BaseInteger, const P: usize> Mul<Self> for &FixedPoint<T, P>
where
    T: Mul<T, Output = T> + Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn mul(self, rhs: Self) -> Self::Output {
        println!("rr");
        Self::Output {
            value: (self.value * rhs.value) >> P,
        }
    }
}

impl<T: BaseInteger, const P: usize> Mul<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    T: Mul<T, Output = T> + Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn mul(self, rhs: FixedPoint<T, P>) -> Self::Output {
        println!("rv");
        Self::Output {
            value: (self.value * rhs.value) >> P,
        }
    }
}
// impl mul done

// impl mul_assign
impl<T: BaseInteger, const P: usize> MulAssign<Self> for FixedPoint<T, P>
where
    T: MulAssign<T> + ShrAssign<usize>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        self.value >>= P;
    }
}

impl<T: BaseInteger, const P: usize> MulAssign<&Self> for FixedPoint<T, P>
where
    T: MulAssign<T> + ShrAssign<usize>,
{
    fn mul_assign(&mut self, rhs: &Self) {
        self.value *= rhs.value;
        self.value >>= P;
    }
}
// impl mul_assign done

// impl div
impl<T: BaseInteger, const P: usize> Div<Self> for FixedPoint<T, P>
where
    T: Div<T, Output = T> + Shl<usize, Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: (self.value / rhs.value) << P,
        }
    }
}

impl<T: BaseInteger, const P: usize> Div<&Self> for FixedPoint<T, P>
where
    T: Div<T, Output = T> + Shl<usize, Output = T>,
{
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        Self::Output {
            value: (self.value / rhs.value) << P,
        }
    }
}

impl<T: BaseInteger, const P: usize> Div<Self> for &FixedPoint<T, P>
where
    T: Div<T, Output = T> + Shl<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: (self.value / rhs.value) << P,
        }
    }
}

impl<T: BaseInteger, const P: usize> Div<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    T: Div<T, Output = T> + Shl<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn div(self, rhs: FixedPoint<T, P>) -> Self::Output {
        Self::Output {
            value: (self.value / rhs.value) << P,
        }
    }
}
// impl div done

// impl div_assign
impl<T: BaseInteger, const P: usize> DivAssign<Self> for FixedPoint<T, P>
where
    T: DivAssign<T> + ShlAssign<usize>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
        self.value <<= P;
    }
}

impl<T: BaseInteger, const P: usize> DivAssign<&Self> for FixedPoint<T, P>
where
    T: DivAssign<T> + ShlAssign<usize>,
{
    fn div_assign(&mut self, rhs: &Self) {
        self.value /= rhs.value;
        self.value <<= P;
    }
}
// impl div_assign done

// impl rem
impl<T: BaseInteger, const P: usize> Rem<Self> for FixedPoint<T, P>
where
    T: Rem<T, Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value % rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Rem<&Self> for FixedPoint<T, P>
where
    T: Rem<T, Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: &Self) -> Self::Output {
        Self::Output {
            value: self.value % rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Rem<Self> for &FixedPoint<T, P>
where
    T: Rem<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value % rhs.value,
        }
    }
}

impl<T: BaseInteger, const P: usize> Rem<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    T: Rem<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn rem(self, rhs: FixedPoint<T, P>) -> Self::Output {
        Self::Output {
            value: self.value % rhs.value,
        }
    }
}
// impl rem done

// impl rem_assign
impl<T: BaseInteger, const P: usize> RemAssign<Self> for FixedPoint<T, P>
where
    T: RemAssign<T>,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

impl<T: BaseInteger, const P: usize> RemAssign<&Self> for FixedPoint<T, P>
where
    T: RemAssign<T>,
{
    fn rem_assign(&mut self, rhs: &Self) {
        self.value %= rhs.value;
    }
}
// impl rem_assign done

// impl shl
impl<T: BaseInteger, const P: usize> Shl<Self> for FixedPoint<T, P>
where
    T: Shl<T, Output = T> + Shr<usize, Output = T>,
{
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value << (rhs.value >> P),
        }
    }
}

impl<T: BaseInteger, const P: usize> Shl<&Self> for FixedPoint<T, P>
where
    T: Shl<T, Output = T> + Shr<usize, Output = T>,
{
    type Output = Self;

    fn shl(self, rhs: &Self) -> Self::Output {
        Self::Output {
            value: self.value << (rhs.value >> P),
        }
    }
}

impl<T: BaseInteger, const P: usize> Shl<Self> for &FixedPoint<T, P>
where
    T: Shl<T, Output = T> + Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn shl(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value << (rhs.value >> P),
        }
    }
}

impl<T: BaseInteger, const P: usize> Shl<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    T: Shl<T, Output = T> + Shr<usize, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn shl(self, rhs: FixedPoint<T, P>) -> Self::Output {
        Self::Output {
            value: self.value << (rhs.value >> P),
        }
    }
}
// impl shl done

// impl shl_assign
impl<T: BaseInteger, const P: usize> ShlAssign<Self> for FixedPoint<T, P>
where
    T: ShlAssign<T> + Shr<usize, Output = T>,
{
    fn shl_assign(&mut self, rhs: Self) {
        self.value <<= rhs.value >> P;
    }
}

impl<T: BaseInteger, const P: usize> ShlAssign<&Self> for FixedPoint<T, P>
where
    T: ShlAssign<T> + Shr<usize, Output = T>,
{
    fn shl_assign(&mut self, rhs: &Self) {
        self.value <<= rhs.value >> P;
    }
}
// impl shl_assign done

// impl shr
impl<T: BaseInteger, const P: usize> Shr<Self> for FixedPoint<T, P>
where
    T: Shr<usize, Output = T> + Shr<T, Output = T>,
{
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value >> (rhs.value >> P),
        }
    }
}

impl<T: BaseInteger, const P: usize> Shr<&Self> for FixedPoint<T, P>
where
    T: Shr<usize, Output = T> + Shr<T, Output = T>,
{
    type Output = Self;

    fn shr(self, rhs: &Self) -> Self::Output {
        Self::Output {
            value: self.value >> (rhs.value >> P),
        }
    }
}

impl<T: BaseInteger, const P: usize> Shr<Self> for &FixedPoint<T, P>
where
    T: Shr<usize, Output = T> + Shr<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn shr(self, rhs: Self) -> Self::Output {
        Self::Output {
            value: self.value >> (rhs.value >> P),
        }
    }
}

impl<T: BaseInteger, const P: usize> Shr<FixedPoint<T, P>> for &FixedPoint<T, P>
where
    T: Shr<usize, Output = T> + Shr<T, Output = T>,
{
    type Output = FixedPoint<T, P>;

    fn shr(self, rhs: FixedPoint<T, P>) -> Self::Output {
        Self::Output {
            value: self.value >> (rhs.value >> P),
        }
    }
}
// impl shr done

// impl shr_assign
impl<T: BaseInteger, const P: usize> ShrAssign<Self> for FixedPoint<T, P>
where
    T: Shr<usize, Output = T> + ShrAssign<T>,
{
    fn shr_assign(&mut self, rhs: Self) {
        self.value >>= rhs.value >> P;
    }
}

impl<T: BaseInteger, const P: usize> ShrAssign<&Self> for FixedPoint<T, P>
where
    T: Shr<usize, Output = T> + ShrAssign<T>,
{
    fn shr_assign(&mut self, rhs: &Self) {
        self.value >>= rhs.value >> P;
    }
}
// impl shr_assign done

// impl cmp
impl<T: BaseInteger, const P: usize> PartialEq<Self> for FixedPoint<T, P>
where
    T: PartialEq<T>,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T: BaseInteger, const P: usize> Eq for FixedPoint<T, P> where FixedPoint<T, P>: PartialEq<Self> {}

impl<T: BaseInteger, const P: usize> PartialOrd<Self> for FixedPoint<T, P>
where
    T: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl<T: BaseInteger, const P: usize> Ord for FixedPoint<T, P>
where
    T: Ord,
    FixedPoint<T, P>: PartialOrd<Self>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
// impl cmp done

// impl neg
impl<T: BaseInteger, const P: usize> Neg for FixedPoint<T, P>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: -self.value
        }
    }
}
// impl neg done
