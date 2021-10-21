use crate::{
    core::{
        cppstd::CppString,
        error::Error,
        frame_buffer::FrameBufferRef,
        header::{Header, HeaderRef},
        preview_image::PreviewRgba,
        Compression, LevelMode, LevelRoundingMode, LineOrder,
    },
    rgba::rgba::{Rgba, RgbaChannels},
};

use imath_traits::{Bound2, Vec2};
use openexr_sys as sys;
use std::ffi::CString;
use std::path::Path;

type Result<T, E = Error> = std::result::Result<T, E>;

/// A simplified interface for writing a tiled RGBA EXR file
/// ```no_run
/// use openexr::prelude::*;
///
/// # use imath_traits::Zero;
/// # let pixels = vec![Rgba::zero(); 64*64];
/// # let width = 64;
/// # let height = 64;
/// let header = Header::from_dimensions(width, height);
/// let mut file = TiledRgbaOutputFile::new(
///     "write_tiled_rgba1.exr",
///     &header,
///     RgbaChannels::WriteRgba,
///     64,
///     64,
///     LevelMode::OneLevel,
///     LevelRoundingMode::RoundDown,
///     1,
/// )
/// .unwrap();
/// file.set_frame_buffer(&pixels, 1, width as usize).unwrap();
/// unsafe {
///    file.write_tiles(
///        0,
///        file.num_x_tiles(0).unwrap() - 1,
///        0,
///        file.num_y_tiles(0).unwrap() - 1,
///        0,
///        0,
///    )
///    .unwrap();
/// }
/// ```
///
#[repr(transparent)]
pub struct TiledRgbaOutputFile(pub(crate) *mut sys::Imf_TiledRgbaOutputFile_t);

