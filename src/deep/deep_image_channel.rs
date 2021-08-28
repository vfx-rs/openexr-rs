use crate::{
    core::{
        channel_list::Channel,
        channel_list::CHANNEL_HALF,
        error::Error,
        refptr::{OpaquePtr, Ref, RefMut},
    },
    deep::{
        deep_frame_buffer::DeepSlice,
        deep_image_level::{DeepImageLevelRef, DeepImageLevelRefMut},
    },
};
use half::f16;
use openexr_sys as sys;

type Result<T, E = Error> = std::result::Result<T, E>;

/// Holds the pixel data for a single half-float channel in one level in a
/// [`DeepImage`](crate::deep::deep_image::DeepImage)
///
#[repr(transparent)]
pub struct DeepChannelF16(pub(crate) *mut sys::Imf_DeepHalfChannel_t);

unsafe impl OpaquePtr for DeepChannelF16 {
    type SysPointee = sys::Imf_DeepHalfChannel_t;
    type Pointee = DeepChannelF16;
}

pub type DeepChannelF16Ref<'a, P = DeepChannelF16> = Ref<'a, P>;
pub type DeepChannelF16RefMut<'a, P = DeepChannelF16> = RefMut<'a, P>;

/// Holds the pixel data for a single float channel in one level in a
/// [`DeepImage`](crate::deep::deep_image::DeepImage)
///
#[repr(transparent)]
pub struct DeepChannelF32(pub(crate) *mut sys::Imf_DeepFloatChannel_t);

unsafe impl OpaquePtr for DeepChannelF32 {
    type SysPointee = sys::Imf_DeepFloatChannel_t;
    type Pointee = DeepChannelF32;
}

pub type DeepChannelF32Ref<'a, P = DeepChannelF32> = Ref<'a, P>;
pub type DeepChannelF32RefMut<'a, P = DeepChannelF32> = RefMut<'a, P>;

/// Holds the pixel data for a single unsigned int channel in one level in a
/// [`DeepImage`](crate::deep::deep_image::DeepImage)
///
#[repr(transparent)]
pub struct DeepChannelU32(pub(crate) *mut sys::Imf_DeepUIntChannel_t);

unsafe impl OpaquePtr for DeepChannelU32 {
    type SysPointee = sys::Imf_DeepUIntChannel_t;
    type Pointee = DeepChannelU32;
}

pub type DeepChannelU32Ref<'a, P = DeepChannelU32> = Ref<'a, P>;
pub type DeepChannelU32RefMut<'a, P = DeepChannelU32> = RefMut<'a, P>;

impl DeepChannelF16 {
    /// Get the [`Channel`] description for this channel
    ///
    pub fn channel(&self) -> Channel {
        let mut c = CHANNEL_HALF;
        unsafe {
            sys::Imf_DeepHalfChannel_channel(self.0, &mut c);
        }
        c
    }

