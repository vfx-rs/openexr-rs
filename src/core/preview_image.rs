use crate::Error;
use half::f16;
use openexr_sys as sys;

use crate::core::refptr::{OpaquePtr, Ref, RefMut};

type Result<T, E = Error> = std::result::Result<T, E>;

/// A usually small, low-dynamic range image,
/// that is intended to be stored in an image file's header
#[repr(transparent)]
pub struct PreviewImage(pub(crate) *mut sys::Imf_PreviewImage_t);

unsafe impl OpaquePtr for PreviewImage {
    type SysPointee = sys::Imf_PreviewImage_t;
    type Pointee = PreviewImage;
}

pub type PreviewImageRef<'a, P = PreviewImage> = Ref<'a, P>;
pub type PreviewImageRefMut<'a, P = PreviewImage> = RefMut<'a, P>;

impl PreviewImage {
    /// Creates a new [`PreviewImage`] with dimensions width = 0 and height = 0
    ///
    /// ## Errors
    /// * [`Error::Base`] - If an error occurs
    ///
    pub fn empty() -> Result<PreviewImage> {
        PreviewImage::with_dimensions(0, 0)
    }

    /// Creates a new [`PreviewImage`] with the provided dimensions
    ///
    /// ## Errors
    /// * [`Error::Overflow`] - If the size of the image is bigger than the supported size
    /// * [`Error::Base`] - If an error occurs
    ///
    pub fn with_dimensions(width: u32, height: u32) -> Result<PreviewImage> {
        let mut _inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_PreviewImage_ctor(
                &mut _inner,
                width,
                height,
                std::ptr::null(),
            )
            .into_result()?;
        }

        Ok(PreviewImage(_inner))
    }

    /// Creates a new [`PreviewImage`] with the provided dimensions and pixels
    ///
    /// ## Errors
    /// * [`Error::Overflow`] - If the size of the image is bigger than the supported size
    /// * [`Error::Base`] - If an error occurs
    ///
    pub fn with_pixels(
        width: u32,
        height: u32,
        pixels: &[PreviewRgba],
    ) -> Result<PreviewImage> {
        let mut _inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_PreviewImage_ctor(
                &mut _inner,
                width,
                height,
                pixels.as_ptr() as *const sys::Imf_PreviewRgba_t,
            )
            .into_result()?;
        }

        Ok(PreviewImage(_inner))
    }

    /// Returns the width of the [`PreviewImage`]
    ///
    pub fn width(&self) -> u32 {
        let mut value = 0;
        unsafe {
            sys::Imf_PreviewImage_width(self.0, &mut value)
                .into_result()
                .expect("Error getting 'width' property");
        }
        value
    }

    /// Returns the height of the [`PreviewImage`]
    ///
    pub fn height(&self) -> u32 {
        let mut value = 0;

        unsafe {
            sys::Imf_PreviewImage_height(self.0, &mut value)
                .into_result()
                .expect("Error getting 'height' property");
        }

        value
    }

    /// Get the image pixels as a slice
    ///
    pub fn pixels<'a>(&'a self) -> &'a [PreviewRgba] {
        let mut pixels = std::ptr::null();

        unsafe {
            sys::Imf_PreviewImage_pixels_const(self.0, &mut pixels)
                .into_result()
                .expect("Error getting pixels");

            if pixels.is_null() {
                panic!("Pixels pointer is null");
            }

            let pixel_count = (self.width() * self.height()) as usize;

            std::slice::from_raw_parts(
                pixels as *const PreviewRgba,
                pixel_count,
            )
        }
    }

    /// Get the image pixels as a mutable slice
    ///
    pub fn mut_pixels<'a>(&'a mut self) -> &'a mut [PreviewRgba] {
        let mut pixels = std::ptr::null_mut();

        unsafe {
            sys::Imf_PreviewImage_pixels(self.0, &mut pixels)
                .into_result()
                .expect("Error getting mutable pixels");

            if pixels.is_null() {
                panic!("Pixels pointer is null");
            }

            let pixel_count = (self.width() * self.height()) as usize;

            std::slice::from_raw_parts_mut(
                pixels as *mut PreviewRgba,
                pixel_count,
            )
        }
    }

    /// Sets the pixel value at the specified coordinates
    ///
    /// ## Errors
    /// * [`Error::OutOfRange`] - If the coordinates are outside of the image size
    /// * [`Error::Base`] - If an error occurs
    ///
    pub fn set_pixel(
        &mut self,
        x: u32,
        y: u32,
        value: PreviewRgba,
    ) -> Result<()> {
        unsafe {
            let mut pixel = std::ptr::null_mut();

            if x >= self.width() || y >= self.height() {
                return Err(Error::OutOfRange);
            }

            sys::Imf_PreviewImage_pixel(self.0, &mut pixel, x, y)
                .into_result()?;

            *(pixel as *mut PreviewRgba) = value
        }

        Ok(())
    }

    /// Gets the pixel value at the specified coordinates
    ///
    /// Returns [`None`] if the coordinates are outside of the image size,
    /// or some other error ocurred
    ///
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<PreviewRgba> {
        unsafe {
            let pixel = std::ptr::null_mut();

            if x >= self.width() || y >= self.height() {
                return None;
            }

            match sys::Imf_PreviewImage_pixel_const(self.0, pixel, x, y)
                .into_result()
            {
                Ok(_) => Some(*(pixel as *const PreviewRgba)),
                Err(_) => None,
            }
        }
    }
}

