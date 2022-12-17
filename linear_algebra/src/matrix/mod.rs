//!
//!
//!

mod convert;
mod fmt;

mod mod_inner;
pub use mod_inner::Matrix;

mod ops_add;

mod ops_mul_scalar;
pub use ops_mul_scalar::MulScalar;
pub use ops_mul_scalar::MulScalarAssign;

mod ops_mul;
mod ops_sub;

mod ops_transpose;
pub use ops_transpose::Transpose;
