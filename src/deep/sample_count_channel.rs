use crate::{
    core::{
        channel_list::{Channel, CHANNEL_HALF},
        error::Error,
        frame_buffer::Slice,
        refptr::{OpaquePtr, Ref, RefMut},
    },
    deep::deep_image_level::{DeepImageLevelRef, DeepImageLevelRefMut},
};
use openexr_sys as sys;

type Result<T, E = Error> = std::result::Result<T, E>;

/// Holds the per-pixel number of samples in a deep image.
///
#[repr(transparent)]
pub struct SampleCountChannel(pub(crate) *mut sys::Imf_SampleCountChannel_t);

unsafe impl OpaquePtr for SampleCountChannel {
    type SysPointee = sys::Imf_SampleCountChannel_t;
    type Pointee = SampleCountChannel;
}

pub type SampleCountChannelRef<'a, P = SampleCountChannel> = Ref<'a, P>;
pub type SampleCountChannelRefMut<'a, P = SampleCountChannel> = RefMut<'a, P>;

impl SampleCountChannel {
    /// Get the [`Channel`] description for this channel
    ///
    pub fn channel(&self) -> Channel {
        let mut c = CHANNEL_HALF;
        unsafe {
            sys::Imf_SampleCountChannel_channel(self.0, &mut c);
        }
        c
    }

    /// Get the sampling rate in the `x` direction
    ///
    pub fn x_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_SampleCountChannel_xSampling(self.0, &mut v);
            v
        }
    }

    /// Get the sampling rate in the `y` direction
    ///
    pub fn y_sampling(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_SampleCountChannel_ySampling(self.0, &mut v);
            v
        }
    }

    /// Is this channel perceptually linear?
    ///
    pub fn p_linear(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_SampleCountChannel_pLinear(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each row of this channel
    ///
    pub fn pixels_per_row(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_SampleCountChannel_pixelsPerRow(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there in each column of this channel
    ///
    pub fn pixels_per_column(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_SampleCountChannel_pixelsPerColumn(self.0, &mut v);
            v
        }
    }

    /// How many pixels are there total in this channel
    ///
    pub fn num_pixels(&self) -> usize {
        let mut v = 0;
        unsafe {
            sys::Imf_SampleCountChannel_numPixels(self.0, &mut v);
            v as usize
        }
    }

    /// Construct a [`Slice`] for this Channel
    ///
    pub fn slice(&self) -> Slice {
        let mut s = sys::Imf_Slice_t::default();
        unsafe {
            sys::Imf_SampleCountChannel_slice(self.0, &mut s);
        }
        Slice(s)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn deep_level(&self) -> DeepImageLevelRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_SampleCountChannel_deepLevel_const(self.0, &mut ptr);
        }
        DeepImageLevelRef::new(ptr)
    }

    /// Get the level which this channel is a part of
    ///
    pub fn deep_level_mut(&mut self) -> DeepImageLevelRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_SampleCountChannel_deepLevel(self.0, &mut ptr);
        }
        DeepImageLevelRefMut::new(ptr)
    }

    /// Get a reference to the pixel at `[x, y]`
    ///
    pub fn get(&self, x: i32, y: i32) -> Option<&i32> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_SampleCountChannel_at(self.0, &mut ptr, x, y)
                .into_result()
                .ok()
                .map(|_| &*(ptr as *const i32))
        }
    }

    /// Get row `r` of the channel as a slice
    ///
    pub fn row(&self, r: i32) -> Option<&[i32]> {
        if r >= self.pixels_per_column() {
            None
        } else {
            let mut ptr = std::ptr::null();
            unsafe {
                sys::Imf_SampleCountChannel_row(self.0, &mut ptr, r);
                if ptr.is_null() {
                    None
                } else {
                    Some(std::slice::from_raw_parts(
                        ptr as *const i32,
                        self.pixels_per_row() as usize,
                    ))
                }
            }
        }
    }
}
