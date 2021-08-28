use openexr_sys as sys;

pub use crate::core::{
    error::Error,
    refptr::{OpaquePtr, Ref, RefMut},
    PixelType,
};
use std::marker::PhantomData;

use std::ffi::{CStr, CString};

use imath_traits::{Bound2, Vec2, Zero};

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct FrameBuffer {
    pub(crate) ptr: *mut sys::Imf_FrameBuffer_t,
    pub(crate) frames: Option<Vec<Frame>>,
}

unsafe impl OpaquePtr for FrameBuffer {
    type SysPointee = sys::Imf_FrameBuffer_t;
    type Pointee = FrameBuffer;
}

pub type FrameBufferRef<'a> = Ref<'a, FrameBuffer>;

impl FrameBuffer {
    /// Create a new `FrameBuffer`.
    pub fn new() -> FrameBuffer {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FrameBuffer_ctor(&mut ptr);
        }
        FrameBuffer {
            ptr,
            frames: Some(Vec::new()),
        }
    }

    /// Insert a [`Slice`] into the `FrameBuffer`.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if name is the empty string
    ///
    pub fn insert(&mut self, name: &str, slice: &Slice) -> Result<()> {
        let c_name =
            CString::new(name).expect("Internal null bytes in filename");

        unsafe {
            sys::Imf_FrameBuffer_insert(self.ptr, c_name.as_ptr(), &slice.0)
                .into_result()?;
        }

        Ok(())
    }

    /// Find the [`Slice`] with the given `name` in the `FrameBuffer`
    ///
    /// # Returns
    /// * `Some([`SliceRef`])` - if the [`Slice`] is found
    /// * `None` - otherwise
    ///
    pub fn get_slice<'a>(&'a self, name: &str) -> Option<SliceRef<'a>> {
        let c_name =
            CString::new(name).expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_FrameBuffer_findSlice_const(
                self.ptr,
                &mut ptr,
                c_name.as_ptr(),
            );
        }

        if ptr.is_null() {
            None
        } else {
            Some(SliceRef::new(ptr))
        }
    }

    /// Find the [`Slice`] with the given `name` in the `FrameBuffer` and get a
    /// mutable Ref to it
    ///
    /// # Returns
    /// * `Some([`SliceRefMut`])` - if the [`Slice`] is found
    /// * `None` - otherwise
    ///
    pub fn get_slice_mut<'a>(
        &'a mut self,
        name: &str,
    ) -> Option<SliceRefMut<'a>> {
        let c_name =
            CString::new(name).expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_FrameBuffer_findSlice(self.ptr, &mut ptr, c_name.as_ptr());
        }

        if ptr.is_null() {
            None
        } else {
            Some(SliceRefMut::new(ptr))
        }
    }

    /// Get an iterator over the [`Slice`]s in this `FrameBuffer`
    ///
    pub fn iter(&self) -> FrameBufferIter {
        unsafe {
            let mut ptr = sys::Imf_FrameBuffer_ConstIterator_t::default();
            sys::Imf_FrameBuffer_begin_const(self.ptr, &mut ptr)
                .into_result()
                .unwrap();
            let ptr = FrameBufferConstIterator(ptr);

            let mut end = sys::Imf_FrameBuffer_ConstIterator_t::default();
            sys::Imf_FrameBuffer_end_const(self.ptr, &mut end)
                .into_result()
                .unwrap();
            let end = FrameBufferConstIterator(end);

            FrameBufferIter {
                ptr,
                end,
                _p: PhantomData,
            }
        }
    }

    pub fn insert_frame(&mut self, frame: Frame) -> Result<FrameHandle> {
        let mut ptr = frame.ptr;
        let w = frame.data_window[2] - frame.data_window[0] + 1;
        let ystride = w as usize * frame.stride;
        for chan in &frame.channel_names {
            self.insert(
                &chan,
                &Slice::with_data_window(
                    frame.channel_type,
                    ptr,
                    frame.data_window,
                )
                .x_stride(frame.stride)
                .y_stride(ystride)
                .build()?,
            )?;

            ptr = unsafe { ptr.add(frame.channel_stride) };
        }

        let handle = match &mut self.frames {
            Some(v) => {
                let handle = v.len();
                v.push(frame);
                FrameHandle(handle)
            }
            _ => unreachable!(),
        };

        Ok(handle)
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_FrameBuffer_dtor(self.ptr);
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        FrameBuffer::new()
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub(crate) struct FrameBufferConstIterator(
    pub(crate) sys::Imf_FrameBuffer_ConstIterator_t,
);

// #[repr(transparent)]
// pub(crate) struct FrameBufferIterator(
//     pub(crate) sys::Imf_FrameBuffer_Iterator_t,
// );

pub struct FrameBufferIter<'a> {
    ptr: FrameBufferConstIterator,
    end: FrameBufferConstIterator,
    _p: PhantomData<&'a FrameBuffer>,
}

