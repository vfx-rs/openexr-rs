use openexr_sys as sys;

use std::ffi::CString;
use std::path::Path;

use crate::core::{
    error::Error,
    frame_buffer::{Frame, FrameBuffer, FrameBufferRef},
    header::HeaderRef,
};

type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(transparent)]
pub struct InputFile(pub(crate) *mut sys::Imf_InputFile_t);

impl InputFile {
    /// Open the file at path `filename` and read the header.
    ///
    /// # Errors
    /// * [`Error::Base`] - if the file cannot be opened
    ///
    pub fn new<P: AsRef<Path>>(
        filename: P,
        num_threads: i32,
    ) -> Result<InputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_InputFile_ctor(&mut ptr, c_filename.as_ptr(), num_threads)
                .into_result()?;
        }

        Ok(InputFile(ptr))
    }

    /// Access to the file [`Header`](crate::core::header::Header)
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_InputFile_header(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_InputFile_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Access to the file format version
    ///
    pub fn version(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_InputFile_version(self.0, &mut v);
        }
        v
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the InputFile object.
    ///
    /// The current frame buffer is the destination for the pixel
    /// data read from the file.  The current frame buffer must be
    /// set at least once before `read_pixels()` is called.
    /// The current frame buffer can be changed after each call
    /// to `read_pixels()`.
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
            sys::Imf_InputFile_setFrameBuffer(self.0, frame_buffer.ptr)
                .into_result()?;
        }

        Ok(())
    }

    /// Access to the current frame buffer
    ///
    pub fn frame_buffer(&self) -> FrameBufferRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_InputFile_frameBuffer(self.0, &mut ptr);
        }

        FrameBufferRef::new(ptr)
    }

    /// Check if all pixels in the data window are present in the input file
    ///
    pub fn is_complete(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_InputFile_isComplete(self.0, &mut v);
        }

        v
    }

    /// Check if SSE optimization is enabled
    ///
    /// Call after `set_frame_buffer()` to query whether optimized file decoding
    /// is available - decode times will be faster if returns true
    ///
    /// Optimization depends on:
    ///   * The file type (only scanline data is supported),
    ///   * The framebuffer channels (RGB/RGBA mono or stereo)
    ///   * The framebuffer channel types (all channels half-float format only)
    ///   * The file channels (RGB/RGBA mono or stereo)
    ///   * The file channel types (all channel half-float format only)
    ///   * Whether SSE2 instruction support was detected at compile time
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if no frame buffer has been set
    ///
    pub fn is_optimization_enabled(&self) -> Result<bool> {
        let mut v = false;
        unsafe {
            sys::Imf_InputFile_isOptimizationEnabled(self.0, &mut v)
                .into_result()?;
        }

        Ok(v)
    }

    /// Read all scanlines in the range [s1, s2] and put them in the current
    /// frame buffer.
    ///
    /// `read_pixel_sample_counts()` must be called before calling this method.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if no frame buffer has been set, if `s1`
    /// or `s2` are outside the data window, or if the sample counts have not been
    /// read yet
    /// * [`Error::Base`] - if any other error occurs
    ///
    pub fn read_pixels(&mut self, s1: i32, s2: i32) -> Result<()> {
        unsafe {
            sys::Imf_InputFile_readPixels(self.0, s1, s2).into_result()?;
        }
        Ok(())
    }

    /// Consume this `InputFile` and convert it to an [`InputFileReader`].
    ///
    /// The [`InputFileReader`] provides a safe API for reading data from the
    /// file into memory, by taking ownership of the memory and handling calculating
    /// [`Slice`](crate::core::frame_buffer::Slice) offsets internally.
    ///
    /// `frames` is a `Vec` of [`Frame`](crate::core::frame_buffer::Frame) objects, which
    /// describe the channels to load from the image, and how they are to be
    /// stored in memory.
    ///
    pub fn into_reader(
        mut self,
        frames: Vec<Frame>,
    ) -> Result<InputFileReader> {
        let mut frame_buffer = FrameBuffer::new();

        for frame in frames {
            frame_buffer.insert_frame(frame)?;
        }

        self.set_frame_buffer(&frame_buffer)?;

        Ok(InputFileReader {
            inner: self.0,
            frame_buffer,
        })
    }
}

/// `InputFileReader` provides a safe API over `InputFile` by taking ownership
/// of the storage into which the channel data is to be read and handling all
/// the [`Slice`](crate::core::frame_buffer::Slice) pointer offset shenanigans internally.
///
pub struct InputFileReader {
    inner: *mut sys::Imf_InputFile_t,
    frame_buffer: FrameBuffer,
}

impl InputFileReader {
    /// Read all scanlines in the range [s1, s2] and put them in the current
    /// frame buffer.
    ///
    /// This consumes the `InputFileReader` and returns the original `InputFile`
    /// object and the `Vec` of the read [`Frame`](crate::core::frame_buffer::Frame)s .
    ///
    /// # Errors
    /// * [`Error::Base`] - if any error occurs
    ///
    pub fn read_pixels(
        self,
        s1: i32,
        s2: i32,
    ) -> Result<(InputFile, Vec<Frame>)> {
        unsafe {
            sys::Imf_InputFile_readPixels(self.inner, s1, s2).into_result()?;
        }

        let InputFileReader {
            inner: _,
            mut frame_buffer,
        } = self;

        Ok((InputFile(self.inner), frame_buffer.frames.take().unwrap()))
    }

