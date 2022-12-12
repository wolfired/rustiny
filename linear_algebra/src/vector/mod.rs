//!
//!
//!

mod from;

mod mod_inner;
pub use mod_inner::Vector;
pub use mod_inner::Vector2;
pub use mod_inner::Vector3;
pub use mod_inner::Vector4;

mod ops_cross;
pub use ops_cross::Cross;
pub use ops_cross::CrossAssign;

mod ops_dot;
pub use ops_dot::Dot;

mod ops_xyzw;
pub use ops_xyzw::W;
pub use ops_xyzw::X;
pub use ops_xyzw::Y;
pub use ops_xyzw::Z;