impl Clone for PreviewImage {
    fn clone(&self) -> Self {
        let mut _inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_PreviewImage_copy(&mut _inner, self.0)
                .into_result()
                .expect("Error invoking copy ctor");
        }

        PreviewImage(_inner)
    }
}

impl Drop for PreviewImage {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_PreviewImage_dtor(self.0);
        }
    }
}

/// Holds the value of a PreviewImage pixel
///
/// Intensity is proportional to `pow(x/255, 2.2)` for `r`, `g`, and `b` components.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PreviewRgba {
    /// Red component of the pixel's color.
    /// Intensity is proportional to `pow(x/255, 2.2)`.
    pub r: u8,

    /// Green component of the pixel's color.
    /// Intensity is proportional to `pow(x/255, 2.2)`.
    pub g: u8,

    /// Blue component of the pixel's color.
    /// Intensity is proportional to `pow(x/255, 2.2)`.
    pub b: u8,

    /// The pixel's alpha.
    /// Transparent -> 0. Opaque -> 255.
    pub a: u8,
}

impl PreviewRgba {
    /// Creates a [`PreviewRgba`] from RGB and Alpha [`u8`] values
    ///
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> PreviewRgba {
        PreviewRgba { r, g, b, a }
    }

    /// Creates a [`PreviewRgba`] from RGB and Alpha [`f16`] values
    ///
    /// ## Note
    /// Applies gamma correction with the formula
    /// `clamp( pow(5.5555 * max(0, x), 0.4545) * 84.66), 0, 255)`
    ///
    pub fn from_f16(r: f16, g: f16, b: f16, a: f16) -> PreviewRgba {
        PreviewRgba::from_f32(
            f16::to_f32(r),
            f16::to_f32(g),
            f16::to_f32(b),
            f16::to_f32(a),
        )
    }

    /// Creates a [`PreviewRgba`] from RGB and Alpha [`f32`] values
    ///
    /// ## Note
    /// Applies gamma correction with the formula
    /// `clamp( pow(5.5555 * max(0, x), 0.4545) * 84.66), 0, 255)`
    ///
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> PreviewRgba {
        PreviewRgba {
            r: PreviewRgba::gamma(r),
            g: PreviewRgba::gamma(g),
            b: PreviewRgba::gamma(b),
            a: (f32::clamp(a * 255f32, 0f32, 255f32) + 0.5f32) as u8,
        }
    }

    /// Creates a [`PreviewRgba`] from RGB and Alpha [`f16`] values
    ///
    /// ## Note
    /// Does not apply gamma correction
    ///
    pub fn from_f16_linear(r: f16, g: f16, b: f16, a: f16) -> PreviewRgba {
        PreviewRgba::from_f32(
            f16::to_f32(r),
            f16::to_f32(g),
            f16::to_f32(b),
            f16::to_f32(a),
        )
    }

    /// Creates a [`PreviewRgba`] from RGB and Alpha [`f32`] values
    ///
    /// ## Note
    /// Does not apply gamma correction
    ///
    pub fn from_f32_linear(r: f32, g: f32, b: f32, a: f32) -> PreviewRgba {
        PreviewRgba {
            r: PreviewRgba::gamma(r),
            g: PreviewRgba::gamma(g),
            b: PreviewRgba::gamma(b),
            a: (f32::clamp(a * 255f32, 0f32, 255f32) + 0.5f32) as u8,
        }
    }