impl TiledRgbaOutputFile {
    /// Create a new [`TiledRgbaOutputFile`] with the given header and parameters.
    ///
    /// # Arguments
    /// * `filename` - The path to which the resulting file will be written
    /// * `header` - Reference to a [`Header`]
    /// * `channels` - Which channels the pixel data will contain (overwrites the value in
    /// `header`)
    /// * `tile_x_size` - The size of the tiles in the x axis (overwrites the value in `header`)
    /// * `tile_y_size` - The size of the tiles in the y axis (overwrites the value in `header`)
    /// * `mode` - The level mode (overwrites the value in `header`)
    /// * `rounding_mode` - The level rounding mode (overwrites the value in `header`)
    /// * `num_threads` - The number of threads to use to write the image
    ///
    /// ## Errors
    /// * [`Error::Base`] - If an error occurs
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new<P: AsRef<Path>>(
        filename: P,
        header: &Header,
        channels: RgbaChannels,
        tile_x_size: i32,
        tile_y_size: i32,
        mode: LevelMode,
        rounding_mode: LevelRoundingMode,
        num_threads: i32,
    ) -> Result<TiledRgbaOutputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut _inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_TiledRgbaOutputFile_ctor(
                &mut _inner,
                c_filename.as_ptr(),
                header.0.as_ref(),
                channels.into(),
                tile_x_size,
                tile_y_size,
                mode.into(),
                rounding_mode.into(),
                num_threads,
            )
            .into_result()?;
        }

        Ok(TiledRgbaOutputFile(_inner))
    }

    /// Create a new [`TiledRgbaOutputFile`] with the given parameters.
    ///
    /// Display window and data window will both be set to [0, width) and
    /// [0, height).
    ///
    /// # Arguments
    /// * `filename` - The path to which the resulting file will be written
    /// * `width` - The width of the image.
    /// * `height` - The height of the image.
    /// * `channels` - Which channels the pixel data will contain.
    /// * `pixel_aspect_ratio` - The width/height ratio of each pixel
    /// * `screen_window_center` - Center of the screen window.
    /// * `screen_window_width` - Width of the screen window.
    /// * `line_order` - Whether the scanlines are stored top-to-bottom.
    /// * `compression` - Which compression scheme to use
    /// * `num_threads` - The number of threads to use to write the image
    ///
    /// ## Errors
    /// * [`Error::Base`] - If an error occurs
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn with_dimensions<P: AsRef<Path>, V>(
        filename: P,
        width: i32,
        height: i32,
        channels: RgbaChannels,
        tile_x_size: i32,
        tile_y_size: i32,
        mode: LevelMode,
        rounding_mode: LevelRoundingMode,
        pixel_aspect_ratio: f32,
        screen_window_center: V,
        screen_window_width: f32,
        line_order: LineOrder,
        compression: Compression,
        num_threads: i32,
    ) -> Result<TiledRgbaOutputFile>
    where
        V: Vec2<f32>,
    {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let s = sys::Imath_V2f_t {
            x: screen_window_center.as_slice()[0],
            y: screen_window_center.as_slice()[1],
        };

        let mut _inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_TiledRgbaOutputFile_with_dimensions(
                &mut _inner,
                c_filename.as_ptr(),
                width,
                height,
                tile_x_size,
                tile_y_size,
                mode.into(),
                rounding_mode.into(),
                channels.into(),
                pixel_aspect_ratio,
                s,
                screen_window_width,
                line_order.into(),
                compression.into(),
                num_threads,
            )
            .into_result()?;
        }

        Ok(TiledRgbaOutputFile(_inner))
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
        data: &[Rgba],
        x_stride: usize,
        y_stride: usize,
    ) -> Result<()> {
        unsafe {
            sys::Imf_TiledRgbaOutputFile_setFrameBuffer(
                self.0,
                data.as_ptr() as *const sys::Imf_Rgba_t,
                x_stride,
                y_stride,
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
            sys::Imf_TiledRgbaOutputFile_header(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_TiledRgbaOutputFile_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Get the [`FrameBuffer`](crate::core::frame_buffer::FrameBuffer)
    ///
    pub fn frame_buffer(&self) -> FrameBufferRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_TiledRgbaOutputFile_frameBuffer(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_TiledRgbaOutputFile_frameBuffer");
            }

            FrameBufferRef::new(ptr)
        }
    }

    /// Get the tiles' x dimension
    ///
    pub fn tile_x_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaOutputFile_tileXSize(self.0, &mut v).into_result().expect("Unexpected exception from Imf_TiledRgbaOutputFile_tileXSize");
        }
        v
    }

    /// Get the tiles' y dimension
    ///
    pub fn tile_y_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaOutputFile_tileYSize(self.0, &mut v).into_result().expect("Unexpected exception from Imf_TiledRgbaOutputFile_tileYSize");
        }
        v
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_TiledRgbaOutputFile_levelMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_TiledRgbaOutputFile_levelMode",
            );
        }

        v.into()
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_TiledRgbaOutputFile_levelRoundingMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_TiledRgbaOutputFile_levelRoundingMode",
            );
        }

        v.into()
    }

    /// Get the number of levels in the file
    ///
    /// # Returns
    /// * `Ok(1)` if [`level_mode()`](TiledRgbaOutputFile::level_mode()) == [`LevelMode::OneLevel`]
    /// * `Ok(rfunc (log (max (w, h)) / log (2)) + 1)` if [`level_mode()`](TiledRgbaOutputFile::level_mode()) == [`LevelMode::MipmapLevels`]

    /// * `Err(Error::LogicError)` if [`level_mode()`](TiledRgbaOutputFile::level_mode()) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](TiledRgbaOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaOutputFile_numLevels(self.0, &mut v)
                .into_result()?;
        }

        Ok(v)
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](TiledRgbaOutputFile::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](TiledRgbaOutputFile::level_mode) == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (w) / log (2)) + 1` if [`level_mode()`](TiledRgbaOutputFile::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](TiledRgbaOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaOutputFile_numXLevels(self.0, &mut v).into_result().expect("Unexpected exception from Imf_TiledRgbaOutputFile_numXLevels");
        }
        v
    }

    /// Get the number of levels in the file in the y axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](TiledRgbaOutputFile::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](TiledRgbaOutputFile::level_mode)`TiledRgbaOutputFile::mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (h) / log (2)) + 1` if [`level_mode()`](TiledRgbaOutputFile::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](TiledRgbaOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaOutputFile_numYLevels(self.0, &mut v).into_result().expect("Unexpected exception from Imf_TiledRgbaOutputFile_numYLevels");
        }
        v
    }

    /// Returns `true` if the file contains a level with level number `(lx, ly)`, `false`
    /// otherwise.
    ///
    pub fn is_valid_level(&self, lx: i32, ly: i32) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_TiledRgbaOutputFile_isValidLevel(self.0, &mut v, lx, ly).into_result().expect("Unexpected exception from Imf_TiledRgbaOutputFile_isValidLevel");
        }
        v
    }

    /// Returns the width of the level with level number `(lx, *)`, where `*` is any number.
    ///
    /// # Returns
    /// * `max (1, rfunc (w / pow (2, lx)))`
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](TiledRgbaOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaOutputFile_levelWidth(self.0, &mut v, lx)
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
    /// [`level_rounding_mode()`](TiledRgbaOutputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaOutputFile_levelHeight(self.0, &mut v, ly)
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
            sys::Imf_TiledRgbaOutputFile_numXTiles(self.0, &mut v, lx)
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
            sys::Imf_TiledRgbaOutputFile_numYTiles(self.0, &mut v, ly)
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
            sys::Imf_TiledRgbaOutputFile_dataWindowForLevel(
                self.0,
                dw.as_mut_ptr() as *mut sys::Imath_Box2i_t,
                lx,
                ly,
            )
            .into_result()?
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
            sys::Imf_TiledRgbaOutputFile_dataWindowForTile(
                self.0,
                dw.as_mut_ptr() as *mut sys::Imath_Box2i_t,
                dx,
                dy,
                lx,
                ly,
            )
            .into_result()?
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
    /// ## [`LineOrder::IncreasingY`]
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
    /// where nx = [`num_x_levels()`](TiledRgbaOutputFile::num_x_levels), and ny = [`num_y_levels()`](TiledRgbaOutputFile::num_y_levels).
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
    /// where tx = [`num_x_tiles(lx)`](TiledRgbaOutputFile::num_x_tiles),
    /// and   ty = [`num_y_tiles(ly)`](TiledRgbaOutputFile::num_y_tiles).
    ///
    /// ## [`LineOrder::DecreasingY`]
    /// As for [`LineOrder::IncreasingY`], the tiles
    /// for each level are stored in a contiguous block.  The levels
    /// are ordered the same way as for [`LineOrder::IncreasingY`],
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
    /// ## [`LineOrder::RandomY`]
    /// The order of the calls to `write_tile()` determines
    /// the order of the tiles in the file.
    ///
    /// # Safety
    /// You must ensure the the [`FrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](TiledRgbaOutputFile::set_frame_buffer) has valid slices
    /// for the channels to be written.
    ///
    pub unsafe fn write_tile(
        &mut self,
        dx: i32,
        dy: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        sys::Imf_TiledRgbaOutputFile_writeTile(self.0, dx, dy, lx, ly)
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
    /// ## [`LineOrder::IncreasingY`]
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
    /// where nx = [`num_x_levels()`](TiledRgbaOutputFile::num_x_levels), and ny = [`num_y_levels()`](TiledRgbaOutputFile::num_y_levels).
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
    /// where tx = [`num_x_tiles(lx)`](TiledRgbaOutputFile::num_x_tiles),
    /// and   ty = [`num_y_tiles(ly)`](TiledRgbaOutputFile::num_y_tiles).
    ///
    /// ## [`LineOrder::DecreasingY`]
    /// As for [`LineOrder::IncreasingY`], the tiles
    /// for each level are stored in a contiguous block.  The levels
    /// are ordered the same way as for [`LineOrder::IncreasingY`],
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
    /// ## [`LineOrder::RandomY`]
    /// The order of the calls to `write_tile()` determines
    /// the order of the tiles in the file.
    ///
    /// # Safety
    /// You must ensure the the [`FrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](TiledRgbaOutputFile::set_frame_buffer) has valid slices
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
        sys::Imf_TiledRgbaOutputFile_writeTiles(
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
            sys::Imf_TiledRgbaOutputFile_updatePreviewImage(
                self.0,
                new_pixels.as_ptr() as *const sys::Imf_PreviewRgba_t,
            )
            .into_result()?;
        }

        Ok(())
    }
}

