use crate::{
    core::{
        cppstd::CppString,
        error::Error,
        refptr::{OpaquePtr, Ref, RefMut},
    },
    flat::{
        flat_image::{FlatImageRef, FlatImageRefMut},
        flat_image_channel::{
            FlatChannelF16Ref, FlatChannelF16RefMut, FlatChannelF32Ref,
            FlatChannelF32RefMut, FlatChannelU32Ref, FlatChannelU32RefMut,
        },
    },
};

use imath_traits::Bound2;
use openexr_sys as sys;

type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(transparent)]
pub struct FlatImageLevel(pub(crate) *mut sys::Imf_FlatImageLevel_t);

unsafe impl OpaquePtr for FlatImageLevel {
    type SysPointee = sys::Imf_FlatImageLevel_t;
    type Pointee = FlatImageLevel;
}

pub type FlatImageLevelRef<'a, P = FlatImageLevel> = Ref<'a, P>;
pub type FlatImageLevelRefMut<'a, P = FlatImageLevel> = RefMut<'a, P>;

impl FlatImageLevel {
    /// Get the level number of this level in the `x` direction
    ///
    pub fn x_level_number(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatImageLevel_xLevelNumber(self.0, &mut v);
        }
        v
    }

    /// Get the level number of this level in the `y` direction
    ///
    pub fn y_level_number(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_FlatImageLevel_yLevelNumber(self.0, &mut v);
        }
        v
    }

    /// Get the data window for this levels
    ///
    pub fn data_window<B: Bound2<i32>>(&self) -> &B {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatImageLevel_dataWindow(self.0, &mut ptr);
            &*(ptr as *const B)
        }
    }

    /// Get the [`FlatImage`](crate::flat::flat_image::FlatImage) to which this level belongs   
    ///
    pub fn flat_image(&self) -> FlatImageRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FlatImageLevel_flatImage_const(self.0, &mut ptr);
            FlatImageRef::new(ptr)
        }
    }

    /// Get a mutable reference to the [`FlatImage`](crate::flat::flat_image::FlatImage) to which this level belongs   
    ///
    pub fn flat_image_mut(&mut self) -> FlatImageRefMut {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FlatImageLevel_flatImage(self.0, &mut ptr);
            FlatImageRefMut::new(ptr)
        }
    }

    /// Find a [`FlatChannelF16`](crate::flat::flat_image_channel::FlatChannelF16) with the given name
    ///
    pub fn find_channel_f16(&self, name: &str) -> Option<FlatChannelF16Ref> {
        let mut ptr = std::ptr::null();
        unsafe {
            let s = CppString::new(name);

            sys::Imf_FlatImageLevel_findTypedChannel_half_const(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(FlatChannelF16Ref::new(ptr))
            }
        }
    }

    /// Find a [`FlatChannelF16`](crate::flat::flat_image_channel::FlatChannelF16) with the given name and get a mutable reference
    /// to it
    ///
    pub fn find_channel_f16_mut(
        &self,
        name: &str,
    ) -> Option<FlatChannelF16RefMut> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let s = CppString::new(name);

            sys::Imf_FlatImageLevel_findTypedChannel_half(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(FlatChannelF16RefMut::new(ptr))
            }
        }
    }

    /// Find a [`FlatChannelF32`](crate::flat::flat_image_channel::FlatChannelF32) with the given name
    ///
    pub fn find_channel_f32(&self, name: &str) -> Option<FlatChannelF32Ref> {
        let mut ptr = std::ptr::null();
        unsafe {
            let s = CppString::new(name);
            sys::Imf_FlatImageLevel_findTypedChannel_float_const(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(FlatChannelF32Ref::new(ptr))
            }
        }
    }

    /// Find a [`FlatChannelF32`](crate::flat::flat_image_channel::FlatChannelF32) with the given name and get a mutable reference
    /// to it
    ///
    pub fn find_channel_f32_mut(
        &self,
        name: &str,
    ) -> Option<FlatChannelF32RefMut> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let s = CppString::new(name);
            sys::Imf_FlatImageLevel_findTypedChannel_float(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(FlatChannelF32RefMut::new(ptr))
            }
        }
    }

    /// Find a [`FlatChannelU32`](crate::flat::flat_image_channel::FlatChannelU32) with the given name
    ///
    pub fn find_channel_u32(&self, name: &str) -> Option<FlatChannelU32Ref> {
        let mut ptr = std::ptr::null();
        unsafe {
            let s = CppString::new(name);
            sys::Imf_FlatImageLevel_findTypedChannel_uint_const(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(FlatChannelU32Ref::new(ptr))
            }
        }
    }

    /// Find a [`FlatChannelU32`](crate::flat::flat_image_channel::FlatChannelU32) with the given name and get a mutable reference
    /// to it
    ///
    pub fn find_channel_u32_mut(
        &self,
        name: &str,
    ) -> Option<FlatChannelU32RefMut> {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            let s = CppString::new(name);

            sys::Imf_FlatImageLevel_findTypedChannel_uint(
                self.0, &mut ptr, s.0,
            );

            if ptr.is_null() {
                None
            } else {
                Some(FlatChannelU32RefMut::new(ptr))
            }
        }
    }
}
