use crate::core::{
    error::Error,
    frame_buffer::{FrameBuffer, FrameBufferRef},
    header::HeaderRef,
    version::Version,
    LevelMode, LevelRoundingMode,
};

use imath_traits::Bound2;
use openexr_sys as sys;
use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::path::{Path, PathBuf};

type Result<T, E = Error> = std::result::Result<T, E>;

/// A TiledInputFile represents reading a tiled EXR file from disk.
///
#[repr(transparent)]
pub struct TiledInputFile(pub(crate) *mut sys::Imf_TiledInputFile_t);

impl Debug for TiledInputFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TiledInputFile")
            .field("file_name", &self.file_name())
            .finish()
    }
}

impl Drop for TiledInputFile {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_TiledInputFile_dtor(self.0).into_result().unwrap();
        }
    }
}

impl TiledInputFile {
    /// A constructor that opens the file with the specified name, and reads the
    /// file header. The num_threads parameter specifies how many worker threads
    /// this file will try to keep busy when decompressing individual tiles.
    /// Destroying TiledInputFile objects constructed with this constructor
    /// automatically closes the corresponding files.
    ///
    /// # Errors
    ///
    /// This will return an error if the file is not tiled.
    ///
    pub fn new<P: AsRef<Path>>(
        file_name: P,
        num_threads: i32,
    ) -> Result<Self, Error> {
        let mut ptr = std::ptr::null_mut();
        let c_file_name = CString::new(
            file_name
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        unsafe {
            sys::Imf_TiledInputFile_ctor(
                &mut ptr,
                c_file_name.as_ptr(),
                num_threads,
            )
            .into_result()?;

            // TODO: Validate if OpenEXR will always return a valid tiled input
            // file.
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_TiledInputFile_ctor");
            }
        }

        Ok(Self(ptr))
    }

    /// Access to the file path
    ///
    pub fn file_name(&self) -> PathBuf {
        let mut ptr = std::ptr::null();

        unsafe {
            sys::Imf_TiledInputFile_fileName(self.0, &mut ptr)
                .into_result()
                .unwrap();

            // TODO: Validate if OpenEXR will always return a valid string, or
            // if we need to return a Result<PathBuf, Error>.
            if ptr.is_null() {
                panic!(
                    "Received null ptr from sys::Imf_TiledInputFile_fileName"
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
            sys::Imf_TiledInputFile_header(self.0, &mut ptr)
                .into_result()
                .unwrap();

            // TODO: Validate if OpenEXR will always return a valid header, or
            // if we need to return a Result<Header, Error>.
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_TiledInputFile_header");
            }
        }

        HeaderRef::new(ptr)
    }

    /// Access to the file format version
    ///
    pub fn version(&self) -> Version {
        let mut version = 0;

        unsafe {
            sys::Imf_TiledInputFile_version(self.0, &mut version)
                .into_result()
                .unwrap();
        }

        Version::from_c_int(version)
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the TiledInputFile object.
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
            sys::Imf_TiledInputFile_setFrameBuffer(self.0, frame_buffer.ptr)
                .into_result()?;
        }

        Ok(())
    }

    /// Access to the current frame buffer
    ///
    pub fn frame_buffer(&self) -> FrameBufferRef {
        let mut ptr = std::ptr::null();

        unsafe {
            sys::Imf_TiledInputFile_frameBuffer(self.0, &mut ptr)
                .into_result()
                .unwrap();

            // TODO: Validate if OpenEXR will always return a valid frame
            // buffer, or if we need to return a Result<FrameBufferRef, Error>.
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_TiledInputFile_frameBuffer");
            }
        }

