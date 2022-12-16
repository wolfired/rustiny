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

mod convert;
mod fmt;

mod lib_inner;
pub use lib_inner::FixedPoint;

mod ops_abs;
mod ops_add;
mod ops_cmp;
mod ops_div;
mod ops_mul;
mod ops_neg;
mod ops_ones;
mod ops_pow;
mod ops_rem;
mod ops_shl;
mod ops_shr;
mod ops_sqrt;
mod ops_sub;
mod ops_tofxx;
mod ops_zeros;
