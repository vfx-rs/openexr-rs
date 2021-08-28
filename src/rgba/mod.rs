#[allow(clippy::module_inception)]
pub mod rgba;
pub mod rgba_file;

pub use rgba::{Rgba, RgbaChannels};
pub use rgba_file::{RgbaInputFile, RgbaOutputFile};
