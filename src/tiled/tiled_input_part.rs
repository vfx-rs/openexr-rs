use crate::{
    core::{
        error::Error,
        frame_buffer::{FrameBuffer, FrameBufferRef},
        header::HeaderRef,
        version::Version,
        LevelMode, LevelRoundingMode,
    },
    multi_part::multi_part_input_file::MultiPartInputFile,
};

use imath_traits::Bound2;
use openexr_sys as sys;
use std::ffi::CStr;
use std::fmt::Debug;
use std::path::PathBuf;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct TiledInputPart<'a> {
    pub(crate) inner: sys::Imf_TiledInputPart_t,
    // The MultiPartInputFile is borrowed in the TiledInputPart, so we need to
    // make sure that its lifetime is longer than the TiledInputPart.
    phantom: std::marker::PhantomData<&'a MultiPartInputFile>,
}

impl<'a> Debug for TiledInputPart<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TiledInputPart")
            .field("file_name", &self.file_name())
            .finish()
    }
}

impl<'a> TiledInputPart<'a> {
    pub fn new(
        multi_part_file: &'a MultiPartInputFile,
        part_number: i32,
    ) -> Self {
        let mut inner = std::mem::MaybeUninit::uninit();

        unsafe {
            sys::Imf_TiledInputPart_ctor(
                inner.as_mut_ptr(),
                multi_part_file.0,
                part_number,
            )
            .into_result()
            .unwrap();

            Self {
                inner: inner.assume_init(),
                phantom: std::marker::PhantomData,
            }
        }
    }

    /// Access to the file path
    ///
    pub fn file_name(&self) -> PathBuf {
        let mut ptr = std::ptr::null();

        unsafe {
            sys::Imf_TiledInputPart_fileName(&self.inner, &mut ptr)
                .into_result()
                .unwrap();

            // TODO: Validate if OpenEXR will always return a valid string, or
            // if we need to return a Result<PathBuf, Error>.
            if ptr.is_null() {
                panic!(
                    "Received null ptr from sys::Imf_TiledInputPart_fileName"
                );
            }

            let file_name = CStr::from_ptr(ptr);

            PathBuf::from(
                file_name.to_str().expect("Invalid bytes in filename"),
            )
        }
    }

    /// Access to the file header
    ///
    pub fn header(&self) -> HeaderRef {
        let mut ptr = std::ptr::null();

        unsafe {
            sys::Imf_TiledInputPart_header(&self.inner, &mut ptr)
                .into_result()
                .unwrap();

            // TODO: Validate if OpenEXR will always return a valid header, or
            // if we need to return a Result<Header, Error>.
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_TiledInputPart_header");
            }
        }

