use openexr_sys as sys;

use crate::{
    core::{
        error::Error, header::HeaderRef, preview_image::PreviewRgba, LevelMode,
        LevelRoundingMode,
    },
    deep::{
        deep_frame_buffer::{DeepFrameBuffer, DeepFrameBufferRef},
        deep_tiled_input_file::DeepTiledInputFile,
        deep_tiled_input_part::DeepTiledInputPart,
    },
    multi_part::multi_part_output_file::MultiPartOutputFile,
};

use imath_traits::Bound2;

type Result<T, E = Error> = std::result::Result<T, E>;

use std::marker::PhantomData;

#[repr(transparent)]
pub struct DeepTiledOutputPart<'a> {
    pub(crate) inner: sys::Imf_DeepTiledOutputPart_t,
    phantom: std::marker::PhantomData<&'a MultiPartOutputFile>,
}

impl<'a> DeepTiledOutputPart<'a> {
    /// Get an interface to the part `part_number` of the [`MultiPartOutputFile`]
    /// `multi_part_file`.
    ///
    pub fn new(
        multi_part_file: &MultiPartOutputFile,
        part_number: i32,
    ) -> Result<DeepTiledOutputPart> {
        let mut part = sys::Imf_DeepTiledOutputPart_t::default();
        unsafe {
            sys::Imf_DeepTiledOutputPart_ctor(
                &mut part,
                multi_part_file.0,
                part_number,
            )
            .into_result()?;
        }

        Ok(DeepTiledOutputPart {
            inner: part,
            phantom: PhantomData,
        })
    }

    /// Define a frame buffer as the pixel data source.
    ///
    /// Pixel (x, y) is at offset x * x_stride + y * y_stride
    ///
    /// ## Errors
    /// * [`Error::Base`] - If an error occurs
    ///
    pub fn set_frame_buffer(
        &mut self,
        frame_buffer: &DeepFrameBuffer,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepTiledOutputPart_setFrameBuffer(
                &mut self.inner,
                frame_buffer.ptr,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Access to the file [`Header`](crate::core::header::Header)
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepTiledOutputPart_header(&self.inner, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_DeepTiledOutputPart_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Get the [`DeepFrameBuffer`](crate::deep::deep_frame_buffer::DeepFrameBuffer)
    ///
    pub fn frame_buffer(&self) -> DeepFrameBufferRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepTiledOutputPart_frameBuffer(&self.inner, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_DeepTiledOutputPart_frameBuffer");
            }

            DeepFrameBufferRef::new(ptr)
        }
    }

    /// Get the tiles' x dimension
    ///
    pub fn tile_x_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputPart_tileXSize(&self.inner, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledOutputPart_tileXSize");
        }
        v
    }

