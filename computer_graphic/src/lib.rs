//!
//!
//!

mod color;
pub use color::Color3;
pub use color::Color4;
pub use color::A;
pub use color::B;
pub use color::G;
pub use color::R;

mod line;
pub use line::line_bresenham;

mod point;
pub use point::Point2;
pub use point::Point3;
pub use point::Point4;
