use half::f16;
use imath_traits::Zero;
use openexr_sys as sys;
pub use sys::RgbaChannels;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Rgba {
    pub r: f16,
    pub g: f16,
    pub b: f16,
    pub a: f16,
}

impl Rgba {
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Rgba {
            r: f16::from_f32(r),
            g: f16::from_f32(g),
            b: f16::from_f32(b),
            a: f16::from_f32(a),
        }
    }
}

impl Zero for Rgba {
    fn zero() -> Self {
        Rgba {
            r: f16::ZERO,
            g: f16::ZERO,
            b: f16::ZERO,
            a: f16::ZERO,
        }
    }
}
