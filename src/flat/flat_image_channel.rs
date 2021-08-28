use crate::{
    core::{
        channel_list::{Channel, CHANNEL_HALF},
        error::Error,
        frame_buffer::Slice,
        refptr::{OpaquePtr, Ref, RefMut},
    },
    flat::flat_image_level::{FlatImageLevelRef, FlatImageLevelRefMut},
};
use half::f16;
use openexr_sys as sys;

type Result<T, E = Error> = std::result::Result<T, E>;

/// Holds the pixel data for a single half-float channel in one level in a
/// [`FlatImage`](crate::flat::flat_image::FlatImage)
///
#[repr(transparent)]
pub struct FlatChannelF16(pub(crate) *mut sys::Imf_FlatHalfChannel_t);

unsafe impl OpaquePtr for FlatChannelF16 {
    type SysPointee = sys::Imf_FlatHalfChannel_t;
    type Pointee = FlatChannelF16;
}

pub type FlatChannelF16Ref<'a, P = FlatChannelF16> = Ref<'a, P>;
pub type FlatChannelF16RefMut<'a, P = FlatChannelF16> = RefMut<'a, P>;

/// Holds the pixel data for a single float channel in one level in a
/// [`FlatImage`](crate::flat::flat_image::FlatImage)
///
#[repr(transparent)]
pub struct FlatChannelF32(pub(crate) *mut sys::Imf_FlatFloatChannel_t);

unsafe impl OpaquePtr for FlatChannelF32 {
    type SysPointee = sys::Imf_FlatFloatChannel_t;
    type Pointee = FlatChannelF32;
}

pub type FlatChannelF32Ref<'a, P = FlatChannelF32> = Ref<'a, P>;
pub type FlatChannelF32RefMut<'a, P = FlatChannelF32> = RefMut<'a, P>;

/// Holds the pixel data for a single unsigned int channel in one level in a
/// [`FlatImage`](crate::flat::flat_image::FlatImage)
///
#[repr(transparent)]
pub struct FlatChannelU32(pub(crate) *mut sys::Imf_FlatUIntChannel_t);

unsafe impl OpaquePtr for FlatChannelU32 {
    type SysPointee = sys::Imf_FlatUIntChannel_t;
    type Pointee = FlatChannelU32;
}

pub type FlatChannelU32Ref<'a, P = FlatChannelU32> = Ref<'a, P>;
pub type FlatChannelU32RefMut<'a, P = FlatChannelU32> = RefMut<'a, P>;

impl FlatChannelF16 {
    /// Get the [`Channel`] description for this channel
    ///
    pub fn channel(&self) -> Channel {
        let mut c = CHANNEL_HALF;
        unsafe {
            sys::Imf_FlatHalfChannel_channel(self.0, &mut c);
        }
        c
    }

