use openexr_sys as sys;

pub use crate::core::{
    error::Error,
    frame_buffer::{Frame, Slice, SliceRef},
    refptr::{OpaquePtr, Ref, RefMut},
    PixelType,
};
pub use imath_traits::{Bound2, Vec2};
use std::marker::PhantomData;

use std::ffi::{CStr, CString};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct DeepFrameBuffer {
    pub(crate) ptr: *mut sys::Imf_DeepFrameBuffer_t,
    pub(crate) sample_count_frame: Option<Frame>,
}

unsafe impl OpaquePtr for DeepFrameBuffer {
    type SysPointee = sys::Imf_DeepFrameBuffer_t;
    type Pointee = DeepFrameBuffer;
}

pub type DeepFrameBufferRef<'a> = Ref<'a, DeepFrameBuffer>;

impl DeepFrameBuffer {
    /// Create a new `DeepFrameBuffer`.
    pub fn new() -> DeepFrameBuffer {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepFrameBuffer_ctor(&mut ptr);
        }
        DeepFrameBuffer {
            ptr,
            sample_count_frame: None,
        }
    }

    /// Insert a [`DeepSlice`] into the `DeepFrameBuffer`.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if name is the empty string
    ///
    pub fn insert(&mut self, name: &str, slice: &DeepSlice) -> Result<()> {
        let c_name =
            CString::new(name).expect("Internal null bytes in filename");

        unsafe {
            sys::Imf_DeepFrameBuffer_insert(
                self.ptr,
                c_name.as_ptr(),
                &slice.0,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Find the [`DeepSlice`] with the given `name` in the `DeepFrameBuffer`
    ///
    /// # Returns
    /// * `Some([`DeepSliceRef`])` - if the [`DeepSlice`] is found
    /// * `None` - otherwise
    ///
    pub fn get_slice<'a>(&'a self, name: &str) -> Option<DeepSliceRef<'a>> {
        let c_name =
            CString::new(name).expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepFrameBuffer_findSlice_const(
                self.ptr,
                &mut ptr,
                c_name.as_ptr(),
            );
        }

        if ptr.is_null() {
            None
        } else {
            Some(DeepSliceRef::new(ptr))
        }
    }

    /// Find the [`DeepSlice`] with the given `name` in the `DeepFrameBuffer` and get a
    /// mutable Ref to it
    ///
    /// # Returns
    /// * `Some([`DeepSliceRefMut`])` - if the [`DeepSlice`] is found
    /// * `None` - otherwise
    ///
    pub fn get_slice_mut<'a>(
        &'a mut self,
        name: &str,
    ) -> Option<DeepSliceRefMut<'a>> {
        let c_name =
            CString::new(name).expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepFrameBuffer_findSlice(
                self.ptr,
                &mut ptr,
                c_name.as_ptr(),
            );
        }

        if ptr.is_null() {
            None
        } else {
            Some(DeepSliceRefMut::new(ptr))
        }
    }

    /// Get an iterator over the [`DeepSlice`]s in this `DeepFrameBuffer`
    ///
    pub fn iter(&self) -> DeepFrameBufferIter {
        unsafe {
            let mut ptr = sys::Imf_DeepFrameBuffer_ConstIterator_t::default();
            sys::Imf_DeepFrameBuffer_begin_const(self.ptr, &mut ptr)
                .into_result()
                .unwrap();
            let ptr = DeepFrameBufferConstIterator(ptr);

            let mut end = sys::Imf_DeepFrameBuffer_ConstIterator_t::default();
            sys::Imf_DeepFrameBuffer_end_const(self.ptr, &mut end)
                .into_result()
                .unwrap();
            let end = DeepFrameBufferConstIterator(end);

            DeepFrameBufferIter {
                ptr,
                end,
                _p: PhantomData,
            }
        }
    }

    /// Insert a [`Slice`] to hold the sample count for each deep pixel.
    ///
    /// The slice must be of type [`PixelType::Uint`]
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if the type of the slice is not [`PixelType::Uint`]
    ///
    pub fn set_sample_count_slice(
        &mut self,
        sample_count_slice: &Slice,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepFrameBuffer_insertSampleCountSlice(
                self.ptr,
                &sample_count_slice.0,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Set a [`Frame`](crate::core::frame_buffer::Frame) to hold the per-pixel sample
    /// counts
    ///
    pub fn set_sample_count_frame(&mut self, frame: Frame) -> Result<()> {
        let w = frame.data_window[2] - frame.data_window[0] + 1;
        let ystride = w as usize * frame.stride;
        self.set_sample_count_slice(
            &Slice::with_data_window(
                frame.channel_type,
                frame.ptr,
                frame.data_window,
            )
            .x_stride(frame.stride)
            .y_stride(ystride)
            .build()?,
        )?;

        self.sample_count_frame = Some(frame);

        Ok(())
    }

    pub fn sample_count_frame(&self) -> Option<&Frame> {
        self.sample_count_frame.as_ref()
    }

    /// Get the sample count slice
    ///
    pub fn sample_count_slice(&self) -> SliceRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepFrameBuffer_getSampleCountSlice(self.ptr, &mut ptr);
        }

        SliceRef::new(ptr)
    }
}