impl Drop for TiledRgbaOutputFile {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_TiledRgbaOutputFile_dtor(self.0)
                .into_result()
                .expect("Unexpected exception in Imf_TiledRgbaOutputFile_dtor");
        }
    }
}

/// A simplified interface for reading a tiled RGBA EXR file
/// ```no_run
/// # use std::path::PathBuf;
/// # let path = PathBuf::from(
/// #     std::env::var("CARGO_MANIFEST_DIR")
/// #         .expect("CARGO_MANIFEST_DIR not set"),
/// # )
/// # .join("images")
/// # .join("ferris-tiled.exr");
///
/// use imath_traits::Zero;
/// use openexr::prelude::*;
///
/// let mut file = TiledRgbaInputFile::new(&path, 1).unwrap();
/// let data_window = file.header().data_window::<[i32; 4]>().clone();
/// let width = data_window[2] - data_window[0] + 1;
/// let height = data_window[3] - data_window[1] + 1;
///
/// let mut pixels = vec![Rgba::zero(); (width * height) as usize];
/// file.set_frame_buffer(&mut pixels, 1, width as usize)
///     .unwrap();
/// unsafe {
///    file.read_tiles(
///        0,
///        file.num_x_tiles(0).unwrap() - 1,
///        0,
///        file.num_y_tiles(0).unwrap() - 1,
///        0,
///        0,
///    )
///    .unwrap();
/// }
/// ```
///
#[repr(transparent)]
pub struct TiledRgbaInputFile(pub(crate) *mut sys::Imf_TiledRgbaInputFile_t);

