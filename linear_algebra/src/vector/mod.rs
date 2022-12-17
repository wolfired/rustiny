//!
//!
//!

mod convert;

mod types;
pub use types::Vector;
pub use types::Vector2;
pub use types::Vector3;
pub use types::Vector4;

mod ops;
pub use ops::cross::Cross;
pub use ops::cross::CrossAssign;
pub use ops::dot::Dot;
pub use ops::xyzw::W;
pub use ops::xyzw::X;
pub use ops::xyzw::Y;
pub use ops::xyzw::Z;