        HeaderRef::new(ptr)
    }

    /// Access to the file format version
    ///
    pub fn version(&self) -> Version {
        let mut version = 0;

        unsafe {
            sys::Imf_TiledInputPart_version(&self.inner, &mut version)
                .into_result()
                .unwrap();
        }

        Version::from_c_int(version)
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the TiledInputPart object.
    ///
    /// The current frame buffer is the destination for the pixel
    /// data read from the file.  The current frame buffer must be
    /// set at least once before read_tile() is called.
    /// The current frame buffer can be changed after each call
    /// to read_tile().
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if the sampling factors do not match or
    /// if the frame buffer does not have a sample count slice.
    ///
    pub fn set_frame_buffer(
        &mut self,
        frame_buffer: &FrameBuffer,
    ) -> Result<()> {
        unsafe {
            // Assume that the frame buffer is valid, since it is coming from
            // Rust.
            sys::Imf_TiledInputPart_setFrameBuffer(
                &mut self.inner,
                frame_buffer.ptr,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Access to the current frame buffer
    ///
    pub fn frame_buffer(&self) -> FrameBufferRef {
        let mut ptr = std::ptr::null();

        unsafe {
            sys::Imf_TiledInputPart_frameBuffer(&self.inner, &mut ptr)
                .into_result()
                .unwrap();

            // TODO: Validate if OpenEXR will always return a valid frame
            // buffer, or if we need to return a Result<FrameBufferRef, Error>.
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_TiledInputPart_frameBuffer");
            }
        }

        FrameBufferRef::new(ptr)
    }

    /// Check if all pixels in the data window are present in the input file
    ///
    pub fn is_complete(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_TiledInputPart_isComplete(&self.inner, &mut v);
        }

        v
    }

    /// Get the tiles' x dimension
    ///
    pub fn tile_x_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_tileXSize(&self.inner, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputPart_tileXSize",
                );
        }
        v
    }

    /// Get the tiles' y dimension
    ///
    pub fn tile_y_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_tileYSize(&self.inner, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputPart_tileYSize",
                );
        }
        v
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_TiledInputPart_levelMode(&self.inner, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputPart_levelMode",
                );
        }

        v.into()
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_TiledInputPart_levelRoundingMode(
                &self.inner,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_TiledInputPart_levelRoundingMode",
            );
        }

        v.into()
    }

    /// Get the number of levels in the file
    ///
    /// # Returns
    /// * `Ok(1)` if [`TiledInputPart::level_mode()`] == [`LevelMode::OneLevel`]
    /// * `Ok(rfunc (log (max (w, h)) / log (2)) + 1)` if [`TiledInputPart::level_mode()`] == [`LevelMode::MipmapLevels`]

    /// * `Err(Error::Logic)` if [`TiledInputPart::level_mode()`] == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`TiledInputPart::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_numLevels(&self.inner, &mut v)
                .into_result()?;
        }

        Ok(v)
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`TiledInputPart::level_mode()`] == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`TiledInputPart::level_mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (w) / log (2)) + 1` if [`TiledInputPart::level_mode()`] == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`TiledInputPart::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_numXLevels(&self.inner, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputPart_numXLevels",
                );
        }
        v
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`TiledInputPart::level_mode()`] == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`TiledInputPart::level_mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (h) / log (2)) + 1` if [`TiledInputPart::level_mode()`] == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`TiledInputPart::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_numYLevels(&self.inner, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputPart_numYLevels",
                );
        }
        v
    }

    /// Returns `true` if the file contains a level with level number `(lx, ly)`, `false`
    /// otherwise.
    ///
    pub fn is_valid_level(&self, lx: i32, ly: i32) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_TiledInputPart_isValidLevel(&self.inner, &mut v, lx, ly)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputPart_isValidLevel",
                );
        }
        v
    }

    /// Returns the width of the level with level number `(lx, *)`, where `*` is any number.
    ///
    /// # Returns
    /// * `max (1, rfunc (w / pow (2, lx)))`
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`TiledInputPart::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_levelWidth(&self.inner, &mut v, lx)
                .into_result()?;
        }
        Ok(v)
    }

    /// Returns the height of the level with level number `(*, ly)`, where `*` is any number.
    ///
    /// # Returns
    /// * `max (1, rfunc (h / pow (2, ly)))`
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`TiledInputPart::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_levelHeight(&self.inner, &mut v, ly)
                .into_result()?;
        }
        Ok(v)
    }

    /// Get the number of tiles in the x axis that cover a level with level number `(lx, *)`
    /// where `*` is any number
    ///
    /// # Returns
    /// *(level_width(lx) + tile_x_size() - 1) / tile_x_size()
    ///
    /// # Errors
    /// *[`Error::InvalidArgument`] - If `lx` is not a valid level
    ///
    pub fn num_x_tiles(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_numXTiles(&self.inner, &mut v, lx)
                .into_result()?;
        }
        Ok(v)
    }

    /// Get the number of tiles in the y axis that cover a level with level number `(*, ly)`
    /// where `*` is any number
    ///
    /// # Returns
    /// * (level_height(ly) + tile_y_size() - 1) / tile_y_size()
    ///
    /// # Errors
    /// *[`Error::InvalidArgument`] - If `lx` is not a valid level
    ///
    pub fn num_y_tiles(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputPart_numYTiles(&self.inner, &mut v, ly)
                .into_result()?;
        }
        Ok(v)
    }

    /// Returns a 2-dimensional region of valid pixel coordinates for a level with level number `(lx, ly)`
    ///
    /// # Errors
    /// *[`Error::Base`] - if any error occurs
    ///
    pub fn data_window_for_level<B: Bound2<i32>>(
        &self,
        lx: i32,
        ly: i32,
    ) -> Result<B> {
        let mut dw = [0i32; 4];
        unsafe {
            sys::Imf_TiledInputPart_dataWindowForLevel(
                &self.inner,
                dw.as_mut_ptr() as *mut sys::Imath_Box2i_t,
                lx,
                ly,
            )
            .into_result()?;
        }

        Ok(B::from_slice(&dw))
    }

    /// Returns a 2-dimensional region of valid pixel coordinates for a level with  tile coordinates `(dx, dy)` and level number `(lx, ly)`
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if the passed tile coordinates are invalid
    /// * [`Error::Base`] - if any other error occurs
    ///
    pub fn data_window_for_tile<B: Bound2<i32>>(
        &self,
        dx: i32,
        dy: i32,
        lx: i32,
        ly: i32,
    ) -> Result<B> {
        let mut dw = [0i32; 4];
        unsafe {
            sys::Imf_TiledInputPart_dataWindowForTile(
                &self.inner,
                dw.as_mut_ptr() as *mut sys::Imath_Box2i_t,
                dx,
                dy,
                lx,
                ly,
            )
            .into_result()?;
        }

        Ok(B::from_slice(&dw))
    }

    /// Reads the tile with tile coordinates `(dx, dy)`, and level number `(lx, ly)`,
    /// and stores it in the current frame buffer.
    ///
    ///   # Errors
    /// * [`Error::InvalidArgument`] - if dx does not lie in the interval [0, num_x_tiles(lx)-1]
    /// * [`Error::InvalidArgument`] - if dy does not lie in the interval [0, num_y_tiles(ly)-1]
    ///
    /// * [`Error::InvalidArgument`] -if lx does not lie in the interval [0, num_x_levels()-1]
    /// * [`Error::InvalidArgument`] -if ly does not lie in the inverval [0, num_y_levels()-1]
    /// * [`Error::Io`] - if there is an error reading data from the file
    /// * [`Error::Base`] - if any other error occurs
    ///
    pub fn read_tile(
        &mut self,
        dx: i32,
        dy: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        unsafe {
            sys::Imf_TiledInputPart_readTile(&mut self.inner, dx, dy, lx, ly)
                .into_result()?;
        }
        Ok(())
    }

    /// Reads the sample counts in tile range with coordinates `(dx1, dy1)`, to `(dx2, dy2)` and level number `(lx, ly)`,
    /// and stores it in the current frame buffer.
    ///
    ///   # Errors
    /// * [`Error::InvalidArgument`] - if dx does not lie in the interval [0, num_x_tiles(lx)-1]
    /// * [`Error::InvalidArgument`] - if dy does not lie in the interval [0, num_y_tiles(ly)-1]
    ///
    /// * [`Error::InvalidArgument`] -if lx does not lie in the interval [0, num_x_levels()-1]
    /// * [`Error::InvalidArgument`] -if ly does not lie in the inverval [0, num_y_levels()-1]
    /// * [`Error::Io`] - if there is an error reading data from the file
    /// * [`Error::Base`] - if any other error occurs
    ///
    pub fn read_tiles(
        &mut self,
        dx1: i32,
        dx2: i32,
        dy1: i32,
        dy2: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        unsafe {
            sys::Imf_TiledInputPart_readTiles(
                &mut self.inner,
                dx1,
                dx2,
                dy1,
                dy2,
                lx,
                ly,
            )
            .into_result()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        core::{frame_buffer::FrameBuffer, LevelMode, LevelRoundingMode},
        multi_part::multi_part_input_file::MultiPartInputFile,
        tiled::tiled_input_part::TiledInputPart,
    };

    lazy_static::lazy_static! {
        static ref SRC_IMAGE_PATH: PathBuf = {
            PathBuf::from(
                std::env::var("CARGO_MANIFEST_DIR")
                    .expect("CARGO_MANIFEST_DIR not set"),
            )
            .join("images")
            .join("ferris-tiled.exr")
        };
    }

    #[test]
    fn test_tiledinputpart_file_name_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = TiledInputPart::new(&input, 0);
        let result = tiled_input_part.file_name();

        assert_eq!(&*SRC_IMAGE_PATH, &result);
    }

    #[test]
    fn test_tiledinputpart_header_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = TiledInputPart::new(&input, 0);
        let header = tiled_input_part.header();

        header.sanity_check(true, false).unwrap();
    }

    #[test]
    fn test_tiledinputpart_version_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let version = tiled_input_part.version();

        assert_eq!(version.is_tiled(), true);
    }

    #[test]
    fn test_tiledinputpart_set_frame_buffer_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let mut tiled_input_part = super::TiledInputPart::new(&input, 0);
        let frame_buffer = FrameBuffer::new();
        tiled_input_part.set_frame_buffer(&frame_buffer).unwrap();
    }

    #[test]
    fn test_tiledinputpart_frame_buffer_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let _ = tiled_input_part.frame_buffer();
    }

    #[test]
    fn test_tiledinputpart_is_complete_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let is_complete = tiled_input_part.is_complete();

        assert_eq!(is_complete, true);
    }

    #[test]
    fn test_tiledinputpart_tile_x_size_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let tile_x_size = tiled_input_part.tile_x_size();

        assert_eq!(tile_x_size, 64);
    }

    #[test]
    fn test_tiledinputpart_tile_y_size_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let tile_y_size = tiled_input_part.tile_y_size();

        assert_eq!(tile_y_size, 64);
    }

    #[test]
    fn test_tiledinputpart_level_mode_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let level_mode = tiled_input_part.level_mode();

        assert_eq!(level_mode, LevelMode::OneLevel);
    }

    #[test]
    fn test_tiledinputpart_level_rounding_mode_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let level_rounding_mode = tiled_input_part.level_rounding_mode();

        assert_eq!(level_rounding_mode, LevelRoundingMode::RoundDown);
    }

    #[test]
    fn test_tiledinputpart_num_levels_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let num_levels = tiled_input_part.num_levels().unwrap();

        assert_eq!(num_levels, 1);
    }

    #[test]
    fn test_tiledinputpart_num_x_levels_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let num_x_levels = tiled_input_part.num_x_levels();

        assert_eq!(num_x_levels, 1);
    }

    #[test]
    fn test_tiledinputpart_num_y_levels_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let num_y_levels = tiled_input_part.num_y_levels();

        assert_eq!(num_y_levels, 1);
    }

    #[test]
    fn test_tiledinputpart_is_valid_level_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let is_valid_level = tiled_input_part.is_valid_level(0, 0);

        assert_eq!(is_valid_level, true);
    }

    #[test]
    fn test_tiledinputpart_is_valid_level_failure_invalid_level() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let is_valid_level = tiled_input_part.is_valid_level(100, 100);

        assert_eq!(is_valid_level, false);
    }

    #[test]
    fn test_tiledinputpart_level_width_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let level_width = tiled_input_part.level_width(0).unwrap();

        assert_eq!(level_width, 1200);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputpart_level_width_failure_invalid_level() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        tiled_input_part.level_width(i32::MIN).unwrap();
    }

    #[test]
    fn test_tiledinputpart_level_height_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let level_height = tiled_input_part.level_height(0).unwrap();

        assert_eq!(level_height, 800);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputpart_level_height_failure_invalid_level() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        tiled_input_part.level_height(i32::MIN).unwrap();
    }

    #[test]
    fn test_tiledinputpart_num_x_tiles_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let num_x_tiles = tiled_input_part.num_x_tiles(0).unwrap();

        assert_eq!(num_x_tiles, 19);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputpart_num_x_tiles_failed_invalid_level() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        tiled_input_part.num_x_tiles(i32::MIN).unwrap();
    }

    #[test]
    fn test_tiledinputpart_num_y_tiles_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let num_y_tiles = tiled_input_part.num_y_tiles(0).unwrap();

        assert_eq!(num_y_tiles, 13);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputpart_num_y_tiles_failed_invalid_level() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        tiled_input_part.num_y_tiles(i32::MIN).unwrap();
    }

    #[test]
    fn test_tiledinputpart_data_window_for_level_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let data_window_for_level: [i32; 4] =
            tiled_input_part.data_window_for_level(0, 0).unwrap();

        assert_eq!(data_window_for_level, [0, 0, 1199, 799]);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputpart_data_window_for_level_failure_invalid_level() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        tiled_input_part
            .data_window_for_level::<[i32; 4]>(i32::MIN, i32::MIN)
            .unwrap();
    }

    #[test]
    fn test_tiledinputpart_data_window_for_tile_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        let data_window_for_tile: [i32; 4] =
            tiled_input_part.data_window_for_tile(0, 0, 0, 0).unwrap();

        assert_eq!(data_window_for_tile, [0, 0, 63, 63]);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputpart_data_window_for_tile_failure_invalid_level() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let tiled_input_part = super::TiledInputPart::new(&input, 0);
        tiled_input_part
            .data_window_for_tile::<[i32; 4]>(
                i32::MIN,
                i32::MIN,
                i32::MIN,
                i32::MIN,
            )
            .unwrap();
    }

    #[test]
    fn test_tiledinputpart_read_tile_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let mut tiled_input_part = super::TiledInputPart::new(&input, 0);
        let frame_buffer = FrameBuffer::new();
        tiled_input_part.set_frame_buffer(&frame_buffer).unwrap();
        tiled_input_part.read_tile(0, 0, 0, 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_tiledinputpart_read_tile_failure_invalid_level_and_frame_buffer() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let mut tiled_input_part = super::TiledInputPart::new(&input, 0);
        tiled_input_part
            .read_tile(i32::MIN, i32::MIN, i32::MIN, i32::MIN)
            .unwrap();
    }

    #[test]
    fn test_tiledinputpart_read_tiles_success() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let mut tiled_input_part = super::TiledInputPart::new(&input, 0);
        let frame_buffer = FrameBuffer::new();
        tiled_input_part.set_frame_buffer(&frame_buffer).unwrap();
        tiled_input_part.read_tiles(0, 0, 0, 0, 0, 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_tiledinputpart_read_tiles_failure_invalid_level_and_frame_buffer() {
        let input = MultiPartInputFile::new(&*SRC_IMAGE_PATH, 0, true).unwrap();
        let mut tiled_input_part = super::TiledInputPart::new(&input, 0);
        tiled_input_part
            .read_tiles(
                i32::MIN,
                i32::MIN,
                i32::MIN,
                i32::MIN,
                i32::MIN,
                i32::MIN,
            )
            .unwrap();
    }
}