    /// Get the sampling rate in the `x` direction
    ///
    pub fn x_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatHalfChannel_xSampling(self.0, &mut v);
            v
        }
    }

    /// Get the sampling rate in the `y` direction
    ///
    pub fn y_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatHalfChannel_ySampling(self.0, &mut v);
            v
        }
    }

    /// Is this channel perceptually linear?
    ///
    pub fn p_linear(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_FlatHalfChannel_pLinear(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each row of this channel
    ///
    pub fn pixels_per_row(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatHalfChannel_pixelsPerRow(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each column of this channel
    ///
    pub fn pixels_per_column(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatHalfChannel_pixelsPerColumn(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there total in this channel
    ///
    pub fn num_pixels(&self) -> usize {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatHalfChannel_numPixels(self.0, &mut v);
            v as usize
        }
    }

    /// Construct a [`Slice`] for this Channel
    ///
    pub fn slice(&self) -> Slice {
        let mut s = sys::Imf_Slice_t::default();
        unsafe {
            sys::Imf_FlatHalfChannel_slice(self.0, &mut s);
        }
        Slice(s)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn flat_level(&self) -> FlatImageLevelRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatHalfChannel_flatLevel_const(self.0, &mut ptr);
        }
        FlatImageLevelRef::new(ptr)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn flat_level_mut(&mut self) -> FlatImageLevelRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FlatHalfChannel_flatLevel(self.0, &mut ptr);
        }
        FlatImageLevelRefMut::new(ptr)
    }

    /// Get a reference to the pixel at `[x, y]`
    ///
    pub fn get(&self, x: i32, y: i32) -> Option<&f16> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatHalfChannel_at_const(self.0, &mut ptr, x, y)
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
            sys::Imf_FlatHalfChannel_at(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &mut *(ptr as *mut f16))
        }
    }

    /// Get row `r` of the channel as a slice
    ///
    pub fn row(&self, r: i32) -> Option<&[f16]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null();
            unsafe {
                sys::Imf_FlatHalfChannel_row_const(self.0, &mut ptr, r);
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
                sys::Imf_FlatHalfChannel_row(self.0, &mut ptr, r);
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
}

impl FlatChannelF32 {
    /// Get the [`Channel`] description for this channel
    ///
    pub fn channel(&self) -> Channel {
        let mut c = CHANNEL_HALF;
        unsafe {
            sys::Imf_FlatFloatChannel_channel(self.0, &mut c);
        }
        c
    }

    /// Get the sampling rate in the `x` direction
    ///
    pub fn x_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatFloatChannel_xSampling(self.0, &mut v);
            v
        }
    }

    /// Get the sampling rate in the `y` direction
    ///
    pub fn y_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatFloatChannel_ySampling(self.0, &mut v);
            v
        }
    }

    /// Is this channel perceptually linear?
    ///
    pub fn p_linear(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_FlatFloatChannel_pLinear(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each row of this channel
    ///
    pub fn pixels_per_row(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatFloatChannel_pixelsPerRow(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each column of this channel
    ///
    pub fn pixels_per_column(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatFloatChannel_pixelsPerColumn(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there total in this channel
    ///
    pub fn num_pixels(&self) -> usize {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatFloatChannel_numPixels(self.0, &mut v);
            v as usize
        }
    }

    /// Construct a [`Slice`] for this Channel
    ///
    pub fn slice(&self) -> Slice {
        let mut s = sys::Imf_Slice_t::default();
        unsafe {
            sys::Imf_FlatFloatChannel_slice(self.0, &mut s);
        }
        Slice(s)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn flat_level(&self) -> FlatImageLevelRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatFloatChannel_flatLevel_const(self.0, &mut ptr);
        }
        FlatImageLevelRef::new(ptr)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn flat_level_mut(&mut self) -> FlatImageLevelRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FlatFloatChannel_flatLevel(self.0, &mut ptr);
        }
        FlatImageLevelRefMut::new(ptr)
    }

    /// Get a reference to the pixel at `[x, y]`
    ///
    pub fn get(&self, x: i32, y: i32) -> Option<&f32> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatFloatChannel_at_const(self.0, &mut ptr, x, y)
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
            sys::Imf_FlatFloatChannel_at(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &mut *(ptr as *mut f32))
        }
    }

    /// Get row `r` of the channel as a slice
    ///
    pub fn row(&self, r: i32) -> Option<&[f32]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null();
            unsafe {
                sys::Imf_FlatFloatChannel_row_const(self.0, &mut ptr, r);
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
                sys::Imf_FlatFloatChannel_row(self.0, &mut ptr, r);
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
}

impl FlatChannelU32 {
    /// Get the [`Channel`] description for this channel
    ///
    pub fn channel(&self) -> Channel {
        let mut c = CHANNEL_HALF;
        unsafe {
            sys::Imf_FlatUIntChannel_channel(self.0, &mut c);
        }
        c
    }

    /// Get the sampling rate in the `x` direction
    ///
    pub fn x_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatUIntChannel_xSampling(self.0, &mut v);
            v
        }
    }

    /// Get the sampling rate in the `y` direction
    ///
    pub fn y_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatUIntChannel_ySampling(self.0, &mut v);
            v
        }
    }

    /// Is this channel perceptually linear?
    ///
    pub fn p_linear(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_FlatUIntChannel_pLinear(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each row of this channel
    ///
    pub fn pixels_per_row(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatUIntChannel_pixelsPerRow(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each column of this channel
    ///
    pub fn pixels_per_column(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatUIntChannel_pixelsPerColumn(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there total in this channel
    ///
    pub fn num_pixels(&self) -> usize {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatUIntChannel_numPixels(self.0, &mut v);
            v as usize
        }
    }

    /// Construct a [`Slice`] for this Channel
    ///
    pub fn slice(&self) -> Slice {
        let mut s = sys::Imf_Slice_t::default();
        unsafe {
            sys::Imf_FlatUIntChannel_slice(self.0, &mut s);
        }
        Slice(s)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn flat_level(&self) -> FlatImageLevelRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatUIntChannel_flatLevel_const(self.0, &mut ptr);
        }
        FlatImageLevelRef::new(ptr)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn flat_level_mut(&mut self) -> FlatImageLevelRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FlatUIntChannel_flatLevel(self.0, &mut ptr);
        }
        FlatImageLevelRefMut::new(ptr)
    }

    /// Get a reference to the pixel at `[x, y]`
    ///
    pub fn get(&self, x: i32, y: i32) -> Option<&u32> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatUIntChannel_at_const(self.0, &mut ptr, x, y)
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
            sys::Imf_FlatUIntChannel_at(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &mut *(ptr as *mut u32))
        }
    }

    /// Get row `r` of the channel as a slice
    ///
    pub fn row(&self, r: i32) -> Option<&[u32]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null();
            unsafe {
                sys::Imf_FlatUIntChannel_row_const(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts(
                        ptr as *const u32,
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
                sys::Imf_FlatUIntChannel_row(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts_mut(
                        ptr as *mut u32,
                        self.pixels_per_row() as usize,
                    ))
                }
            }
        }
    }
}