impl<'a> Iterator for FrameBufferIter<'a> {
    type Item = (&'a str, SliceRef<'a>);

    fn next(&mut self) -> Option<(&'a str, SliceRef<'a>)> {
        let ptr_curr = self.ptr.clone();
        let mut ptr_next = self.ptr.clone();
        unsafe {
            let mut dummy = std::ptr::null_mut();
            sys::Imf_FrameBuffer_ConstIterator__op_inc(
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
                sys::Imf_FrameBuffer_ConstIterator_name(
                    &ptr_curr.0,
                    &mut nameptr,
                )
                .into_result()
                .unwrap();

                if nameptr.is_null() {
                    panic!("FrameBuffer::ConstIterator::name() returned NULL");
                }

                let mut sliceptr = std::ptr::null();
                sys::Imf_FrameBuffer_ConstIterator_slice(
                    &ptr_curr.0,
                    &mut sliceptr,
                )
                .into_result()
                .unwrap();

                Some((
                    CStr::from_ptr(nameptr)
                        .to_str()
                        .expect("NUL bytes in channel name"),
                    SliceRef::new(sliceptr),
                ))
            }
        }
    }
}

impl PartialEq for FrameBufferConstIterator {
    fn eq(&self, rhs: &FrameBufferConstIterator) -> bool {
        unsafe {
            let mut result = false;
            sys::Imf_frame_buffer_const_iter_eq(&mut result, &self.0, &rhs.0)
                .into_result()
                .unwrap();

            result
        }
    }
}

#[repr(transparent)]
pub struct Slice(pub(crate) sys::Imf_Slice_t);
pub type SliceRef<'a, P = Slice> = Ref<'a, P>;
pub type SliceRefMut<'a, P = Slice> = RefMut<'a, P>;

unsafe impl OpaquePtr for Slice {
    type SysPointee = sys::Imf_Slice_t;
    type Pointee = Slice;
}

pub struct SliceBuilder {
    pixel_type: PixelType,
    data: *const u8,
    origin: [i32; 2],
    w: i64,
    h: i64,
    x_stride: usize,
    y_stride: usize,
    x_sampling: i32,
    y_sampling: i32,
    fill_value: f64,
    x_tile_coords: bool,
    y_tile_coords: bool,
}

impl SliceBuilder {
    pub fn build(self) -> Result<Slice> {
        let mut slice = sys::Imf_Slice_t::default();
        unsafe {
            sys::Imf_Slice_with_origin(
                &mut slice,
                self.pixel_type.into(),
                self.data as *const std::os::raw::c_void,
                self.origin.as_ptr() as *const sys::Imath_V2i_t,
                self.w,
                self.h,
                self.x_stride,
                self.y_stride,
                self.x_sampling,
                self.y_sampling,
                self.fill_value,
                self.x_tile_coords,
                self.y_tile_coords,
            )
            .into_result()?;
        }

        Ok(Slice(slice))
    }

    /// Set the data window origin.
    ///
    pub fn origin<V>(mut self, o: V) -> Self
    where
        V: Vec2<i32>,
    {
        let o = o.as_slice();
        self.origin = *o;
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

impl Slice {
    pub fn builder(
        pixel_type: PixelType,
        data: *const u8,
        w: i64,
        h: i64,
    ) -> SliceBuilder {
        SliceBuilder {
            pixel_type,
            data,
            origin: [0i32; 2],
            w,
            h,
            x_stride: 0,
            y_stride: 0,
            x_sampling: 1,
            y_sampling: 1,
            fill_value: 0.0,
            x_tile_coords: false,
            y_tile_coords: false,
        }
    }

    pub fn with_origin(
        pixel_type: PixelType,
        data: *const u8,
        origin: [i32; 2],
        w: i64,
        h: i64,
    ) -> SliceBuilder {
        SliceBuilder {
            pixel_type,
            data,
            origin,
            w,
            h,
            x_stride: 0,
            y_stride: 0,
            x_sampling: 1,
            y_sampling: 1,
            fill_value: 0.0,
            x_tile_coords: false,
            y_tile_coords: false,
        }
    }

    pub fn with_data_window<B>(
        pixel_type: PixelType,
        data: *const u8,
        data_window: B,
    ) -> SliceBuilder
    where
        B: Bound2<i32>,
    {
        let b = data_window.as_slice();

        SliceBuilder {
            pixel_type,
            data,
            origin: [b[0], b[1]],
            w: b[2] as i64 + 1,
            h: b[3] as i64 + 1,
            x_stride: 0,
            y_stride: 0,
            x_sampling: 1,
            y_sampling: 1,
            fill_value: 0.0,
            x_tile_coords: false,
            y_tile_coords: false,
        }
    }
}

impl Drop for Slice {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_Slice_dtor(&mut self.0);
        }
    }
}

