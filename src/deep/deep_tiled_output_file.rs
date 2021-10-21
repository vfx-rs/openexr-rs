use openexr_sys as sys;

use std::ffi::CString;
use std::path::Path;

use crate::{
    core::{
        error::Error,
        header::{Header, HeaderRef},
        preview_image::PreviewRgba,
        LevelMode, LevelRoundingMode,
    },
    deep::{
        deep_frame_buffer::{DeepFrameBuffer, DeepFrameBufferRef},
        deep_tiled_input_file::DeepTiledInputFile,
        deep_tiled_input_part::DeepTiledInputPart,
    },
};

use imath_traits::Bound2;

type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(transparent)]
pub struct DeepTiledOutputFile(pub(crate) *mut sys::Imf_DeepTiledOutputFile_t);

impl DeepTiledOutputFile {
    /// Opens the file and writes the header.
    ///
    /// # Errors
    /// * [`Error::Base`] - If the file could not be opened or the [`Header`]
    /// is invalid
    ///
    pub fn new<P: AsRef<Path>>(
        filename: P,
        header: &Header,
        num_threads: i32,
    ) -> Result<DeepTiledOutputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut _inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepTiledOutputFile_ctor(
                &mut _inner,
                c_filename.as_ptr(),
                header.0.as_ref(),
                num_threads,
            )
            .into_result()?;
        }

        Ok(DeepTiledOutputFile(_inner))
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
            sys::Imf_DeepTiledOutputFile_setFrameBuffer(
                self.0,
                frame_buffer.ptr,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Access to the file [`Header`]
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepTiledOutputFile_header(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_DeepTiledOutputFile_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Get the [`DeepFrameBuffer`](crate::deep::deep_frame_buffer::DeepFrameBuffer)
    ///
    pub fn frame_buffer(&self) -> DeepFrameBufferRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepTiledOutputFile_frameBuffer(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_DeepTiledOutputFile_frameBuffer");
            }

            DeepFrameBufferRef::new(ptr)
        }
    }

    /// Get the tiles' x dimension
    ///
    pub fn tile_x_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputFile_tileXSize(self.0, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledOutputFile_tileXSize");
        }
        v
    }

    /// Get the tiles' y dimension
    ///
    pub fn tile_y_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputFile_tileYSize(self.0, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledOutputFile_tileYSize");
        }
        v
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_DeepTiledOutputFile_levelMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledOutputFile_levelMode",
            );
        }

        v.into()
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_DeepTiledOutputFile_levelRoundingMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_DeepTiledOutputFile_levelRoundingMode",
            );
        }

        v.into()
    }

    /// Get the number of levels in the file
    ///
    /// # Returns
    /// * `Ok(1)` if [`level_mode()`](DeepTiledOutputFile::level_mode()) == [`LevelMode::OneLevel`]
    /// * `Ok(rfunc (log (max (w, h)) / log (2)) + 1)` if [`level_mode()`](DeepTiledOutputFile::level_mode()) == [`LevelMode::MipmapLevels`]

    /// * `Err(Error::Logic)` if [`level_mode()`](DeepTiledOutputFile::level_mode()) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputFile_numLevels(self.0, &mut v)
                .into_result()?;
        }

        Ok(v)
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](DeepTiledOutputFile::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](DeepTiledOutputFile::level_mode) == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (w) / log (2)) + 1` if [`level_mode()`](DeepTiledOutputFile::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputFile_numXLevels(self.0, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledOutputFile_numXLevels");
        }
        v
    }

    /// Get the number of levels in the file in the y axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](DeepTiledOutputFile::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](DeepTiledOutputFile::level_mode)`DeepTiledOutputFile::mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (h) / log (2)) + 1` if [`level_mode()`](DeepTiledOutputFile::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputFile_numYLevels(self.0, &mut v).into_result().expect("Unexpected exception from Imf_DeepTiledOutputFile_numYLevels");
        }
        v
    }

    /// Returns `true` if the file contains a level with level number `(lx, ly)`, `false`
    /// otherwise.
    ///
    pub fn is_valid_level(&self, lx: i32, ly: i32) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepTiledOutputFile_isValidLevel(self.0, &mut v, lx, ly).into_result().expect("Unexpected exception from Imf_DeepTiledOutputFile_isValidLevel");
        }
        v
    }

    /// Returns the width of the level with level number `(lx, *)`, where `*` is any number.
    ///
    /// # Returns
    /// * `max (1, rfunc (w / pow (2, lx)))`
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](DeepTiledOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputFile_levelWidth(self.0, &mut v, lx)
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
    /// [`level_rounding_mode()`](DeepTiledOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepTiledOutputFile_levelHeight(self.0, &mut v, ly)
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
            sys::Imf_DeepTiledOutputFile_numXTiles(self.0, &mut v, lx)
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
            sys::Imf_DeepTiledOutputFile_numYTiles(self.0, &mut v, ly)
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
            sys::Imf_DeepTiledOutputFile_dataWindowForLevel(
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
            sys::Imf_DeepTiledOutputFile_dataWindowForTile(
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
    /// where nx = [`num_x_levels()`](DeepTiledOutputFile::num_x_levels), and ny = [`num_y_levels()`](DeepTiledOutputFile::num_y_levels).
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
    /// where tx = [`num_x_tiles(lx)`](DeepTiledOutputFile::num_x_tiles),
    /// and   ty = [`num_y_tiles(ly)`](DeepTiledOutputFile::num_y_tiles).
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
        sys::Imf_DeepTiledOutputFile_writeTile(self.0, dx, dy, lx, ly)
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
    /// where nx = [`num_x_levels()`](DeepTiledOutputFile::num_x_levels), and ny = [`num_y_levels()`](DeepTiledOutputFile::num_y_levels).
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
    /// where tx = [`num_x_tiles(lx)`](DeepTiledOutputFile::num_x_tiles),
    /// and   ty = [`num_y_tiles(ly)`](DeepTiledOutputFile::num_y_tiles).
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
        sys::Imf_DeepTiledOutputFile_writeTiles(
            self.0, dx1, dx2, dy1, dy2, lx, ly,
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
            sys::Imf_DeepTiledOutputFile_updatePreviewImage(
                self.0,
                new_pixels.as_ptr() as *const sys::Imf_PreviewRgba_t,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Shortcut to copy all pixels from a [`DeepTiledInputFile`] into this file,
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
            sys::Imf_DeepTiledOutputFile_copyPixels_from_file(self.0, file.0)
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
            sys::Imf_DeepTiledOutputFile_copyPixels_from_part(
                self.0,
                &mut file.inner,
            )
            .into_result()?;
        }
        Ok(())
    }
}

impl Drop for DeepTiledOutputFile {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_DeepTiledOutputFile_dtor(self.0)
                .into_result()
                .expect("Unexpected exception in Imf_DeepTiledOutputFile_dtor");
        }
    }
}

#[cfg(test)]
#[test]
fn test_write_deep_tiled1() -> Result<()> {
    use crate::{
        core::{
            channel_list::{CHANNEL_FLOAT, CHANNEL_HALF},
            frame_buffer::Frame,
            header::ImageType,
            tile_description::TileDescription,
            Compression, LineOrder,
        },
        deep::deep_frame_buffer::{DeepFrameBuffer, DeepSlice},
    };
    use half::f16;

    let width = 256;
    let height = 256;
    let num_pixels = (width * height) as usize;

    let data_window = [0, 0, width - 1, height - 1];

    let mut header = Header::with_dimensions(
        width,
        height,
        1.0,
        [0.0f32, 0.0],
        1.0,
        LineOrder::IncreasingY,
        Compression::No,
    )?;

    header.set_image_type(ImageType::DeepTiled);
    header.set_tile_description(&TileDescription::new(
        64,
        64,
        LevelMode::OneLevel,
        LevelRoundingMode::RoundUp,
    ));
    header.channels_mut().insert("Z", &CHANNEL_FLOAT);
    header.channels_mut().insert("A", &CHANNEL_HALF);

    let mut frame_buffer = DeepFrameBuffer::new();
    let sample_counts = vec![1u32; num_pixels];
    let sample_count_frame =
        Frame::with_vec(&["sampleCounts"], sample_counts, data_window)?;
    frame_buffer.set_sample_count_frame(sample_count_frame)?;

    let (mut z_pixels, mut a_pixels): (Vec<Vec<f32>>, Vec<Vec<f16>>) =
        crate::tests::deep_testpattern(width, height);

    let mut z_ptrs: Vec<*mut f32> =
        z_pixels.iter_mut().map(|v| v.as_mut_ptr()).collect();
    let mut a_ptrs: Vec<*mut f16> =
        a_pixels.iter_mut().map(|v| v.as_mut_ptr()).collect();

    // set slices
    frame_buffer.insert(
        "Z",
        &DeepSlice::from_sample_ptr(z_ptrs.as_mut_ptr(), width).build()?,
    )?;

    frame_buffer.insert(
        "A",
        &DeepSlice::from_sample_ptr(a_ptrs.as_mut_ptr(), width).build()?,
    )?;

    let mut file =
        DeepTiledOutputFile::new("write_deep_tiled1.exr", &header, 4)?;
    file.set_frame_buffer(&frame_buffer)?;
    unsafe {
        file.write_tiles(
            0,
            file.num_x_tiles(0)? - 1,
            0,
            file.num_y_tiles(0)? - 1,
            0,
            0,
        )?;
    }

    drop(file);

    Ok(())
}
