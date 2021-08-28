use openexr_sys as sys;

use crate::{
    core::{
        channel_list::Channel,
        cppstd::CppString,
        error::Error,
        refptr::{OpaquePtr, Ref, RefMut},
        LevelMode, LevelRoundingMode,
    },
    flat::flat_image_level::{FlatImageLevelRef, FlatImageLevelRefMut},
};

use imath_traits::Bound2;

type Result<T, E = Error> = std::result::Result<T, E>;

/// An in-memory data structure that can hold an arbitrary OpenEXR image with
/// one or multiple resolution levels, and with an arbitrary set of channels.
///
/// An image is a container for a set of image levels, and an image level
/// is a container for a set of image channels.  An image channel contains
/// an array of pixel values of type f16, f32 or u32.
///
/// For example:
///
/// ```text
///     image ──┬── level 0 ──┬── channel "R" ─── pixel data
///             │             │
///             │             ├── channel "G" ─── pixel data
///             │             │
///             │             └── channel "B" ─── pixel data
///             │
///             ├── level 1 ──┬── channel "R" ─── pixel data
///             │             │
///             │             ├── channel "G" ─── pixel data
///             │             │
///             │             └── channel "B" ─── pixel data
///             │
///             └── level 2 ──┬── channel "R" ─── pixel data
///                           │
///                           ├── channel "G" ─── pixel data
///                           │
///                           └── channel "B" ─── pixel data
/// ```
///
/// An image has a level mode, which can be [`LevelMode::OneLevel`],
/// [`LevelMode::MipmapLevels`] or [`LevelMode::RipmapLevels`], and a level
/// rounding mode, which can be [`LevelRoundingMode::RoundUp`] or
/// [`LevelRoundingMode::RoundDown`]. Together, the level mode and the level
/// rounding mode determine how many levels an image contains, and how large the
/// data window for each level is. All levels in an image have the same set of
/// channels.
///
/// An image channel has a name (e.g. "R", "Z", or "xVelocity"), a type
/// ([`CHANNEL_HALF`](crate::core::channel_list::CHANNEL_HALF), [`CHANNEL_FLOAT`](crate::core::channel_list::CHANNEL_FLOAT) or [`CHANNEL_UINT`](crate::core::channel_list::CHANNEL_UINT)) and x and y sampling
/// rates. A channel stores samples for a pixel if the pixel is inside the data
/// window of the level to which the channel belongs, and the x and y coordinates
/// of the pixel are divisible by the x and y sampling rates of the channel.
///
/// In a flat image each channel in each level stores at most one value per pixel.  
///
#[repr(transparent)]
pub struct FlatImage(pub(crate) *mut sys::Imf_FlatImage_t);

unsafe impl OpaquePtr for FlatImage {
    type SysPointee = sys::Imf_FlatImage_t;
    type Pointee = FlatImage;
}

pub type FlatImageRef<'a, P = FlatImage> = Ref<'a, P>;
pub type FlatImageRefMut<'a, P = FlatImage> = RefMut<'a, P>;