        FrameBufferRef::new(ptr)
    }

    /// Check if all pixels in the data window are present in the input file
    ///
    pub fn is_complete(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_TiledInputFile_isComplete(self.0, &mut v);
        }

        v
    }

    /// Get the tiles' x dimension
    ///
    pub fn tile_x_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputFile_tileXSize(self.0, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputFile_tileXSize",
                );
        }
        v
    }

    /// Get the tiles' y dimension
    ///
    pub fn tile_y_size(&self) -> u32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputFile_tileYSize(self.0, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputFile_tileYSize",
                );
        }
        v
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_TiledInputFile_levelMode(self.0, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputFile_levelMode",
                );
        }

        v.into()
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_TiledInputFile_levelRoundingMode(
                self.0,
                &mut v,
            )
            .into_result()
            .expect(
                "Unexpected exception from Imf_TiledInputFile_levelRoundingMode",
            );
        }

        v.into()
    }

    /// Get the number of levels in the file
    ///
    /// # Returns
    /// * `Ok(1)` if [`TiledInputFile::level_mode()`] == [`LevelMode::OneLevel`]
    /// * `Ok(rfunc (log (max (w, h)) / log (2)) + 1)` if [`TiledInputFile::level_mode()`] == [`LevelMode::MipmapLevels`]

    /// * `Err(Error::Logic)` if [`TiledInputFile::level_mode()`] == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`TiledInputFile::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputFile_numLevels(self.0, &mut v).into_result()?;
        }

        Ok(v)
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`TiledInputFile::level_mode()`] == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`TiledInputFile::level_mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (w) / log (2)) + 1` if [`TiledInputFile::level_mode()`] == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`TiledInputFile::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputFile_numXLevels(self.0, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputFile_numXLevels",
                );
        }
        v
    }

    /// Get the number of levels in the file in the x axis
    ///
    /// # Returns
    /// * `1` if [`TiledInputFile::level_mode()`] == [`LevelMode::OneLevel`]
    /// * `rfunc (log (max (w, h)) / log (2)) + 1` if [`TiledInputFile::level_mode()`] == [`LevelMode::MipmapLevels`]

    /// * `rfunc (log (h) / log (2)) + 1` if [`TiledInputFile::level_mode()`] == [`LevelMode::RipmapLevels`]
    ///
    /// where `rfunc` is either `floor()` or `ceil()` depending on whether
    /// [`TiledInputFile::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputFile_numYLevels(self.0, &mut v)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputFile_numYLevels",
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
            sys::Imf_TiledInputFile_isValidLevel(self.0, &mut v, lx, ly)
                .into_result()
                .expect(
                    "Unexpected exception from Imf_TiledInputFile_isValidLevel",
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
    /// [`TiledInputFile::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputFile_levelWidth(self.0, &mut v, lx)
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
    /// [`TiledInputFile::level_rounding_mode()`] is [`LevelRoundingMode::RoundUp`] or [`LevelRoundingMode::RoundDown`]
    ///
    /// # Errors
    /// *[`Error::Base`] - If any error occurs
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_TiledInputFile_levelHeight(self.0, &mut v, ly)
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
            sys::Imf_TiledInputFile_numXTiles(self.0, &mut v, lx)
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
            sys::Imf_TiledInputFile_numYTiles(self.0, &mut v, ly)
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
            sys::Imf_TiledInputFile_dataWindowForLevel(
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
            sys::Imf_TiledInputFile_dataWindowForTile(
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
    /// You must ensure the the [`FrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](TiledInputFile::set_frame_buffer) has valid slices
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
        sys::Imf_TiledInputFile_readTile(self.0, dx, dy, lx, ly)
            .into_result()?;
        Ok(())
    }

    /// Reads the sample counts in tile range with coordinates `(dx1, dy1)`, to `(dx2, dy2)` and level number `(lx, ly)`,
    /// and stores it in the current frame buffer.
    ///
    /// # Safety
    /// You must ensure the the [`FrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](TiledInputFile::set_frame_buffer) has valid slices
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
        sys::Imf_TiledInputFile_readTiles(self.0, dx1, dx2, dy1, dy2, lx, ly)
            .into_result()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::core::{
        frame_buffer::FrameBuffer, LevelMode, LevelRoundingMode,
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

    fn get_tiled_input_file() -> super::TiledInputFile {
        super::TiledInputFile::new(&*SRC_IMAGE_PATH, 0).unwrap()
    }

    #[test]
    fn test_tiledinputfile_new_0_threads_success() {
        super::TiledInputFile::new(&*SRC_IMAGE_PATH, 0).unwrap();
    }

    #[test]
    fn test_tiledinputfile_new_1_threads_success() {
        super::TiledInputFile::new(&*SRC_IMAGE_PATH, 1).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_new_failure_invalid_file_path() {
        let path = PathBuf::from("/this/does/not/exist.exr");
        super::TiledInputFile::new(&path, 1).unwrap();
    }

    #[test]
    fn test_tiledinputfile_file_name_success() {
        let tiled_input_file = get_tiled_input_file();
        let result = tiled_input_file.file_name();

        assert_eq!(&*SRC_IMAGE_PATH, &result);
    }

    #[test]
    fn test_tiledinputfile_header_success() {
        let tiled_input_file = get_tiled_input_file();
        let header = tiled_input_file.header();

        header.sanity_check(true, false).unwrap();
    }

    #[test]
    fn test_tiledinputfile_version_success() {
        let tiled_input_file = get_tiled_input_file();
        let version = tiled_input_file.version();

        assert_eq!(version.is_tiled(), true);
    }

    #[test]
    fn test_tiledinputfile_set_frame_buffer_success() {
        let mut tiled_input_file = get_tiled_input_file();
        let frame_buffer = FrameBuffer::new();
        tiled_input_file.set_frame_buffer(&frame_buffer).unwrap();
    }

    #[test]
    fn test_tiledinputfile_frame_buffer_success() {
        let tiled_input_file = get_tiled_input_file();
        let _ = tiled_input_file.frame_buffer();
    }

    #[test]
    fn test_tiledinputfile_is_complete_success() {
        let tiled_input_file = get_tiled_input_file();
        let is_complete = tiled_input_file.is_complete();

        assert_eq!(is_complete, true);
    }

    #[test]
    fn test_tiledinputfile_tile_x_size_success() {
        let tiled_input_file = get_tiled_input_file();
        let tile_x_size = tiled_input_file.tile_x_size();

        assert_eq!(tile_x_size, 64);
    }

    #[test]
    fn test_tiledinputfile_tile_y_size_success() {
        let tiled_input_file = get_tiled_input_file();
        let tile_y_size = tiled_input_file.tile_y_size();

        assert_eq!(tile_y_size, 64);
    }

    #[test]
    fn test_tiledinputfile_level_mode_success() {
        let tiled_input_file = get_tiled_input_file();
        let level_mode = tiled_input_file.level_mode();

        assert_eq!(level_mode, LevelMode::OneLevel);
    }

    #[test]
    fn test_tiledinputfile_level_rounding_mode_success() {
        let tiled_input_file = get_tiled_input_file();
        let level_rounding_mode = tiled_input_file.level_rounding_mode();

        assert_eq!(level_rounding_mode, LevelRoundingMode::RoundDown);
    }

    #[test]
    fn test_tiledinputfile_num_levels_success() {
        let tiled_input_file = get_tiled_input_file();
        let num_levels = tiled_input_file.num_levels().unwrap();

        assert_eq!(num_levels, 1);
    }

    #[test]
    fn test_tiledinputfile_num_x_levels_success() {
        let tiled_input_file = get_tiled_input_file();
        let num_x_levels = tiled_input_file.num_x_levels();

        assert_eq!(num_x_levels, 1);
    }

    #[test]
    fn test_tiledinputfile_num_y_levels_success() {
        let tiled_input_file = get_tiled_input_file();
        let num_y_levels = tiled_input_file.num_y_levels();

        assert_eq!(num_y_levels, 1);
    }

    #[test]
    fn test_tiledinputfile_is_valid_level_success() {
        let tiled_input_file = get_tiled_input_file();
        let is_valid_level = tiled_input_file.is_valid_level(0, 0);

        assert_eq!(is_valid_level, true);
    }

    #[test]
    fn test_tiledinputfile_is_valid_level_failure_invalid_level() {
        let tiled_input_file = get_tiled_input_file();
        let is_valid_level = tiled_input_file.is_valid_level(100, 100);

        assert_eq!(is_valid_level, false);
    }

    #[test]
    fn test_tiledinputfile_level_width_success() {
        let tiled_input_file = get_tiled_input_file();
        let level_width = tiled_input_file.level_width(0).unwrap();

        assert_eq!(level_width, 1200);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_level_width_failure_invalid_level() {
        let tiled_input_file = get_tiled_input_file();
        tiled_input_file.level_width(i32::MIN).unwrap();
    }

    #[test]
    fn test_tiledinputfile_level_height_success() {
        let tiled_input_file = get_tiled_input_file();
        let level_height = tiled_input_file.level_height(0).unwrap();

        assert_eq!(level_height, 800);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_level_height_failure_invalid_level() {
        let tiled_input_file = get_tiled_input_file();
        tiled_input_file.level_height(i32::MIN).unwrap();
    }

    #[test]
    fn test_tiledinputfile_num_x_tiles_success() {
        let tiled_input_file = get_tiled_input_file();
        let num_x_tiles = tiled_input_file.num_x_tiles(0).unwrap();

        assert_eq!(num_x_tiles, 19);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_num_x_tiles_failed_invalid_level() {
        let tiled_input_file = get_tiled_input_file();
        tiled_input_file.num_x_tiles(i32::MIN).unwrap();
    }

    #[test]
    fn test_tiledinputfile_num_y_tiles_success() {
        let tiled_input_file = get_tiled_input_file();
        let num_y_tiles = tiled_input_file.num_y_tiles(0).unwrap();

        assert_eq!(num_y_tiles, 13);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_num_y_tiles_failed_invalid_level() {
        let tiled_input_file = get_tiled_input_file();
        tiled_input_file.num_y_tiles(i32::MIN).unwrap();
    }

    #[test]
    fn test_tiledinputfile_data_window_for_level_success() {
        let tiled_input_file = get_tiled_input_file();
        let data_window_for_level: [i32; 4] =
            tiled_input_file.data_window_for_level(0, 0).unwrap();

        assert_eq!(data_window_for_level, [0, 0, 1199, 799]);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_data_window_for_level_failure_invalid_level() {
        let tiled_input_file = get_tiled_input_file();
        tiled_input_file
            .data_window_for_level::<[i32; 4]>(i32::MIN, i32::MIN)
            .unwrap();
    }

    #[test]
    fn test_tiledinputfile_data_window_for_tile_success() {
        let tiled_input_file = get_tiled_input_file();
        let data_window_for_tile: [i32; 4] =
            tiled_input_file.data_window_for_tile(0, 0, 0, 0).unwrap();

        assert_eq!(data_window_for_tile, [0, 0, 63, 63]);
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_data_window_for_tile_failure_invalid_level() {
        let tiled_input_file = get_tiled_input_file();
        tiled_input_file
            .data_window_for_tile::<[i32; 4]>(
                i32::MIN,
                i32::MIN,
                i32::MIN,
                i32::MIN,
            )
            .unwrap();
    }

    #[test]
    fn test_tiledinputfile_read_tile_success() {
        let mut tiled_input_file = get_tiled_input_file();
        let frame_buffer = FrameBuffer::new();
        tiled_input_file.set_frame_buffer(&frame_buffer).unwrap();
        unsafe {
            tiled_input_file.read_tile(0, 0, 0, 0).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_read_tile_failure_invalid_level_and_frame_buffer() {
        let mut tiled_input_file = get_tiled_input_file();
        unsafe {
            tiled_input_file
                .read_tile(i32::MIN, i32::MIN, i32::MIN, i32::MIN)
                .unwrap();
        }
    }

    #[test]
    fn test_tiledinputfile_read_tiles_success() {
        let mut tiled_input_file = get_tiled_input_file();
        let frame_buffer = FrameBuffer::new();
        tiled_input_file.set_frame_buffer(&frame_buffer).unwrap();
        unsafe {
            tiled_input_file.read_tiles(0, 0, 0, 0, 0, 0).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn test_tiledinputfile_read_tiles_failure_invalid_level_and_frame_buffer() {
        let mut tiled_input_file = get_tiled_input_file();
        unsafe {
            tiled_input_file
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
}