    /// Creates a [`PreviewRgba`] where all components have value 0 (zero)
    ///
    pub fn zero() -> PreviewRgba {
        PreviewRgba::from_u8(0u8, 0u8, 0u8, 0u8)
    }

    /// Convert a floating-point pixel value to an 8-bit gamma-2.2
    /// preview pixel.  (This routine is a simplified version of
    /// how the exrdisplay program transforms floating-point pixel
    /// values in order to display them on the screen.)
    ///
    /// ## Note
    /// Adapted from `previewImageExamples.cpp`
    ///
    fn gamma(linear: f32) -> u8 {
        let gamma =
            f32::powf(5.5555f32 * f32::max(0f32, linear), 0.4545f32) * 84.66f32;

        f32::clamp(gamma, 0f32, 255f32) as u8
    }
}

impl From<crate::rgba::Rgba> for PreviewRgba {
    fn from(rgba: crate::rgba::Rgba) -> Self {
        PreviewRgba::from_f16(rgba.r, rgba.g, rgba.b, rgba.a)
    }
}

impl Default for PreviewRgba {
    fn default() -> PreviewRgba {
        PreviewRgba::from_u8(0u8, 0u8, 0u8, 255u8)
    }
}

#[cfg(test)]
mod tests {
    use super::Result;
    use crate::core::header::Header;
    use crate::core::preview_image::{PreviewImage, PreviewRgba};
    use crate::rgba::{
        rgba::RgbaChannels,
        rgba_file::{RgbaInputFile, RgbaOutputFile},
    };
    use crate::tests::load_ferris;
    use half::f16;
    use std::path::PathBuf;

    #[test]
    fn test_empty_preview_image() -> Result<()> {
        PreviewImage::empty()?;
        Ok(())
    }

    #[test]
    fn test_preview_image_with_ok_dimensions() -> Result<()> {
        PreviewImage::with_dimensions(0, 0)?;

        PreviewImage::with_dimensions(1, 1)?;
        PreviewImage::with_dimensions(1, 0)?;
        PreviewImage::with_dimensions(0, 1)?;

        PreviewImage::with_dimensions(8, 1)?;

        PreviewImage::with_dimensions(1024, 1024)?;

        PreviewImage::with_dimensions(4096, 4096)?;

        Ok(())
    }

    #[test]
    fn test_preview_image_with_unsupported_dimensions() {
        assert!(PreviewImage::with_dimensions(std::u32::MAX, std::u32::MAX)
            .is_err());
    }

    #[test]
    fn test_preview_image_with_pixels() -> Result<()> {
        const PREVIEW_WIDTH: u32 = 128;
        const PREVIEW_HEIGHT: u32 = 64;

        let pixels =
            [PreviewRgba::zero(); (PREVIEW_WIDTH * PREVIEW_HEIGHT) as usize];

        PreviewImage::with_pixels(PREVIEW_WIDTH, PREVIEW_HEIGHT, &pixels)?;

        Ok(())
    }

    #[test]
    fn test_clone_preview_image() -> Result<()> {
        const PREVIEW_WIDTH: u32 = 128;
        const PREVIEW_HEIGHT: u32 = 64;

        let pixels =
            [PreviewRgba::zero(); (PREVIEW_WIDTH * PREVIEW_HEIGHT) as usize];

        let preview_image =
            PreviewImage::with_pixels(PREVIEW_WIDTH, PREVIEW_HEIGHT, &pixels)?;

        let _ = preview_image.clone();

        Ok(())
    }

    #[test]
    fn test_read_file_without_preview() -> Result<()> {
        let path = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR not set"),
        )
        .join("images")
        .join("comp_piz.exr");

        let file = RgbaInputFile::new(&path, 1).unwrap();

        assert_eq!(file.header().has_preview_image(), false);