impl FlatImage {
    /// Constructs a new `FlatImage` with the given `data_window`, `level_mode` and
    /// `level_rounding_mode`
    ///
    /// # Errors
    /// * [`Error::Base`] - if any error occurs
    pub fn new<B: Bound2<i32>>(
        data_window: B,
        level_mode: LevelMode,
        level_rounding_mode: LevelRoundingMode,
    ) -> Result<FlatImage> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FlatImage_ctor(
                &mut ptr,
                data_window.as_ptr() as *const sys::Imath_Box2i_t,
                level_mode.into(),
                level_rounding_mode.into(),
            )
            .into_result()?;
        }
        Ok(FlatImage(ptr))
    }

    /// Get the level mode
    ///
    pub fn level_mode(&self) -> LevelMode {
        let mut v = sys::Imf_LevelMode(0);
        unsafe {
            sys::Imf_FlatImage_levelMode(self.0, &mut v);
            v.into()
        }
    }

    /// Get the level rounding mode
    ///
    pub fn level_rounding_mode(&self) -> LevelRoundingMode {
        let mut v = sys::Imf_LevelRoundingMode(0);
        unsafe {
            sys::Imf_FlatImage_levelRoundingMode(self.0, &mut v);
            v.into()
        }
    }

    /// Get the number of levels in the image.
    ///
    /// This is a convenience function for use with mipmaps, in which case this
    /// function is equivalent to [`num_x_levels()`](FlatImage::num_x_levels).
    ///
    /// If this image's level mode is [`LevelMode::RipmapLevels`], you must call
    /// [`num_x_levels()`](FlatImage::num_x_levels) or [`num_y_levels`](FlatImage::num_y_levels) instead.
    ///
    /// # Errors
    /// * [`Error::LogicError`] - if this image's level mode is [`LevelMode::RipmapLevels`]
    ///
    pub fn num_levels(&self) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatImage_numLevels(self.0, &mut v).into_result()?;
        }
        Ok(v)
    }

    /// Returns the image's number of levels in the x direction.
    ///
    /// # Returns
    /// * `1` if [`level_mode()`](FlatImage::level_mode) is [`LevelMode::OneLevel`]
    /// * `rfunc(log(max(w, h)) / log(2)) + 1` if [`level_mode()`](FlatImage::level_mode) is [`LevelMode::MipmapLevels`]
    /// * `rfunc(log(w) / log(2)) + 1` if [`level_mode()`](FlatImage::level_mode) is [`LevelMode::RipmapLevels`]
    ///
    /// Where:
    /// * `w` is the width of the image's data window,  max.x - min.x + 1,
    /// * `h` is the height of the image's data window, max.y - min.y + 1,
    /// * and `rfunc(x)` is either `floor(x)`, or `ceil(x)`, depending on
    /// whether [`level_rounding_mode()`](FlatImage::level_rounding_mode) returns [`LevelRoundingMode::RoundDown`] or
    /// [`LevelRoundingMode::RoundUp`].
    ///
    pub fn num_x_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatImage_numXLevels(self.0, &mut v);
        }
        v
    }

    /// Returns the image's number of levels in the y direction.
    ///
    /// # Returns
    /// * Same as [`num_x_levels()`](FlatImage::num_x_levels) if [`level_mode()`](FlatImage::level_mode) is [`LevelMode::OneLevel`] or [`LevelMode::MipmapLevels`]
    /// * `rfunc(log(h) / log(2)) + 1` if [`level_mode()`](FlatImage::level_mode) is [`LevelMode::RipmapLevels`]
    ///
    /// Where:
    /// * `h` is the height of the image's data window, max.y - min.y + 1,
    /// * and `rfunc(x)` is either `floor(x)`, or `ceil(x)`, depending on
    /// whether [`level_rounding_mode()`](FlatImage::level_rounding_mode) returns [`LevelRoundingMode::RoundDown`] or
    /// [`LevelRoundingMode::RoundUp`].
    ///
    pub fn num_y_levels(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatImage_numXLevels(self.0, &mut v);
        }
        v
    }

    /// Returns the data window for the whole image.
    ///
    /// Equivalent to `data_window_for_level(0, 0)`.
    ///
    pub fn data_window<B: Bound2<i32>>(&self) -> &B {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatImage_dataWindow(self.0, &mut ptr);
            &*(ptr as *const B)
        }
    }

    /// Returns the data window for the image at level `(lx, ly)`.
    ///
    /// That is, the window for which the image level with level number
    /// (lx, ly) has allocated pixel storage.
    ///
    /// # Returns
    /// A reference to a `Bound2<i32>` with min value (dataWindow().min.x, dataWindow().min.y) and max value (dataWindow().min.x + levelWidth(lx) - 1, dataWindow().min.y + levelHeight(ly) - 1)
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if `(lx, ly)` does not correspond to a
    /// valid image level.
    ///
    pub fn data_window_for_level<B: Bound2<i32>>(
        &self,
        lx: i32,
        ly: i32,
    ) -> Result<&B> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatImage_dataWindowForLevel(self.0, &mut ptr, lx, ly)
                .into_result()?;

            Ok(&*(ptr as *const B))
        }
    }

    /// Returns the width of a level with level number (lx, *), where * is any number.
    ///
    /// # Returns
    /// `max (1, rfunc (w / pow (2, lx)))`
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if `lx` is not a valid level number
    ///
    pub fn level_width(&self, lx: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatImage_levelWidth(self.0, &mut v, lx).into_result()?;
        }
        Ok(v)
    }

    /// Returns the height of a level with level number (*, ly), where * is any number.
    ///
    /// # Returns
    /// `max (1, rfunc (y / pow (2, ly)))`
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if `ly` is not a valid level number
    ///
    pub fn level_height(&self, ly: i32) -> Result<i32> {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatImage_levelHeight(self.0, &mut v, ly).into_result()?;
        }
        Ok(v)
    }

    /// Resize the image.
    ///
    /// Sets the data window of the image to `dw`, sets the level mode to `lm`
    /// and the level rounding mode to `lrm`, and allocates new storage for image
    /// levels and image channels.  The set of channels in the image does not
    /// change.
    ///
    /// The contents of the image are lost; pixel data are not preserved across
    /// the resize operation.  If resizing fails, then the image will be left
    /// with an empty data window and no image levels.
    ///
    /// # Errors
    /// *[`Error::Base`] - if any error occurs. The image will be set to empty
    /// in this case.
    ///
    pub fn resize<B: Bound2<i32>>(
        &mut self,
        dw: B,
        lm: LevelMode,
        lrm: LevelRoundingMode,
    ) -> Result<()> {
        unsafe {
            sys::Imf_FlatImage_resize(
                self.0,
                dw.as_ptr() as *const sys::Imath_Box2i_t,
                lm.into(),
                lrm.into(),
            )
            .into_result()?;
        }
        Ok(())
    }

    /// Shift the pixels and the data window of an image.
    ///
    /// Shifts the image by `dx` pixels horizontally and `dy` pixels vertically.  
    /// A pixel at location `(x,y)` moves to position `(x+dx, y+dy)`.  The data
    /// window of the image is shifted along with the pixels.  No pixel data are
    /// lost.
    ///
    /// The horizontal and vertical shift distances must be multiples of
    /// the x and y sampling rates of all image channels.  If they are not,
    /// [`Error::InvalidArgument`] is returned and this method has no effect.
    ///
    pub fn shift_pixels(&mut self, dx: i32, dy: i32) -> Result<()> {
        unsafe {
            sys::Imf_FlatImage_shiftPixels(self.0, dx, dy).into_result()?;
        }
        Ok(())
    }

    /// Insert a new channel into the image.
    ///
    /// The arguments to this function are the same as for adding a
    /// a channel to a [`Header`](crate::core::header::Header): channel name, x and y sampling
    /// rates, and a "perceptually approximately linear" flag.
    ///
    /// If the image already contains a channel with the same name
    /// as the new name then the existing channel is deleted before
    /// the new channel is added.
    ///
    /// # Errors
    /// * [`Error::Base`] - if any error occurs
    ///
    pub fn insert_channel(
        &mut self,
        name: &str,
        channel: &Channel,
    ) -> Result<()> {
        unsafe {
            let s = CppString::new(name);

            sys::Imf_FlatImage_insertChannel(
                self.0,
                s.0,
                channel.type_,
                channel.x_sampling,
                channel.y_sampling,
                channel.p_linear,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Erase the channel `name` from the image
    ///
    /// If no channel called `name` is in the image, this method has no effect.
    ///
    pub fn erase_channel(&mut self, name: &str) {
        unsafe {
            let s = CppString::new(name);
            sys::Imf_FlatImage_eraseChannel(self.0, s.0);
        }
    }

    /// Erase all channels from the image.
    ///
    pub fn clear_channels(&mut self) {
        unsafe {
            sys::Imf_FlatImage_clearChannels(self.0);
        }
    }

    /// Rename the channel `old_name` to have the name `new_name`
    ///
    /// # Errors
    /// [`Error::InvalidArgument`] - if a channel called `new_name` already exists,
    /// or if no channel named `old_name` exists.
    /// [`Error::OutOfMemory`] - if allocation fails, in which case the channel
    /// being renamed is erased.
    ///
    pub fn rename_channel(
        &mut self,
        old_name: &str,
        new_name: &str,
    ) -> Result<()> {
        unsafe {
            let sold = CppString::new(old_name);

            let snew = CppString::new(new_name);

            sys::Imf_FlatImage_renameChannel(self.0, sold.0, snew.0)
                .into_result()?;
        }

        Ok(())
    }

    /// Access image level with number `(lx, ly)`
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if `(lx, ly`) does not refer to a valid image level.
    ///
    pub fn level(&self, lx: i32, ly: i32) -> Result<FlatImageLevelRef> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatImage_level_const(self.0, &mut ptr, lx, ly)
                .into_result()?;
        }
        Ok(FlatImageLevelRef::new(ptr))
    }

    /// Access image level with number `(lx, ly)` as a mutable ref.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if `(lx, ly`) does not refer to a valid image level.
    ///
    pub fn level_mut(&self, lx: i32, ly: i32) -> Result<FlatImageLevelRefMut> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FlatImage_level(self.0, &mut ptr, lx, ly).into_result()?;
        }
        Ok(FlatImageLevelRefMut::new(ptr))
    }
}

impl Default for FlatImage {
    /// Constructs a `FlatImage` with an empty data window, [`LevelMode::OneLevel`] level and
    /// [`LevelRoundingMode::RoundDown`]
    ///
    fn default() -> FlatImage {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FlatImage_default(&mut ptr);
        }
        FlatImage(ptr)
    }
}

impl Drop for FlatImage {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_FlatImage_dtor(self.0);
        }
    }
}