impl Drop for DeepFrameBuffer {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_DeepFrameBuffer_dtor(self.ptr);
        }
    }
}

impl Default for DeepFrameBuffer {
    fn default() -> Self {
        DeepFrameBuffer::new()
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct DeepFrameBufferConstIterator(
    pub(crate) sys::Imf_DeepFrameBuffer_ConstIterator_t,
);

// #[repr(transparent)]
// pub(crate) struct DeepFrameBufferIterator(
//     pub(crate) sys::Imf_DeepFrameBuffer_Iterator_t,
// );

pub struct DeepFrameBufferIter<'a> {
    ptr: DeepFrameBufferConstIterator,
    end: DeepFrameBufferConstIterator,
    _p: PhantomData<&'a DeepFrameBuffer>,
}

impl<'a> Iterator for DeepFrameBufferIter<'a> {
    type Item = (&'a str, DeepSliceRef<'a>);

    fn next(&mut self) -> Option<(&'a str, DeepSliceRef<'a>)> {
        let ptr_curr = self.ptr.clone();
        let mut ptr_next = self.ptr.clone();
        unsafe {
            let mut dummy = std::ptr::null_mut();
            sys::Imf_DeepFrameBuffer_ConstIterator__op_inc(
                &mut ptr_next.0,
                &mut dummy,
            )
            .into_result()
            .unwrap();
        }

        if ptr_curr == self.end {
            None
        } else {
            self.ptr = ptr_next;
            unsafe {
                let mut nameptr = std::ptr::null();
                sys::Imf_DeepFrameBuffer_ConstIterator_name(
                    &ptr_curr.0,
                    &mut nameptr,
                )
                .into_result()
                .unwrap();

                if nameptr.is_null() {
                    panic!(
                        "DeepFrameBuffer::ConstIterator::name() returned NULL"
                    );
                }

                let mut sliceptr = std::ptr::null();
                sys::Imf_DeepFrameBuffer_ConstIterator_slice(
                    &ptr_curr.0,
                    &mut sliceptr,
                )
                .into_result()
                .unwrap();

                Some((
                    CStr::from_ptr(nameptr)
                        .to_str()
                        .expect("NUL bytes in channel name"),
                    DeepSliceRef::new(sliceptr),
                ))
            }
        }
    }
}

impl PartialEq for DeepFrameBufferConstIterator {
    fn eq(&self, rhs: &DeepFrameBufferConstIterator) -> bool {
        unsafe {
            let mut result = false;
            sys::Imf_deep_frame_buffer_const_iter_eq(
                &mut result,
                &self.0,
                &rhs.0,
            )
            .into_result()
            .unwrap();

            result
        }
    }
}

#[repr(transparent)]
pub struct DeepSlice(pub(crate) sys::Imf_DeepSlice_t);
pub type DeepSliceRef<'a, P = DeepSlice> = Ref<'a, P>;
pub type DeepSliceRefMut<'a, P = DeepSlice> = RefMut<'a, P>;

unsafe impl OpaquePtr for DeepSlice {
    type SysPointee = sys::Imf_DeepSlice_t;
    type Pointee = DeepSlice;
}

pub struct DeepSliceBuilder {
    pixel_type: PixelType,
    data: *mut i8,
    x_stride: usize,
    y_stride: usize,
    sample_stride: usize,
    x_sampling: i32,
    y_sampling: i32,
    fill_value: f64,
    x_tile_coords: bool,
    y_tile_coords: bool,
}

impl DeepSliceBuilder {
    pub fn build(self) -> Result<DeepSlice> {
        let mut slice = sys::Imf_DeepSlice_t::default();
        unsafe {
            sys::Imf_DeepSlice_ctor(
                &mut slice,
                self.pixel_type.into(),
                self.data,
                self.x_stride,
                self.y_stride,
                self.sample_stride,
                self.x_sampling,
                self.y_sampling,
                self.fill_value,
                self.x_tile_coords,
                self.y_tile_coords,
            )
            .into_result()?;
        }

        Ok(DeepSlice(slice))
    }

