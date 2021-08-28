use crate::{
    core::{
        cppstd::CppString,
        error::Error,
        refptr::{OpaquePtr, Ref, RefMut},
    },
    deep::{
        deep_image::{DeepImageRef, DeepImageRefMut},
        deep_image_channel::{
            DeepChannelF16Ref, DeepChannelF16RefMut, DeepChannelF32Ref,
            DeepChannelF32RefMut, DeepChannelU32Ref, DeepChannelU32RefMut,
        },
    },
};

use imath_traits::Bound2;
use openexr_sys as sys;

type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(transparent)]
pub struct DeepImageLevel(pub(crate) *mut sys::Imf_DeepImageLevel_t);

unsafe impl OpaquePtr for DeepImageLevel {
    type SysPointee = sys::Imf_DeepImageLevel_t;
    type Pointee = DeepImageLevel;
}

pub type DeepImageLevelRef<'a, P = DeepImageLevel> = Ref<'a, P>;
pub type DeepImageLevelRefMut<'a, P = DeepImageLevel> = RefMut<'a, P>;

impl DeepImageLevel {
    /// Get the level number of this level in the `x` direction
    ///
    pub fn x_level_number(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepImageLevel_xLevelNumber(self.0, &mut v);
        }
        v
    }

    /// Get the level number of this level in the `y` direction
    ///
    pub fn y_level_number(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepImageLevel_yLevelNumber(self.0, &mut v);
        }
        v
    }

    /// Get the data window for this levels
    ///
    pub fn data_window<B: Bound2<i32>>(&self) -> &B {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepImageLevel_dataWindow(self.0, &mut ptr);
            &*(ptr as *const B)
        }
    }

    /// Get the [`DeepImage`](crate::deep::deep_image::DeepImage) to which this level belongs   
    ///
    pub fn deep_image(&self) -> DeepImageRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepImageLevel_deepImage_const(self.0, &mut ptr);
            DeepImageRef::new(ptr)
        }
    }

    /// Get a mutable reference to the [`DeepImage`](crate::deep::deep_image::DeepImage) to which this level belongs   
    ///
    pub fn deep_image_mut(&mut self) -> DeepImageRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepImageLevel_deepImage(self.0, &mut ptr);
            DeepImageRefMut::new(ptr)
        }
    }

    /// Find a [`DeepChannelF16`](crate::deep::deep_image_channel::DeepChannelF16) with the given name
    ///
    pub fn find_channel_f16(&self, name: &str) -> Option<DeepChannelF16Ref> {
        let mut ptr = std::ptr::null();
        unsafe {
            let s = CppString::new(name);

            sys::Imf_DeepImageLevel_findTypedChannel_half_const(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(DeepChannelF16Ref::new(ptr))
            }
        }
    }

    /// Find a [`DeepChannelF16`](crate::deep::deep_image_channel::DeepChannelF16) with the given name and get a mutable reference
    /// to it
    ///
    pub fn find_channel_f16_mut(
        &self,
        name: &str,
    ) -> Option<DeepChannelF16RefMut> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let s = CppString::new(name);

            sys::Imf_DeepImageLevel_findTypedChannel_half(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(DeepChannelF16RefMut::new(ptr))
            }
        }
    }

    /// Find a [`DeepChannelF32`](crate::deep::deep_image_channel::DeepChannelF32) with the given name
    ///
    pub fn find_channel_f32(&self, name: &str) -> Option<DeepChannelF32Ref> {
        let mut ptr = std::ptr::null();
        unsafe {
            let s = CppString::new(name);
            sys::Imf_DeepImageLevel_findTypedChannel_float_const(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(DeepChannelF32Ref::new(ptr))
            }
        }
    }

    /// Find a [`DeepChannelF32`](crate::deep::deep_image_channel::DeepChannelF32) with the given name and get a mutable reference
    /// to it
    ///
    pub fn find_channel_f32_mut(
        &self,
        name: &str,
    ) -> Option<DeepChannelF32RefMut> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let s = CppString::new(name);
            sys::Imf_DeepImageLevel_findTypedChannel_float(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(DeepChannelF32RefMut::new(ptr))
            }
        }
    }

    /// Find a [`DeepChannelU32`](crate::deep::deep_image_channel::DeepChannelU32) with the given name
    ///
    pub fn find_channel_u32(&self, name: &str) -> Option<DeepChannelU32Ref> {
        let mut ptr = std::ptr::null();
        unsafe {
            let s = CppString::new(name);
            sys::Imf_DeepImageLevel_findTypedChannel_uint_const(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(DeepChannelU32Ref::new(ptr))
            }
        }
    }

    /// Find a [`DeepChannelU32`](crate::deep::deep_image_channel::DeepChannelU32) with the given name and get a mutable reference
    /// to it
    ///
    pub fn find_channel_u32_mut(
        &self,
        name: &str,
    ) -> Option<DeepChannelU32RefMut> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let s = CppString::new(name);

            sys::Imf_DeepImageLevel_findTypedChannel_uint(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(DeepChannelU32RefMut::new(ptr))
            }
        }
    }
}