    /// Get the sampling rate in the `x` direction
    ///
    pub fn x_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepHalfChannel_xSampling(self.0, &mut v);
            v
        }
    }

    /// Get the sampling rate in the `y` direction
    ///
    pub fn y_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepHalfChannel_ySampling(self.0, &mut v);
            v
        }
    }

    /// Is this channel perceptually linear?
    ///
    pub fn p_linear(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepHalfChannel_pLinear(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each row of this channel
    ///
    pub fn pixels_per_row(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepHalfChannel_pixelsPerRow(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each column of this channel
    ///
    pub fn pixels_per_column(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepHalfChannel_pixelsPerColumn(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there total in this channel
    ///
    pub fn num_pixels(&self) -> usize {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepHalfChannel_numPixels(self.0, &mut v);
            v as usize
        }
    }

    /// Construct a [`DeepSlice`] for this Channel
    ///
    pub fn slice(&self) -> DeepSlice {
        let mut s = sys::Imf_DeepSlice_t::default();
        unsafe {
            sys::Imf_DeepHalfChannel_slice(self.0, &mut s);
        }
        DeepSlice(s)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn deep_level(&self) -> DeepImageLevelRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepHalfChannel_deepLevel_const(self.0, &mut ptr);
        }
        DeepImageLevelRef::new(ptr)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn deep_level_mut(&mut self) -> DeepImageLevelRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepHalfChannel_deepLevel(self.0, &mut ptr);
        }
        DeepImageLevelRefMut::new(ptr)
    }

    /// Get a reference to the pixel at `[x, y]`
    ///
    pub fn get(&self, x: i32, y: i32) -> Option<&f16> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepHalfChannel_at_const(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &*(ptr as *const f16))
        }
    }

    /// Get a mutable reference to the pixel at `[x, y]`
    ///
    pub fn get_mut(&self, x: i32, y: i32) -> Option<&mut f16> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepHalfChannel_at(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &mut *(ptr as *mut f16))
        }
    }

    /*
    // FIXME: need to figure out how we can handle rows - if we can?
    /// Get row `r` of the channel as a slice
    ///
    pub fn row(&self, r: i32) -> Option<&[f16]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null();
            unsafe {
                sys::Imf_DeepHalfChannel_row_const(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts(
                        ptr as *const f16,
                        self.pixels_per_row() as usize,
                    ))
                }
            }
        }
    }

    /// Get row `r` of the channel as a mutable slice
    ///
    pub fn row_mut(&mut self, r: i32) -> Option<&mut [f16]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null_mut();
            unsafe {
                sys::Imf_DeepHalfChannel_row(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts_mut(
                        ptr as *mut f16,
                        self.pixels_per_row() as usize,
                    ))
                }
            }
        }
    }
    */
}

impl DeepChannelF32 {
    /// Get the [`Channel`] description for this channel
    ///
    pub fn channel(&self) -> Channel {
        let mut c = CHANNEL_HALF;
        unsafe {
            sys::Imf_DeepFloatChannel_channel(self.0, &mut c);
        }
        c
    }

    /// Get the sampling rate in the `x` direction
    ///
    pub fn x_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepFloatChannel_xSampling(self.0, &mut v);
            v
        }
    }

    /// Get the sampling rate in the `y` direction
    ///
    pub fn y_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepFloatChannel_ySampling(self.0, &mut v);
            v
        }
    }

    /// Is this channel perceptually linear?
    ///
    pub fn p_linear(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepFloatChannel_pLinear(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each row of this channel
    ///
    pub fn pixels_per_row(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepFloatChannel_pixelsPerRow(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each column of this channel
    ///
    pub fn pixels_per_column(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepFloatChannel_pixelsPerColumn(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there total in this channel
    ///
    pub fn num_pixels(&self) -> usize {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepFloatChannel_numPixels(self.0, &mut v);
            v as usize
        }
    }

    /// Construct a [`DeepSlice`] for this Channel
    ///
    pub fn slice(&self) -> DeepSlice {
        let mut s = sys::Imf_DeepSlice_t::default();
        unsafe {
            sys::Imf_DeepFloatChannel_slice(self.0, &mut s);
        }
        DeepSlice(s)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn deep_level(&self) -> DeepImageLevelRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepFloatChannel_deepLevel_const(self.0, &mut ptr);
        }
        DeepImageLevelRef::new(ptr)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn deep_level_mut(&mut self) -> DeepImageLevelRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepFloatChannel_deepLevel(self.0, &mut ptr);
        }
        DeepImageLevelRefMut::new(ptr)
    }

    /// Get a reference to the pixel at `[x, y]`
    ///
    pub fn get(&self, x: i32, y: i32) -> Option<&f32> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepFloatChannel_at_const(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &*(ptr as *const f32))
        }
    }

    /// Get a mutable reference to the pixel at `[x, y]`
    ///
    pub fn get_mut(&self, x: i32, y: i32) -> Option<&mut f32> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepFloatChannel_at(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &mut *(ptr as *mut f32))
        }
    }

    /*
    /// Get row `r` of the channel as a slice
    ///
    pub fn row(&self, r: i32) -> Option<&[f32]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null();
            unsafe {
                sys::Imf_DeepFloatChannel_row_const(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts(
                        ptr as *const f32,
                        self.pixels_per_row() as usize,
                    ))
                }
            }
        }
    }

    /// Get row `r` of the channel as a mutable slice
    ///
    pub fn row_mut(&mut self, r: i32) -> Option<&mut [f32]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null_mut();
            unsafe {
                sys::Imf_DeepFloatChannel_row(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts_mut(
                        ptr as *mut f32,
                        self.pixels_per_row() as usize,
                    ))
                }
            }
        }
    }
    */
}