    pub fn sample_stride(mut self, v: usize) -> Self {
        self.sample_stride = v;
        self
    }

    pub fn fill_value(mut self, v: f64) -> Self {
        self.fill_value = v;
        self
    }

    pub fn x_stride(mut self, x: usize) -> Self {
        self.x_stride = x;
        self
    }

    pub fn y_stride(mut self, y: usize) -> Self {
        self.y_stride = y;
        self
    }

    pub fn x_sampling(mut self, x: i32) -> Self {
        self.x_sampling = x;
        self
    }

    pub fn y_sampling(mut self, y: i32) -> Self {
        self.y_sampling = y;
        self
    }

    pub fn x_tile_coords(mut self, x: bool) -> Self {
        self.x_tile_coords = x;
        self
    }

    pub fn y_tile_coords(mut self, y: bool) -> Self {
        self.y_tile_coords = y;
        self
    }
}

impl DeepSlice {
    pub fn builder(pixel_type: PixelType, data: *mut i8) -> DeepSliceBuilder {
        DeepSliceBuilder {
            pixel_type,
            data,
            x_stride: std::mem::size_of::<*mut i8>(),
            y_stride: 0,
            sample_stride: 0,
            x_sampling: 1,
            y_sampling: 1,
            fill_value: 0.0,
            x_tile_coords: false,
            y_tile_coords: false,
        }
    }

    pub fn from_sample_ptr<S: DeepSample>(
        data: *mut *mut S,
        width: i32,
    ) -> DeepSliceBuilder {
        DeepSliceBuilder {
            pixel_type: S::CHANNEL_TYPE,
            data: data as *mut i8,
            x_stride: std::mem::size_of::<*mut i8>(),
            y_stride: std::mem::size_of::<*mut i8>() * width as usize,
            sample_stride: std::mem::size_of::<S>(),
            x_sampling: 1,
            y_sampling: 1,
            fill_value: 0.0,
            x_tile_coords: false,
            y_tile_coords: false,
        }
    }
}

impl Drop for DeepSlice {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_DeepSlice_dtor(&mut self.0);
        }
    }
}

pub trait DeepSample {
    type Type;
    const CHANNEL_TYPE: PixelType;
    const STRIDE: usize = std::mem::size_of::<Self::Type>();
}

impl DeepSample for half::f16 {
    type Type = Self;
    const CHANNEL_TYPE: PixelType = PixelType::Half;
}

impl DeepSample for f32 {
    type Type = Self;
    const CHANNEL_TYPE: PixelType = PixelType::Float;
}

impl DeepSample for u32 {
    type Type = Self;
    const CHANNEL_TYPE: PixelType = PixelType::Uint;
}