pub trait Pixel: Zero + Clone {
    const CHANNEL_TYPE: PixelType;
    const NUM_CHANNELS: usize;
    const STRIDE: usize = std::mem::size_of::<Self>();
    const CHANNEL_STRIDE: usize = std::mem::size_of::<Self>();
}

impl Pixel for half::f16 {
    // type Type = Self;
    const CHANNEL_TYPE: PixelType = PixelType::Half;
    const NUM_CHANNELS: usize = 1;
}

impl Pixel for f32 {
    // type Type = Self;
    const CHANNEL_TYPE: PixelType = PixelType::Float;
    const NUM_CHANNELS: usize = 1;
}

impl Pixel for u32 {
    // type Type = Self;
    const CHANNEL_TYPE: PixelType = PixelType::Uint;
    const NUM_CHANNELS: usize = 1;
}

impl Pixel for crate::rgba::rgba::Rgba {
    // type Type = Self;
    const CHANNEL_TYPE: PixelType = PixelType::Half;
    const NUM_CHANNELS: usize = 4;
    const CHANNEL_STRIDE: usize = std::mem::size_of::<half::f16>();
}

#[cfg(feature = "imath_cgmath")]
mod impl_cgmath {
    use super::{Pixel, PixelType};

    impl Pixel for cgmath::Vector2<half::f16> {
        const CHANNEL_TYPE: PixelType = half::f16::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 2;
        const CHANNEL_STRIDE: usize = half::f16::CHANNEL_STRIDE;
    }

    impl Pixel for cgmath::Vector2<f32> {
        const CHANNEL_TYPE: PixelType = f32::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 2;
        const CHANNEL_STRIDE: usize = f32::CHANNEL_STRIDE;
    }

    impl Pixel for cgmath::Vector2<u32> {
        const CHANNEL_TYPE: PixelType = u32::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 2;
        const CHANNEL_STRIDE: usize = u32::CHANNEL_STRIDE;
    }

    impl Pixel for cgmath::Vector3<half::f16> {
        const CHANNEL_TYPE: PixelType = half::f16::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 3;
        const CHANNEL_STRIDE: usize = half::f16::CHANNEL_STRIDE;
    }

    impl Pixel for cgmath::Vector3<f32> {
        const CHANNEL_TYPE: PixelType = f32::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 3;
        const CHANNEL_STRIDE: usize = f32::CHANNEL_STRIDE;
    }

    impl Pixel for cgmath::Vector3<u32> {
        const CHANNEL_TYPE: PixelType = u32::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 3;
        const CHANNEL_STRIDE: usize = u32::CHANNEL_STRIDE;
    }

    impl Pixel for cgmath::Vector4<half::f16> {
        const CHANNEL_TYPE: PixelType = half::f16::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 4;
        const CHANNEL_STRIDE: usize = half::f16::CHANNEL_STRIDE;
    }

    impl Pixel for cgmath::Vector4<f32> {
        const CHANNEL_TYPE: PixelType = f32::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 4;
        const CHANNEL_STRIDE: usize = f32::CHANNEL_STRIDE;
    }

    impl Pixel for cgmath::Vector4<u32> {
        const CHANNEL_TYPE: PixelType = u32::CHANNEL_TYPE;
        const NUM_CHANNELS: usize = 4;
        const CHANNEL_STRIDE: usize = u32::CHANNEL_STRIDE;
    }
}

/// `Frame` attempts to provide a safer API on top of OpenEXR's
/// [`Slice`](crate::core::frame_buffer::Slice) type.
///
/// Instead of providing a pointer and calculating offsets based on the data
/// window offset, as [`Slice`](crate::core::frame_buffer::Slice) does, `Frame` wraps up the data window offset
/// and handles memory allocation internally so that you can't get it wrong.
///
/// # Examples
/// ```no_run
/// # fn foo() -> Result<(), openexr::Error> {
/// use openexr::prelude::*;
///
/// let file = InputFile::new("test.exr", 4)?;
/// let data_window: [i32; 4] = *file.header().data_window();
///
/// let frame_rgba =
///     Frame::new::<Rgba, _, _>(
///         &["R", "G", "B", "A"],
///         data_window
///         )?;
///
/// let (file, frames) = file
///     .into_reader(vec![frame_rgba])?
///     .read_pixels(data_window[1], data_window[3])?;
///
/// # Ok(())
/// # }
/// ```
pub struct Frame {
    pub(crate) channel_type: PixelType,
    pub(crate) data_window: [i32; 4],
    pub(crate) channel_names: Vec<String>,
    pub(crate) stride: usize,
    pub(crate) channel_stride: usize,
    pub(crate) ptr: *mut u8,
    pub(crate) byte_len: usize,
    pub(crate) align: usize,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Hash)]
