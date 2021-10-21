use openexr_sys as sys;

use crate::{
    core::{error::Error, header::HeaderRef, LevelMode, LevelRoundingMode},
    deep::deep_frame_buffer::{DeepFrameBuffer, DeepFrameBufferRef},
};

use imath_traits::Bound2;
use std::ffi::CString;
use std::path::Path;

type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(transparent)]
pub struct DeepTiledInputFile(pub(crate) *mut sys::Imf_DeepTiledInputFile_t);

impl DeepTiledInputFile {
    /// Open the file at path `filename` and read the header.
    ///
    /// # Errors
    /// * [`Error::Base`] - if the file cannot be opened
    ///
    pub fn new<P: AsRef<Path>>(
        filename: P,
        num_threads: i32,
    ) -> Result<DeepTiledInputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepTiledInputFile_ctor(
                &mut ptr,
                c_filename.as_ptr(),
                num_threads,
            )
            .into_result()?;
        }

        Ok(DeepTiledInputFile(ptr))
    }

    /// Access to the file [`Header`](crate::core::header::Header)
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepTiledInputFile_header(self.0, &mut ptr);
            if ptr.is_null() {
                panic!(
                    "Received null ptr from sys::Imf_DeepTiledInputFile_header"
                );
            }

            HeaderRef::new(ptr)
        }
    }

    /// Access to the file format version
    ///
    pub fn version(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputFile_version(self.0, &mut v);
        }
        v
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the DeepTiledInputFile object.
    ///
    /// The current frame buffer is the destination for the pixel
    /// data read from the file.  The current frame buffer must be
    /// set at least once before `read_pixels()` is called.
    /// The current frame buffer can be changed after each call
    /// to `read_pixels()`.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if the sampling factors do not match or
    /// if the frame buffer does not have a sample count slice.
    ///
    pub fn set_frame_buffer(
        &mut self,
        frame_buffer: &DeepFrameBuffer,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepTiledInputFile_setFrameBuffer(
                self.0,
                frame_buffer.ptr,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Access to the current frame buffer
    ///
    pub fn frame_buffer(&self) -> DeepFrameBufferRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepTiledInputFile_frameBuffer(self.0, &mut ptr);
        }

        DeepFrameBufferRef::new(ptr)
    }

    /// Check if all pixels in the data window are present in the input file
    ///
    pub fn is_complete(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepTiledInputFile_isComplete(self.0, &mut v);
        }

        v
    }

    /// Get the tiles' x dimension
    ///
    pub fn tile_x_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputFile_tileXSize(self.0, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledInputFile_tileXSize");
        }
        v
    }

    /// Get the tiles' y dimension
    ///
    pub fn tile_y_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputFile_tileYSize(self.0, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledInputFile_tileYSize");
        }
        v
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_DeepTiledInputFile_levelMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledInputFile_levelMode",
            );
        }

        v.into()
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_DeepTiledInputFile_levelRoundingMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledInputFile_levelRoundingMode",
            );
        }

        v.into()
    }

    /// Get the number of levels in the file
    ///
    /// # Returns
    /// * `Ok(1)` if [`level_mode()`](DeepTiledInputFile::level_mode()) == [`LevelMode::OneLevel`]
    /// * `Ok(rfunc (log (max (w, h)) / log (2)) + 1)` if [`level_mode()`](DeepTiledInputFile::level_mode()) == [`LevelMode::MipmapLevels`]

    /// * `Err(Error::Logic)` if [`level_mode()`](DeepTiledInputFile::level_mode()) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledInputFile::level_rounding_mode`) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputFile_numLevels(self.0, &mut v)
                .into_result()?;
        }

        Ok(v)
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](DeepTiledInputFile::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](DeepTiledInputFile::level_mode) == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (w) / log (2)) + 1` if [`level_mode()`](DeepTiledInputFile::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledInputFile::level_rounding_mode) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputFile_numXLevels(self.0, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledInputFile_numXLevels");
        }
        v
    }

    /// Get the number of levels in the file in the y axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](DeepTiledInputFile::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](DeepTiledInputFile::level_mode)`DeepTiledInputFile::mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (h) / log (2)) + 1` if [`level_mode()`](DeepTiledInputFile::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledInputFile::level_rounding_mode) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputFile_numYLevels(self.0, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledInputFile_numYLevels");
        }
        v
    }

    /// Returns `true` if the file contains a level with level number `(lx, ly)`, `false`
    /// otherwise.
    ///
    pub fn is_valid_level(&self, lx: i32, ly: i32) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepTiledInputFile_isValidLevel(self.0, &mut v, lx, ly).into_result().expect("Unexpected exception from Imf_DeepTiledInputFile_isValidLevel");
        }
        v
    }

    /// Returns the width of the level with level number `(lx, *)`, where `*` is any number.
    ///
    /// # Returns
    /// * `max (1, rfunc (w / pow (2, lx)))`
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledInputFile::level_rounding_mode) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputFile_levelWidth(self.0, &mut v, lx)
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
    /// [`level_rounding_mode()`](DeepTiledInputFile::level_rounding_mode) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputFile_levelHeight(self.0, &mut v, ly)
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
            sys::Imf_DeepTiledInputFile_numXTiles(self.0, &mut v, lx)
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
            sys::Imf_DeepTiledInputFile_numYTiles(self.0, &mut v, ly)
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
            sys::Imf_DeepTiledInputFile_dataWindowForLevel(
                self.0,
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
            sys::Imf_DeepTiledInputFile_dataWindowForTile(
                self.0,
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
    /// # Safety
    /// This method is wildly unsafe as on the C++ side it's reading from
    /// pointers offset from the base pointers supplied by the
    /// [`DeepSlice`](crate::deep::deep_frame_buffer::DeepSlice) in
    /// the [`DeepFrameBuffer`]. You must ensure the the [`DeepFrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](DeepScanLineInputFile::set_frame_buffer) has valid slices
    /// for the channels to be read.
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
    pub unsafe fn read_tile(
        &mut self,
        dx: i32,
        dy: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        sys::Imf_DeepTiledInputFile_readTile(self.0, dx, dy, lx, ly)
            .into_result()?;
        Ok(())
    }

    /// Reads the sample counts in tile range with coordinates `(dx1, dy1)`, to `(dx2, dy2)` and level number `(lx, ly)`,
    /// and stores it in the current frame buffer.
    ///
    /// # Safety
    /// This method is wildly unsafe as on the C++ side it's reading from
    /// pointers offset from the base pointers supplied by the
    /// [`DeepSlice`](crate::deep::deep_frame_buffer::DeepSlice) in
    /// the [`DeepFrameBuffer`]. You must ensure the the [`DeepFrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](DeepScanLineInputFile::set_frame_buffer) has valid slices
    /// for the channels to be read.
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
    pub unsafe fn read_tiles(
        &mut self,
        dx1: i32,
        dx2: i32,
        dy1: i32,
        dy2: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        sys::Imf_DeepTiledInputFile_readTiles(
            self.0, dx1, dx2, dy1, dy2, lx, ly,
        )
        .into_result()?;
        Ok(())
    }

    /// Reads the tile with tile coordinates `(dx, dy)`, and level number `(lx, ly)`,
    /// and stores it in the current frame buffer.
    ///
    /// # Safety
    /// This method is wildly unsafe as on the C++ side it's reading from
    /// pointers offset from the base pointers supplied by the
    /// [`DeepSlice`](crate::deep::deep_frame_buffer::DeepSlice) in
    /// the [`DeepFrameBuffer`]. You must ensure the the [`DeepFrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](DeepScanLineInputFile::set_frame_buffer) has valid slices
    /// for the channels to be read.
    ///
    ///   # Errors
    /// * [`Error::InvalidArgument`] - if dx does not lie in the interval [0, num_x_tiles(lx)-1]
    /// * [`Error::InvalidArgument`] - if dy does not lie in the interval [0, num_y_tiles(ly)-1]
    ///
    /// * [`Error::InvalidArgument`] -if lx does not lie in the interval [0, num_x_levels()-1]
    /// * [`Error::InvalidArgument`] -if ly does not lie in the inverval [0, num_y_levels()-1]
    /// * [`Error::Input`] - if the file has an unexpected structure
    /// * [`Error::Base`] - if any other error occurs
    ///
    pub unsafe fn read_pixel_sample_count(
        &mut self,
        dx: i32,
        dy: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        sys::Imf_DeepTiledInputFile_readPixelSampleCount(
            self.0, dx, dy, lx, ly,
        )
        .into_result()?;
        Ok(())
    }

    /// Reads the sample counts in tile range with coordinates `(dx1, dy1)`, to `(dx2, dy2)` and level number `(lx, ly)`,
    /// and stores it in the current frame buffer.
    ///
    /// # Safety
    /// This method is wildly unsafe as on the C++ side it's reading from
    /// pointers offset from the base pointers supplied by the
    /// [`DeepSlice`](crate::deep::deep_frame_buffer::DeepSlice) in
    /// the [`DeepFrameBuffer`]. You must ensure the the [`DeepFrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](DeepScanLineInputFile::set_frame_buffer) has valid slices
    /// for the channels to be read.
    ///
    ///   # Errors
    /// * [`Error::InvalidArgument`] - if dx does not lie in the interval [0, num_x_tiles(lx)-1]
    /// * [`Error::InvalidArgument`] - if dy does not lie in the interval [0, num_y_tiles(ly)-1]
    ///
    /// * [`Error::InvalidArgument`] -if lx does not lie in the interval [0, num_x_levels()-1]
    /// * [`Error::InvalidArgument`] -if ly does not lie in the inverval [0, num_y_levels()-1]
    /// * [`Error::Input`] - if the file has an unexpected structure
    /// * [`Error::Base`] - if any other error occurs
    ///
    pub unsafe fn read_pixel_sample_counts(
        &mut self,
        dx1: i32,
        dx2: i32,
        dy1: i32,
        dy2: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        sys::Imf_DeepTiledInputFile_readPixelSampleCounts(
            self.0, dx1, dx2, dy1, dy2, lx, ly,
        )
        .into_result()?;
        Ok(())
    }
}

impl Drop for DeepTiledInputFile {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_DeepTiledInputFile_dtor(self.0);
        }
    }
}