impl DeepChannelU32 {
    /// Get the [`Channel`] description for this channel
    ///
    pub fn channel(&self) -> Channel {
        let mut c = CHANNEL_HALF;
        unsafe {
            sys::Imf_DeepUIntChannel_channel(self.0, &mut c);
        }
        c
    }

    /// Get the sampling rate in the `x` direction
    ///
    pub fn x_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepUIntChannel_xSampling(self.0, &mut v);
            v
        }
    }

    /// Get the sampling rate in the `y` direction
    ///
    pub fn y_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepUIntChannel_ySampling(self.0, &mut v);
            v
        }
    }

    /// Is this channel perceptually linear?
    ///
    pub fn p_linear(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepUIntChannel_pLinear(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each row of this channel
    ///
    pub fn pixels_per_row(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepUIntChannel_pixelsPerRow(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each column of this channel
    ///
    pub fn pixels_per_column(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepUIntChannel_pixelsPerColumn(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there total in this channel
    ///
    pub fn num_pixels(&self) -> usize {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepUIntChannel_numPixels(self.0, &mut v);
            v as usize
        }
    }

    /// Construct a [`DeepSlice`] for this Channel
    ///
    pub fn slice(&self) -> DeepSlice {
        let mut s = sys::Imf_DeepSlice_t::default();
        unsafe {
            sys::Imf_DeepUIntChannel_slice(self.0, &mut s);
        }
        DeepSlice(s)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn deep_level(&self) -> DeepImageLevelRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepUIntChannel_deepLevel_const(self.0, &mut ptr);
        }
        DeepImageLevelRef::new(ptr)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn deep_level_mut(&mut self) -> DeepImageLevelRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepUIntChannel_deepLevel(self.0, &mut ptr);
        }
        DeepImageLevelRefMut::new(ptr)
    }

    /// Get a reference to the pixel at `[x, y]`
    ///
    pub fn get(&self, x: i32, y: i32) -> Option<&u32> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepUIntChannel_at_const(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &*(ptr as *const u32))
        }
    }

    /// Get a mutable reference to the pixel at `[x, y]`
    ///
    pub fn get_mut(&self, x: i32, y: i32) -> Option<&mut u32> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepUIntChannel_at(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &mut *(ptr as *mut u32))
        }
    }

    /*
    /// Get row `r` of the channel as a slice
    ///
    pub fn row(&self, r: i32) -> Option<&[u32]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null();
            unsafe {
                sys::Imf_DeepUIntChannel_row_const(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts(
                        ptr as *const *const u32,
                        self.pixels_per_row() as usize,
                    ))
                }
            }
        }
    }

    /// Get row `r` of the channel as a mutable slice
    ///
    pub fn row_mut(&mut self, r: i32) -> Option<&mut [u32]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null_mut();
            unsafe {
                sys::Imf_DeepUIntChannel_row(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts_mut(
                        ptr as *mut *mut u32,
                        self.pixels_per_row() as usize,
                    ))
                }
            }
        }
    }
    */
}