impl TiledRgbaInputFile {
    /// Opens the file at `filename` and reads the header.
    ///
    /// # Errors
    /// * [`Error::Base`] - If any error occurs
    ///
    pub fn new<P: AsRef<Path>>(
        filename: P,
        num_threads: i32,
    ) -> Result<TiledRgbaInputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_TiledRgbaInputFile_ctor(
                &mut inner,
                c_filename.as_ptr(),
                num_threads,
            )
            .into_result()?;
        }

        Ok(TiledRgbaInputFile(inner))
    }

    /// Opens the file at `filename`, reads the header and prepares to read
    /// layer `layer_name` from the file.
    ///
    /// # Errors
    /// * [`Error::Base`] - If any error occurs
    ///
    pub fn with_layer<P: AsRef<Path>>(
        filename: P,
        layer_name: &str,
        num_threads: i32,
    ) -> Result<TiledRgbaInputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let s_layer_name = CppString::new(layer_name);

        let mut inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_TiledRgbaInputFile_with_layer(
                &mut inner,
                c_filename.as_ptr(),
                s_layer_name.0,
                num_threads,
            )
            .into_result()?;
        }

        Ok(TiledRgbaInputFile(inner))
    }

    /// Set a frame buffer as the destination for the decoded pixels
    ///
    /// Pixel (x, y) is at offset x * x_stride + y * y_stride
    ///
    /// ## Errors
    /// * [`Error::Base`] - If any error occurs
    ///
    pub fn set_frame_buffer(
        &mut self,
        pixels: &mut [Rgba],
        x_stride: usize,
        y_stride: usize,
    ) -> Result<()> {
        unsafe {
            sys::Imf_TiledRgbaInputFile_setFrameBuffer(
                self.0,
                pixels.as_mut_ptr() as *mut Rgba as *mut sys::Imf_Rgba_t,
                x_stride,
                y_stride,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Switch to a different layer for reading.
    ///
    /// Subsequent calls to `read_tile()` and `read_tiles()` will read channels layer_name.R, layer_name.G, etc.
    /// After each call to `set_layer_name()`, `set_frame_buffer()` must be called
    /// at least once before the next call to `read_tile()` or `read_tiles()`.
    ///
    pub fn set_layer_name(&mut self, layer_name: &str) {
        let s_layer_name = CppString::new(layer_name);
        unsafe {
            sys::Imf_TiledRgbaInputFile_setLayerName(self.0, s_layer_name.0).into_result().expect("Unexpected exception from Imf_TiledRgbaInputFile_setLayerName");
        }
    }

    /// Access to the file [`Header`]
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_TiledRgbaInputFile_header(self.0, &mut ptr);
            if ptr.is_null() {
                panic!(
                    "Received null ptr from sys::Imf_TiledRgbaInputFile_header"
                );
            }

            HeaderRef::new(ptr)
        }
    }

    /// Get the [`FrameBuffer`](crate::core::frame_buffer::FrameBuffer)
    ///
    pub fn frame_buffer(&self) -> FrameBufferRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_TiledRgbaInputFile_frameBuffer(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_TiledRgbaInputFile_frameBuffer");
            }

            FrameBufferRef::new(ptr)
        }
    }

    /// Get the tiles' x dimension
    ///
    pub fn tile_x_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaInputFile_tileXSize(self.0, &mut v).into_result().expect("Unexpected exception from Imf_TiledRgbaInputFile_tileXSize");
        }
        v
    }

    /// Get the tiles' y dimension
    ///
    pub fn tile_y_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaInputFile_tileYSize(self.0, &mut v).into_result().expect("Unexpected exception from Imf_TiledRgbaInputFile_tileYSize");
        }
        v
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_TiledRgbaInputFile_levelMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_TiledRgbaInputFile_levelMode",
            );
        }

        v.into()
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_TiledRgbaInputFile_levelRoundingMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_TiledRgbaInputFile_levelRoundingMode",
            );
        }

        v.into()
    }

    /// Get the number of levels in the file
    ///
    /// # Returns
    /// * `Ok(1)` if [`level_mode()`](TiledRgbaInputFile::level_mode()) == [`LevelMode::OneLevel`]
    /// * `Ok(rfunc (log (max (w, h)) / log (2)) + 1)` if [`level_mode()`](TiledRgbaInputFile::level_mode()) == [`LevelMode::MipmapLevels`]

    /// * `Err(Error::LogicError)` if [`level_mode()`](TiledRgbaInputFile::level_mode()) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](TiledRgbaInputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaInputFile_numLevels(self.0, &mut v)
                .into_result()?;
        }

        Ok(v)
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](TiledRgbaInputFile::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](TiledRgbaInputFile::level_mode) == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (w) / log (2)) + 1` if [`level_mode()`](TiledRgbaInputFile::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](TiledRgbaInputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaInputFile_numXLevels(self.0, &mut v).into_result().expect("Unexpected exception from Imf_TiledRgbaInputFile_numXLevels");
        }
        v
    }

    /// Get the number of levels in the file in the y axis
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](TiledRgbaInputFile::level_mode) == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`level_mode()`](TiledRgbaInputFile::level_mode)`TiledRgbaInputFile::mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (h) / log (2)) + 1` if [`level_mode()`](TiledRgbaInputFile::level_mode) == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](TiledRgbaInputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaInputFile_numYLevels(self.0, &mut v).into_result().expect("Unexpected exception from Imf_TiledRgbaInputFile_numYLevels");
        }
        v
    }

    /// Returns `true` if the file contains a level with level number `(lx, ly)`, `false`
    /// otherwise.
    ///
    pub fn is_valid_level(&self, lx: i32, ly: i32) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_TiledRgbaInputFile_isValidLevel(self.0, &mut v, lx, ly).into_result().expect("Unexpected exception from Imf_TiledRgbaInputFile_isValidLevel");
        }
        v
    }

    /// Returns the width of the level with level number `(lx, *)`, where `*` is any number.
    ///
    /// # Returns
    /// * `max (1, rfunc (w / pow (2, lx)))`
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`level_rounding_mode()`](TiledRgbaInputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaInputFile_levelWidth(self.0, &mut v, lx)
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
    /// [`level_rounding_mode()`](TiledRgbaInputFile::level_rounding_mode()) is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledRgbaInputFile_levelHeight(self.0, &mut v, ly)
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
            sys::Imf_TiledRgbaInputFile_numXTiles(self.0, &mut v, lx)
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
            sys::Imf_TiledRgbaInputFile_numYTiles(self.0, &mut v, ly)
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
            sys::Imf_TiledRgbaInputFile_dataWindowForLevel(
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
            sys::Imf_TiledRgbaInputFile_dataWindowForTile(
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
    ///   # Errors
    /// * [`Error::InvalidArgument`] - if dx does not lie in the interval [0, num_x_tiles(lx)-1]
    /// * [`Error::InvalidArgument`] - if dy does not lie in the interval [0, num_y_tiles(ly)-1]
    ///
    /// * [`Error::InvalidArgument`] -if lx does not lie in the interval [0, num_x_levels()-1]
    /// * [`Error::InvalidArgument`] -if ly does not lie in the inverval [0, num_y_levels()-1]
    /// * [`Error::Base`] - if any other error occurs
    ///
    /// # Safety
    /// You must ensure the the [`FrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](TiledRgbaInputFile::set_frame_buffer) has valid slices
    /// for the channels to be written.
    ///
    pub unsafe fn read_tile(
        &mut self,
        dx: i32,
        dy: i32,
        lx: i32,
        ly: i32,
    ) -> Result<()> {
        sys::Imf_TiledRgbaInputFile_readTile(self.0, dx, dy, lx, ly)
            .into_result()?;
        Ok(())
    }

    /// Reads the tile range from with coordinates `(dx1, dy1)`, to `(dx2, dy2)` and level number `(lx, ly)`,
    /// and stores it in the current frame buffer.
    ///
    ///   # Errors
    /// * [`Error::InvalidArgument`] - if dx does not lie in the interval [0, num_x_tiles(lx)-1]
    /// * [`Error::InvalidArgument`] - if dy does not lie in the interval [0, num_y_tiles(ly)-1]
    ///
    /// * [`Error::InvalidArgument`] -if lx does not lie in the interval [0, num_x_levels()-1]
    /// * [`Error::InvalidArgument`] -if ly does not lie in the inverval [0, num_y_levels()-1]
    /// * [`Error::Base`] - if any other error occurs
    ///
    /// # Safety
    /// You must ensure the the [`FrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](TiledRgbaInputFile::set_frame_buffer) has valid slices
    /// for the channels to be written.
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
        sys::Imf_TiledRgbaInputFile_readTiles(
            self.0, dx1, dx2, dy1, dy2, lx, ly,
        )
        .into_result()?;
        Ok(())
    }
}

