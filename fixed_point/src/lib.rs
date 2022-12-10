//!
//!
//!

mod fmt;
mod from;
mod into;

mod lib_inner;
pub use lib_inner::FixedPoint;

mod number;
mod ops_add;
mod ops_cmp;
mod ops_div;
mod ops_mul;
mod ops_neg;
mod ops_rem;
mod ops_shl;
mod ops_shr;
mod ops_sub;