pub struct FrameHandle(usize);

use std::alloc::{GlobalAlloc, Layout, System};
impl Frame {
    /// Constructs a new frame for the given `channel_names` and `data_window`
    /// and allocates storage to hold the pixel data.
    ///
    /// # Errors
    /// [`Error::InvalidArgument`] - if the length of the `channel_names` slice
    /// is not a multiple of `Pixel::NUM_CHANNELS`
    ///
    pub fn new<T: Pixel, B: Bound2<i32>, S: AsRef<str>>(
        channel_names: &[S],
        data_window: B,
    ) -> Result<Frame> {
        let channel_names = channel_names
            .iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<String>>();

        if channel_names.len() % T::NUM_CHANNELS != 0 {
            return Err(Error::InvalidArgument(format!("channel_names ({:?}) has {} channels but pixel type has {} channels. Channel names length must be a multiple of pixel type num channels in order to pack.", channel_names, channel_names.len(), T::NUM_CHANNELS)));
        }

        let data_window = *data_window.as_slice();
        let w = data_window[2] - data_window[0] + 1;
        let h = data_window[3] - data_window[1] + 1;

        // We're packing the data into an array where elements may have multiple
        // channels themselves
        let num_packed = channel_names.len() / T::NUM_CHANNELS;
        let len = (w * h) as usize * num_packed;
        let byte_len = len * std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let ptr = unsafe { System.alloc(Layout::array::<T>(len).unwrap()) };

        Ok(Frame {
            channel_type: T::CHANNEL_TYPE,
            data_window,
            channel_names,
            stride: T::STRIDE * num_packed,
            channel_stride: T::CHANNEL_STRIDE,
            ptr,
            byte_len,
            align,
        })
    }

    /// Construct a new `Frame` using the storage provided in `vec`.
    ///
    /// This can either be used to provide pixel data for writing, or to re-use
    /// storage between reads.
    ///
    /// # Errors
    /// [`Error::InvalidArgument`] - if the length of the `channel_names` slice
    /// is not a multiple of `Pixel::NUM_CHANNELS`
    ///
    pub fn with_vec<T: Pixel, B: Bound2<i32>, S: AsRef<str>>(
        channel_names: &[S],
        mut vec: Vec<T>,
        data_window: B,
    ) -> Result<Frame> {
        let channel_names = channel_names
            .iter()
            .map(|s| s.as_ref().to_string())
            .collect::<Vec<String>>();

        if channel_names.len() != T::NUM_CHANNELS {
            return Err(Error::InvalidArgument(format!("channel_names ({:?}) has {} channels but pixel type has {} channels.", channel_names, channel_names.len(), T::NUM_CHANNELS)));
        }

        let data_window = *data_window.as_slice();
        let w = data_window[2] - data_window[0] + 1;
        let h = data_window[3] - data_window[1] + 1;
        let len = (w * h) as usize;

        vec.resize(len, T::zero());
        let mut vec = vec.into_boxed_slice();
        let ptr = vec.as_mut_ptr() as *mut u8;
        Box::leak(vec);

        let byte_len = len * std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();

        Ok(Frame {
            channel_type: T::CHANNEL_TYPE,
            data_window,
            channel_names,
            stride: T::STRIDE,
            channel_stride: T::CHANNEL_STRIDE,
            ptr,
            byte_len,
            align,
        })
    }

    /// Get a reference to the pixel data slice.
    ///
    pub fn as_slice<T: Pixel>(&self) -> &[T] {
        let stride = std::mem::size_of::<T>();
        if T::STRIDE != stride {
            panic!("Attempt to get a slice with a different type");
        }

        unsafe {
            std::slice::from_raw_parts(
                self.ptr as *const T,
                self.byte_len / stride,
            )
        }
    }

    /// Get a mutable reference to the pixel data slice.
    ///
    pub fn as_mut_slice<T: Pixel>(&mut self) -> &mut [T] {
        let stride = std::mem::size_of::<T>();
        if T::STRIDE != stride {
            panic!("Attempt to get a slice with a different type");
        }

        unsafe {
            std::slice::from_raw_parts_mut(
                self.ptr as *mut T,
                self.byte_len / stride,
            )
        }
    }

    /// Consume this `Frame` and return the pixel data as a `Vec`.
    ///
    pub fn into_vec<T: Pixel>(self) -> Vec<T> {
        self.as_slice().to_vec()
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            System.dealloc(
                self.ptr,
                Layout::from_size_align(self.byte_len, self.align).unwrap(),
            )
        }
    }
}