impl Drop for TiledRgbaInputFile {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_TiledRgbaInputFile_dtor(self.0)
                .into_result()
                .expect("Unexpected exception in Imf_TiledRgbaInputFile_dtor");
        }
    }
}

#[cfg(test)]
#[test]
fn test_write_tiled_rgba1() -> Result<(), Box<dyn std::error::Error>> {
    let (pixels, width, height) = crate::tests::load_ferris();

    let header = Header::from_dimensions(width, height);

    let mut file = TiledRgbaOutputFile::new(
        "write_tiled_rgba1.exr",
        &header,
        RgbaChannels::WriteRgba,
        64,
        64,
        LevelMode::OneLevel,
        LevelRoundingMode::RoundDown,
        1,
    )?;

    file.set_frame_buffer(&pixels, 1, width as usize)?;
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

    Ok(())
}

#[cfg(test)]
#[test]
fn test_read_tiled_rgba1() -> Result<(), Box<dyn std::error::Error>> {
    use crate::prelude::*;
    use imath_traits::Zero;
    use std::path::PathBuf;

    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("ferris-tiled.exr");

    let mut file = TiledRgbaInputFile::new(&path, 1)?;
    let data_window = file.header().data_window::<[i32; 4]>().clone();
    let width = data_window[2] - data_window[0] + 1;
    let height = data_window[3] - data_window[1] + 1;

    let mut pixels = vec![Rgba::zero(); (width * height) as usize];
    file.set_frame_buffer(&mut pixels, 1, width as usize)?;
    unsafe {
        file.read_tiles(
            0,
            file.num_x_tiles(0)? - 1,
            0,
            file.num_y_tiles(0)? - 1,
            0,
            0,
        )?;
    }

    let mut ofile = RgbaOutputFile::with_dimensions(
        "read_tiled_rgba1.exr",
        width,
        height,
        RgbaChannels::WriteRgba,
        1.0f32,
        [0.0f32, 0.0f32],
        1.0f32,
        LineOrder::IncreasingY,
        Compression::Piz,
        1,
    )?;

    ofile.set_frame_buffer(&pixels, 1, width as usize)?;
    unsafe {
        ofile.write_pixels(height)?;
    }

    Ok(())
}
