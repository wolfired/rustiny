//!
//!
//!

mod convert;
mod fmt;

mod types;
pub use types::Matrix;

mod ops;
pub use ops::mul_scalar::MulScalar;
pub use ops::mul_scalar::MulScalarAssign;
pub use ops::transpose::Transpose;
