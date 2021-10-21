use openexr_sys as sys;

use crate::{
    core::{error::Error, header::HeaderRef, LevelMode, LevelRoundingMode},
    deep::deep_frame_buffer::{DeepFrameBuffer, DeepFrameBufferRef},
    multi_part::multi_part_input_file::MultiPartInputFile,
};

use imath_traits::Bound2;

type Result<T, E = Error> = std::result::Result<T, E>;

use std::marker::PhantomData;

#[repr(transparent)]
pub struct DeepTiledInputPart<'a> {
    pub(crate) inner: sys::Imf_DeepTiledInputPart_t,
    phantom: std::marker::PhantomData<&'a MultiPartInputFile>,
}

impl<'a> DeepTiledInputPart<'a> {
    /// Get an interface to the part `part_number` of the [`MultiPartInputFile`]
    /// `multi_part_file`.
    ///
    pub fn new(
        multi_part_file: &MultiPartInputFile,
        part_number: i32,
    ) -> Result<DeepTiledInputPart> {
        let mut part = sys::Imf_DeepTiledInputPart_t::default();
        unsafe {
            sys::Imf_DeepTiledInputPart_ctor(
                &mut part,
                multi_part_file.0,
                part_number,
            )
            .into_result()?;
        }

        Ok(DeepTiledInputPart {
            inner: part,
            phantom: PhantomData,
        })
    }

    /// Access to the file [`Header`](crate::core::header::Header)
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepTiledInputPart_header(&self.inner, &mut ptr);
            if ptr.is_null() {
                panic!(
                    "Received null ptr from sys::Imf_DeepTiledInputPart_header"
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
            sys::Imf_DeepTiledInputPart_version(&self.inner, &mut v);
        }
        v
    }

    /// Set the current frame buffer -- copies the [`DeepFrameBuffer`]
    /// object into the DeepTiledInputPart object.
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
            sys::Imf_DeepTiledInputPart_setFrameBuffer(
                &mut self.inner,
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
            sys::Imf_DeepTiledInputPart_frameBuffer(&self.inner, &mut ptr);
        }

        DeepFrameBufferRef::new(ptr)
    }

    /// Check if all pixels in the data window are present in the input file
    ///
    pub fn is_complete(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepTiledInputPart_isComplete(&self.inner, &mut v);
        }

        v
    }

    /// Get the tiles' x dimension
    ///
    pub fn tile_x_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputPart_tileXSize(& self.inner, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledInputPart_tileXSize");
        }
        v
    }

    /// Get the tiles' y dimension
    ///
    pub fn tile_y_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputPart_tileYSize(& self.inner, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledInputPart_tileYSize");
        }
        v
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_DeepTiledInputPart_levelMode(
                & self.inner,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledInputPart_levelMode",
            );
        }

        v.into()
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_DeepTiledInputPart_levelRoundingMode(
                & self.inner,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledInputPart_levelRoundingMode",
            );
        }

        v.into()
    }

    /// Get the number of levels in the file
    ///
    /// # Returns
    /// * `Ok(1)` if [`level_mode()`](DeepTiledInputPart::level_mode()) == [`LevelMode::OneLevel`]
    /// * `Ok(rfunc (log (max (w, h)) / log (2)) + 1)` if [`level_mode()`](DeepTiledInputPart::level_mode()) == [`LevelMode::MipmapLevels`]

    /// * `Err(Error::Logic)` if [`level_mode()`](DeepTiledInputPart::level_mode()) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledInputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputPart_numLevels(&self.inner, &mut v)
                .into_result()?;
        }

        Ok(v)
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](DeepTiledInputPart::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](DeepTiledInputPart::level_mode) == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (w) / log (2)) + 1` if [`level_mode()`](DeepTiledInputPart::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledInputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputPart_numXLevels(& self.inner, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledInputPart_numXLevels");
        }
        v
    }

    /// Get the number of levels in the file in the y axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](DeepTiledInputPart::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](DeepTiledInputPart::level_mode)`DeepTiledInputPart::mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (h) / log (2)) + 1` if [`level_mode()`](DeepTiledInputPart::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledInputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputPart_numYLevels(& self.inner, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledInputPart_numYLevels");
        }
        v
    }

    /// Returns `true` if the file contains a level with level number `(lx, ly)`, `false`
    /// otherwise.
    ///
    pub fn is_valid_level(&self, lx: i32, ly: i32) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepTiledInputPart_isValidLevel(
                &self.inner,
                &mut v,
                lx,
                ly,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledInputPart_isValidLevel",
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
    /// [`level_rounding_mode()`](DeepTiledInputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputPart_levelWidth(&self.inner, &mut v, lx)
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
    /// [`level_rounding_mode()`](DeepTiledInputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledInputPart_levelHeight(&self.inner, &mut v, ly)
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
            sys::Imf_DeepTiledInputPart_numXTiles(&self.inner, &mut v, lx)
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
            sys::Imf_DeepTiledInputPart_numYTiles(&self.inner, &mut v, ly)
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
            sys::Imf_DeepTiledInputPart_dataWindowForLevel(
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
            sys::Imf_DeepTiledInputPart_dataWindowForTile(
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
        sys::Imf_DeepTiledInputPart_readTile(&mut self.inner, dx, dy, lx, ly)
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
        sys::Imf_DeepTiledInputPart_readTiles(
            &mut self.inner,
            dx1,
            dx2,
            dy1,
            dy2,
            lx,
            ly,
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
        sys::Imf_DeepTiledInputPart_readPixelSampleCount(
            &mut self.inner,
            dx,
            dy,
            lx,
            ly,
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
        sys::Imf_DeepTiledInputPart_readPixelSampleCounts(
            &mut self.inner,
            dx1,
            dx2,
            dy1,
            dy2,
            lx,
            ly,
        )
        .into_result()?;
        Ok(())
    }
}