        Ok(())
    }

    #[test]
    fn test_read_file_with_preview() -> Result<()> {
        const PREVIEW_WIDTH: u32 = 128;
        const PREVIEW_HEIGHT: u32 = 85;

        let path = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR")
                .expect("CARGO_MANIFEST_DIR not set"),
        )
        .join("images")
        .join("ferris-preview.exr");

        let file = RgbaInputFile::new(&path, 1).unwrap();

        let header = file.header();

        assert!(header.has_preview_image());

        let preview_image = header.preview_image().unwrap();

        assert_eq!(preview_image.width(), PREVIEW_WIDTH);
        assert_eq!(preview_image.height(), PREVIEW_HEIGHT);

        assert!(preview_image
            .pixels()
            .iter()
            .any(|&x| x != PreviewRgba::zero()));

        Ok(())
    }

    #[test]
    fn test_preview_roundtrip() -> Result<()> {
        const ROUNDTRIP_FILE: &str = "preview_rtrip.exr";
        const PREVIEW_WIDTH: u32 = 128;
        const PREVIEW_HEIGHT: u32 = 64;

        let (pixels, width, height) = load_ferris();

        let mut preview_image = PreviewImage::with_dimensions(
            PREVIEW_WIDTH as u32,
            PREVIEW_HEIGHT as u32,
        )?;

        let preview_pixels = preview_image.mut_pixels();
        for y in 0..PREVIEW_HEIGHT {
            for x in 0..PREVIEW_WIDTH {
                let pixel = &pixels[(x + PREVIEW_WIDTH * y) as usize];

                preview_pixels[(x + PREVIEW_WIDTH * y) as usize] =
                    PreviewRgba::from_f16(pixel.r, pixel.g, pixel.b, pixel.a);
            }
        }

        let mut header = Header::from_dimensions(width, height);

        header.set_preview_image(&preview_image)?;

        let mut output_file = RgbaOutputFile::new(
            ROUNDTRIP_FILE,
            &header,
            RgbaChannels::WriteRgba,
            1,
        )?;

        output_file.set_frame_buffer(&pixels, 1, width as usize)?;
        output_file.write_pixels(height)?;

        std::mem::drop(output_file);

        let input_file = RgbaInputFile::new(ROUNDTRIP_FILE, 4)?;

        let header = input_file.header();

        assert!(header.has_preview_image());

        let preview_image = header.preview_image().unwrap();

        assert_eq!(preview_image.width(), PREVIEW_WIDTH);
        assert_eq!(preview_image.height(), PREVIEW_HEIGHT);

        let preview_pixels = preview_image.pixels();

        assert_eq!(
            preview_pixels.len(),
            (PREVIEW_WIDTH * PREVIEW_HEIGHT) as usize
        );

        for y in 0..PREVIEW_HEIGHT {
            for x in 0..PREVIEW_WIDTH {
                let file_preview_pixel =
                    &preview_pixels[(x + PREVIEW_WIDTH * y) as usize];

                let pixel = &pixels[(x + PREVIEW_WIDTH * y) as usize];
                let computed_preview_pixel =
                    PreviewRgba::from_f16(pixel.r, pixel.g, pixel.b, pixel.a);

                assert_eq!(
                    file_preview_pixel.r, computed_preview_pixel.r,
                    "Red value not matching for pixel at x: {} y: {}",
                    x, y
                );
                assert_eq!(
                    file_preview_pixel.g, computed_preview_pixel.g,
                    "Green value not matching for pixel at x: {} y: {}",
                    x, y
                );
                assert_eq!(
                    file_preview_pixel.b, computed_preview_pixel.b,
                    "Blue value not matching for pixel at x: {} y: {}",
                    x, y
                );
                assert_eq!(
                    file_preview_pixel.a, computed_preview_pixel.a,
                    "Alpha value not matching for pixel at x: {} y: {}",
                    x, y
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_preview_rgba_f32_gamma_conversion() {
        let preview_black = PreviewRgba::from_f32(0f32, 0f32, 0f32, 0f32);

        assert_eq!(
            preview_black.r, 0,
            "Mismatch red channel value on black pixel"
        );
        assert_eq!(
            preview_black.g, 0,
            "Mismatch green channel value on black pixel"
        );
        assert_eq!(
            preview_black.b, 0,
            "Mismatch blue channel value on black pixel"
        );
        assert_eq!(
            preview_black.a, 0,
            "Mismatch alpha channel value on black pixel"
        );

        let preview_white = PreviewRgba::from_f32(1f32, 1f32, 1f32, 1f32);

        assert_eq!(
            preview_white.r, 184,
            "Mismatch red channel value on white pixel"
        );
        assert_eq!(
            preview_white.g, 184,
            "Mismatch green channel value on white pixel"
        );
        assert_eq!(
            preview_white.b, 184,
            "Mismatch blue channel value on white pixel"
        );
        assert_eq!(
            preview_white.a, 255,
            "Mismatch alpha channel value on white pixel"
        );

        let preview_bright_white =
            PreviewRgba::from_f32(2.037f32, 2.037f32, 2.037f32, 1f32);

        assert_eq!(
            preview_bright_white.r, 255,
            "Mismatch red channel value on bright white pixel"
        );
        assert_eq!(
            preview_bright_white.g, 255,
            "Mismatch green channel value on bright white pixel"
        );
        assert_eq!(
            preview_bright_white.b, 255,
            "Mismatch blue channel value on bright white pixel"
        );
        assert_eq!(
            preview_bright_white.a, 255,
            "Mismatch alpha channel value on white pixel"
        );

        let preview_grey =
            PreviewRgba::from_f32(0.5f32, 0.5f32, 0.5f32, 0.5f32);

        assert_eq!(
            preview_grey.r, 134,
            "Mismatch red channel value on grey pixel"
        );
        assert_eq!(
            preview_grey.g, 134,
            "Mismatch green channel value on grey pixel"
        );
        assert_eq!(
            preview_grey.b, 134,
            "Mismatch blue channel value on grey pixel"
        );
        assert_eq!(
            preview_grey.a, 128,
            "Mismatch alpha channel value on grey pixel"
        );
    }

    #[test]
    fn test_preview_rgba_f16_gamma_conversion() {
        let preview_black = PreviewRgba::from_f16(
            f16::from_f32(0f32),
            f16::from_f32(0f32),
            f16::from_f32(0f32),
            f16::from_f32(0f32),
        );

        assert_eq!(
            preview_black.r, 0,
            "Mismatch red channel value on black pixel"
        );
        assert_eq!(
            preview_black.g, 0,
            "Mismatch green channel value on black pixel"
        );
        assert_eq!(
            preview_black.b, 0,
            "Mismatch blue channel value on black pixel"
        );
        assert_eq!(
            preview_black.a, 0,
            "Mismatch alpha channel value on black pixel"
        );

        let preview_white = PreviewRgba::from_f16(
            f16::from_f32(1f32),
            f16::from_f32(1f32),
            f16::from_f32(1f32),
            f16::from_f32(1f32),
        );

        assert_eq!(
            preview_white.r, 184,
            "Mismatch red channel value on white pixel"
        );
        assert_eq!(
            preview_white.g, 184,
            "Mismatch green channel value on white pixel"
        );
        assert_eq!(
            preview_white.b, 184,
            "Mismatch blue channel value on white pixel"
        );
        assert_eq!(
            preview_white.a, 255,
            "Mismatch alpha channel value on white pixel"
        );

        let preview_bright_white = PreviewRgba::from_f16(
            f16::from_f32(2.037f32),
            f16::from_f32(2.037f32),
            f16::from_f32(2.037f32),
            f16::from_f32(1f32),
        );

        assert_eq!(
            preview_bright_white.r, 255,
            "Mismatch red channel value on bright white pixel"
        );
        assert_eq!(
            preview_bright_white.g, 255,
            "Mismatch green channel value on bright white pixel"
        );
        assert_eq!(
            preview_bright_white.b, 255,
            "Mismatch blue channel value on bright white pixel"
        );
        assert_eq!(
            preview_bright_white.a, 255,
            "Mismatch alpha channel value on white pixel"
        );

        let preview_grey = PreviewRgba::from_f16(
            f16::from_f32(0.5f32),
            f16::from_f32(0.5f32),
            f16::from_f32(0.5f32),
            f16::from_f32(0.5f32),
        );

        assert_eq!(
            preview_grey.r, 134,
            "Mismatch red channel value on grey pixel"
        );
        assert_eq!(
            preview_grey.g, 134,
            "Mismatch green channel value on grey pixel"
        );
        assert_eq!(
            preview_grey.b, 134,
            "Mismatch blue channel value on grey pixel"
        );
        assert_eq!(
            preview_grey.a, 128,
            "Mismatch alpha channel value on grey pixel"
        );
    }
}