    /// Get the tiles' y dimension
    ///
    pub fn tile_y_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputPart_tileYSize(&self.inner, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledOutputPart_tileYSize");
        }
        v
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_DeepTiledOutputPart_levelMode(
                &self.inner,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledOutputPart_levelMode",
            );
        }

        v.into()
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_DeepTiledOutputPart_levelRoundingMode(
                &self.inner,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledOutputPart_levelRoundingMode",
            );
        }

        v.into()
    }

    /// Get the number of levels in the file
    ///
    /// # Returns
    /// * `Ok(1)` if [`level_mode()`](DeepTiledOutputPart::level_mode()) == [`LevelMode::OneLevel`]
    /// * `Ok(rfunc (log (max (w, h)) / log (2)) + 1)` if [`level_mode()`](DeepTiledOutputPart::level_mode()) == [`LevelMode::MipmapLevels`]

    /// * `Err(Error::Logic)` if [`level_mode()`](DeepTiledOutputPart::level_mode()) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledOutputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`](crate::core::LevelRoundingMode::RoundUp) or [`LevelRoundingMode::RoundDown`](crate::core::LevelRoundingMode::RoundDown)
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputPart_numLevels(&self.inner, &mut v)
                .into_result()?;
        }

        Ok(v)
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](DeepTiledOutputPart::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](DeepTiledOutputPart::level_mode) == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (w) / log (2)) + 1` if [`level_mode()`](DeepTiledOutputPart::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledOutputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputPart_numXLevels(&self.inner, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledOutputPart_numXLevels");
        }
        v
    }

    /// Get the number of levels in the file in the y axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](DeepTiledOutputPart::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](DeepTiledOutputPart::level_mode)`DeepTiledOutputPart::mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (h) / log (2)) + 1` if [`level_mode()`](DeepTiledOutputPart::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledOutputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputPart_numYLevels(&self.inner, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledOutputPart_numYLevels");
        }
        v
    }

    /// Returns `true` if the file contains a level with level number `(lx, ly)`, `false`
    /// otherwise.
    ///
    pub fn is_valid_level(&self, lx: i32, ly: i32) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepTiledOutputPart_isValidLevel(&self.inner, &mut v, lx, ly).into_result().expect("Unexpected exception from Imf_DeepTiledOutputPart_isValidLevel");
        }
        v
    }

    /// Returns the width of the level with level number `(lx, *)`, where `*` is any number.
    ///
    /// # Returns
    /// * `max (1, rfunc (w / pow (2, lx)))`
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledOutputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputPart_levelWidth(&self.inner, &mut v, lx)
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
    /// [`level_rounding_mode()`](DeepTiledOutputPart::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputPart_levelHeight(&self.inner, &mut v, ly)
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
            sys::Imf_DeepTiledOutputPart_numXTiles(&self.inner, &mut v, lx)
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
            sys::Imf_DeepTiledOutputPart_numYTiles(&self.inner, &mut v, ly)
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
            sys::Imf_DeepTiledOutputPart_dataWindowForLevel(
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
            sys::Imf_DeepTiledOutputPart_dataWindowForTile(
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

    /// Writes the tile with tile
    /// coordinates (dx, dy), and level number (lx, ly) to
    /// the file.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] if dx does not lie in the interval [0, numXTiles(lx) - 1]
    /// * [`Error::InvalidArgument`] if dy does not lie in the interval [0, numYTiles(ly) - 1]
    ///
    /// * [`Error::InvalidArgument`] if lx does not lie in the interval [0, numXLevels() - 1]
    /// * [`Error::InvalidArgument`] if ly does not lie in the inverval [0, numYLevels() - 1]
    /// * [`Error::Base`] if any other error occurs
    ///
    ///
    /// Pixels that are outside the pixel coordinate range for the tile's
    /// level, are never accessed by writeTile().
    ///
    /// Each tile in the file must be written exactly once.
    ///
    /// The file's line order attribute determines the order of the tiles
    /// in the file:
    ///
    /// ## [`LineOrder::IncreasingY`](crate::core::LineOrder)
    /// In the file, the tiles for each level are stored
    /// in a contiguous block.  The levels are ordered
    /// like this:
    /// ```c
    /// (0, 0)   (1, 0)   ... (nx-1, 0)
    /// (0, 1)   (1, 1)   ... (nx-1, 1)
    ///  ...
    /// (0,ny-1) (1,ny-1) ... (nx-1,ny-1)
    /// ```
    ///
    /// where nx = [`num_x_levels()`](DeepTiledOutputPart::num_x_levels), and ny = [`num_y_levels()`](DeepTiledOutputPart::num_y_levels).
    ///
    /// In an individual level, `(lx, ly)`, the tiles
    /// are stored in the following order:
    ///
    /// ```c
    /// (0, 0)   (1, 0)   ... (tx-1, 0)
    /// (0, 1)   (1, 1)   ... (tx-1, 1)
    ///  ...
    /// (0,ty-1) (1,ty-1) ... (tx-1,ty-1)
    /// ```
    ///
    /// where tx = [`num_x_tiles(lx)`](DeepTiledOutputPart::num_x_tiles),
    /// and   ty = [`num_y_tiles(ly)`](DeepTiledOutputPart::num_y_tiles).
    ///
    /// ## [`LineOrder::DecreasingY`](crate::core::LineOrder)
    /// As for [`LineOrder::IncreasingY`](crate::core::LineOrder), the tiles
    /// for each level are stored in a contiguous block.  The levels
    /// are ordered the same way as for [`LineOrder::IncreasingY`](crate::core::LineOrder),
    /// but within an individual level, the tiles
    /// are stored in this order:
    ///
    /// ```c
    /// (0,ty-1) (1,ty-1) ... (tx-1,ty-1)
    ///  ...
    /// (0, 1)   (1, 1)   ... (tx-1, 1)
    /// (0, 0)   (1, 0)   ... (tx-1, 0)
    /// ```
    ///
    ///
    /// ## [`LineOrder::RandomY`](crate::core::LineOrder)
    /// The order of the calls to `write_tile()` determines
    /// the order of the tiles in the file.
    ///
    /// # Safety
    /// This method is wildly unsafe as on the C++ side it's reading from
    /// pointers offset from the base pointers supplied by the
    /// [`DeepSlice`](crate::deep::deep_frame_buffer::DeepSlice) in
    /// the [`DeepFrameBuffer`]. You must ensure the the [`DeepFrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](DeepScanLineInputFile::set_frame_buffer) has valid slices
    /// for the channels to be written.
    ///
    pub unsafe fn write_tile(
        &mut self,
        dx: i32,
        dy: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        sys::Imf_DeepTiledOutputPart_writeTile(&mut self.inner, dx, dy, lx, ly)
            .into_result()?;
        Ok(())
    }

    /// Writes multiple tiles at once.  
    ///
    /// If multi-threading is used multiple tiles are written concurrently.  
    /// The tile coordinates, dx1, dx2 and dy1, dy2, specify inclusive ranges of tile
    /// coordinates.  It is valid for dx1 < dx2 or dy1 < dy2; the
    /// tiles are always written in the order specified by the line
    /// order attribute.  Hence, it is not possible to specify an
    /// "invalid" or empty tile range.
    ///
    /// Pixels that are outside the pixel coordinate range for the tile's
    /// level, are never accessed by writeTile().
    ///
    /// Each tile in the file must be written exactly once.
    ///
    /// The file's line order attribute determines the order of the tiles
    /// in the file:
    ///
    /// ## [`LineOrder::IncreasingY`](crate::core::LineOrder)
    /// In the file, the tiles for each level are stored
    /// in a contiguous block.  The levels are ordered
    /// like this:
    /// ```c
    /// (0, 0)   (1, 0)   ... (nx-1, 0)
    /// (0, 1)   (1, 1)   ... (nx-1, 1)
    ///  ...
    /// (0,ny-1) (1,ny-1) ... (nx-1,ny-1)
    /// ```
    ///
    /// where nx = [`num_x_levels()`](DeepTiledOutputPart::num_x_levels), and ny = [`num_y_levels()`](DeepTiledOutputPart::num_y_levels).
    ///
    /// In an individual level, `(lx, ly)`, the tiles
    /// are stored in the following order:
    ///
    /// ```c
    /// (0, 0)   (1, 0)   ... (tx-1, 0)
    /// (0, 1)   (1, 1)   ... (tx-1, 1)
    ///  ...
    /// (0,ty-1) (1,ty-1) ... (tx-1,ty-1)
    /// ```
    ///
    /// where tx = [`num_x_tiles(lx)`](DeepTiledOutputPart::num_x_tiles),
    /// and   ty = [`num_y_tiles(ly)`](DeepTiledOutputPart::num_y_tiles).
    ///
    /// ## [`LineOrder::DecreasingY`](crate::core::LineOrder)
    /// As for [`LineOrder::IncreasingY`](crate::core::LineOrder), the tiles
    /// for each level are stored in a contiguous block.  The levels
    /// are ordered the same way as for [`LineOrder::IncreasingY`](crate::core::LineOrder),
    /// but within an individual level, the tiles
    /// are stored in this order:
    ///
    /// ```c
    /// (0,ty-1) (1,ty-1) ... (tx-1,ty-1)
    ///  ...
    /// (0, 1)   (1, 1)   ... (tx-1, 1)
    /// (0, 0)   (1, 0)   ... (tx-1, 0)
    /// ```
    ///
    ///
    /// ## [`LineOrder::RandomY`](crate::core::LineOrder)
    /// The order of the calls to `write_tile()` determines
    /// the order of the tiles in the file.
    ///
    /// # Safety
    /// This method is wildly unsafe as on the C++ side it's reading from
    /// pointers offset from the base pointers supplied by the
    /// [`DeepSlice`](crate::deep::deep_frame_buffer::DeepSlice) in
    /// the [`DeepFrameBuffer`]. You must ensure the the [`DeepFrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](DeepScanLineInputFile::set_frame_buffer) has valid slices
    /// for the channels to be written.
    ///
    pub unsafe fn write_tiles(
        &mut self,
        dx1: i32,
        dx2: i32,
        dy1: i32,
        dy2: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        sys::Imf_DeepTiledOutputPart_writeTiles(
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

    /// Supplies a new set of pixels for the preview image attribute in the file's header
    ///
    /// Note: update_preview_image() is necessary because images are
    /// often stored in a file incrementally, a few tiles at a time,
    /// while the image is being generated.  Since the preview image
    /// is an attribute in the file's header, it gets stored in the
    /// file as soon as the file is opened, but we may not know what
    /// the preview image should look like until we have written the
    /// last tile of the main image.
    ///
    /// # Errors
    /// * [`Error::LogicError`] - If the image header does not contain a preview image
    /// * [`Error::Base`] - If any other error occurs
    ///
    pub fn update_preview_image(
        &mut self,
        new_pixels: &[PreviewRgba],
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepTiledOutputPart_updatePreviewImage(
                &mut self.inner,
                new_pixels.as_ptr() as *const sys::Imf_PreviewRgba_t,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Shortcut to copy all pixels from an [`DeepTiledInputFile`] into this file,
    /// without uncompressing and then recompressing the pixel data.
    ///
    /// This file's header must be compatible with the [`DeepTiledInputFile`]'s
    /// header:  The two header's "dataWindow", "compression",
    /// "lineOrder" and "channels" attributes must be the same.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If the headers do not match
    /// * [`Error::LogicError`] - If tiles have already been written to this file.
    /// * [`Error::Base`] - If any other error occurs
    ///
    pub fn copy_pixels_from_file(
        &mut self,
        file: &DeepTiledInputFile,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepTiledOutputPart_copyPixels_from_file(
                &mut self.inner,
                file.0,
            )
            .into_result()?;
        }
        Ok(())
    }

    /// Shortcut to copy all pixels from an [`DeepTiledInputPart`] into this file,
    /// without uncompressing and then recompressing the pixel data.
    ///
    /// This file's header must be compatible with the [`DeepTiledInputPart`]'s
    /// header:  The two header's "dataWindow", "compression",
    /// "lineOrder" and "channels" attributes must be the same.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If the headers do not match
    /// * [`Error::LogicError`] - If tiles have already been written to this file.
    /// * [`Error::Base`] - If any other error occurs
    ///
    pub fn copy_pixels_from_part(
        &mut self,
        file: &mut DeepTiledInputPart,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepTiledOutputPart_copyPixels_from_part(
                &mut self.inner,
                &mut file.inner,
            )
            .into_result()?;
        }
        Ok(())
    }
}
