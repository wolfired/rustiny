//! TGAHeader.color_map_type:
//!     0=indicates that no color-map data is included with this image.
//!     1=indicates that a color-map is included with this image.
//!
//! TGAHeader.image_type:
//!     0=no image data included.
//!     1=uncompressed, color-mapped image.
//!     2=uncompressed, RGB image.
//!     3=uncompressed, black and white image.
//!     9=runlength encoded color-mapped image.
//!    10=runlength encoded RGB image.
//!    11=compressed, black and white image.
//!    32=compressed color-mapped data, using Huffman, Delta, and runlength encoding.
//!    33=compressed color-mapped data, using Huffman, Delta, and runlength encoding. 4-pass quadtree-type process.
//!

mod error;
pub use error::TGAError;

mod header;

mod image;
pub use image::TGAImage;