    /// Check if SSE optimization is enabled. Decode times will be faster if
    /// this returns true.
    ///
    /// Optimization depends on:
    ///   * The file type (only scanline data is supported),
    ///   * The framebuffer channels (RGB/RGBA mono or stereo)
    ///   * The framebuffer channel types (all channels half-float format only)
    ///   * The file channels (RGB/RGBA mono or stereo)
    ///   * The file channel types (all channel half-float format only)
    ///   * Whether SSE2 instruction support was detected at compile time
    ///
    pub fn is_optimization_enabled(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_InputFile_isOptimizationEnabled(self.inner, &mut v);
        }

        v
    }
}

#[cfg(test)]
#[test]
fn read_input_safe1() -> Result<()> {
    use crate::{core::frame_buffer::Frame, rgba::rgba::Rgba};
    use std::path::PathBuf;

    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("window.exr");

    // Open the `InputFile` and read the header
    let file = InputFile::new(&path, 4).unwrap();

    // Get the data window from the header so we know how much storage we need
    // to allocate
    let data_window: [i32; 4] = *file.header().data_window();

    // Create a `Frame` to hold the channels we want to read from the file.
    // `Frame` handles:
    // * Allocating the required memory (or it can use existing storage with the
    // `with_vec[_multi]()` constructors).
    // * Creating slices with the correct offsets into the storage for the
    // requested channels. Note that asking for ["R"] or ["R", "G", "B"] here
    // would be a runtime error as the `Pixel` trait we've implemented for `Rgba`
    // tells the constructor to expect 4 channels, but we could ask for ["R", "G", "B", "A",
    // "diff.R", "diff.G", diff.B", "Z"] and it would pack those 8 channels into
    // 2 `Rgba`s per pixel.
    let frame_rgba =
        Frame::new::<Rgba, _, _>(&["R", "G", "B", "A"], data_window)?;

    // Consume the `InputFile` into an `InputFileReader` by giving it the list
    // of `Frames` to read into. This internally creates the `FrameBuffer` and
    // gives it to the `InputFile` rather than having those as separate operations
    // which create lifetime dependencies between all the moving parts.
    // `InputFileReader` can have a safe `read_pixels()` method because it owns
    // all the data associated with the read operation.
    // `read_pixels()` consumes `InputFileReader` and returns an `InputFile` and
    // the completed `Frame`s.
    let (_file, mut frames) = file
        .into_reader(vec![frame_rgba])?
        .read_pixels(data_window[1], data_window[3])?;

    // Consume the `Frame` into a Vec (or we could just get a reference
    // to the data if we wanted)
    let pixels: Vec<Rgba> = frames.remove(0).into_vec();

    for _rgba in pixels.iter() {
        // ... do something with pixels
    }

    Ok(())
}

#[cfg(all(test, feature = "impl_cgmath"))]
#[test]
fn read_input_safe_cgmath1() -> Result<()> {
    use crate::{Frame, Rgba};
    use std::path::PathBuf;

    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("window.exr");

    // Use the Box2i implementation of Bound2 which we define as
    //     struct Box2<T> {
    //         min: cgmath::Vector2<T>,
    //         max: cgmath::Vector2<T>,
    //     }
    // when the "impl_cgmath" feature is enabled (which transitively enables the\
    // "cgmath" feature in imath-traits.
    use cgmath::Vector2;
    use imath_traits::impl_cgmath::Box2i;

    // Open the `InputFile` and read the header
    let file = InputFile::new(&path, 4).unwrap();

    // Get the data window from the header so we know how much storage we need
    // to allocate
    let data_window: Box2i = *file.header().data_window();

    // Create a `Frame` to hold the channels we want to read from the file.
    // `Frame` handles:
    // * Allocating the required memory (or it can use existing storage with the
    // `with_vec[_multi]()` constructors).
    // * Creating slices with the correct offsets into the storage for the
    // requested channels. Note that asking for ["R"] or ["R", "G", "B"] here
    // would be a runtime error as the `Pixel` trait we've implemented for `Rgba`
    // tells the constructor to expect 4 channels.
    let frame_rgba =
        Frame::new::<Vector2<f32>, _, _>(&["R", "G", "B", "A"], data_window)?;

    // Consume the `InputFile` into an `InputFileReader` by giving it the list
    // of `Frames` to read into. This internally creates the `FrameBuffer` and
    // gives it to the `InputFile` rather than having those as separate operations
    // which create lifetime dependencies between all the moving parts.
    // `InputFileReader` can have a safe `read_pixels()` method because it owns
    // all the data associated with the read operation.
    // `read_pixels()` consumes `InputFileReader` and returns an `InputFile` and
    // the completed `Frame`s.
    let (_file, mut frames) = file
        .into_reader(vec![frame_rgba])?
        .read_pixels(data_window.min.y, data_window.max.y)?;

    // Consume the `Frame` into a Vec (or we could just get a reference
    // to the data if we wanted)
    let pixels: Vec<Rgba> = frames.remove(0).into_vec();

    for _rgba in pixels.iter() {
        // ... do something with pixels
    }

    Ok(())
}
