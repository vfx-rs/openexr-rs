use openexr_sys as sys;

use crate::{
    core::{error::Error, header::HeaderRef},
    deep::deep_frame_buffer::{DeepFrameBuffer, DeepFrameBufferRef},
    multi_part::multi_part_input_file::MultiPartInputFile,
};

type Result<T, E = Error> = std::result::Result<T, E>;

use std::marker::PhantomData;

#[repr(transparent)]
pub struct DeepScanLineInputPart<'a> {
    pub(crate) inner: sys::Imf_DeepScanLineInputPart_t,
    phantom: std::marker::PhantomData<&'a MultiPartInputFile>,
}

impl<'a> DeepScanLineInputPart<'a> {
    /// Get an interface to the part `part_number` of the [`MultiPartInputFile`]
    /// `multi_part_file`.
    ///
    pub fn new(
        multi_part_file: &MultiPartInputFile,
        part_number: i32,
    ) -> Result<DeepScanLineInputPart> {
        let mut part = sys::Imf_DeepScanLineInputPart_t::default();
        unsafe {
            sys::Imf_DeepScanLineInputPart_ctor(
                &mut part,
                multi_part_file.0,
                part_number,
            )
            .into_result()?;
        }

        Ok(DeepScanLineInputPart {
            inner: part,
            phantom: PhantomData,
        })
    }

    /// Access to the file [`Header`](crate::core::header::Header)
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepScanLineInputPart_header(&self.inner, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_DeepScanLineInputPart_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Access to the file format version
    ///
    pub fn version(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepScanLineInputPart_version(&self.inner, &mut v);
        }
        v
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the DeepScanLineInputPart object.
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
        frame_buffer: &DeepFrameBuffer,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepScanLineInputPart_setFrameBuffer(
                &mut self.inner,
                frame_buffer.ptr,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Access to the current frame buffer
    ///
    pub fn frame_buffer(&self) -> DeepFrameBufferRef {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_DeepScanLineInputPart_frameBuffer(&self.inner, &mut ptr);
        }

        DeepFrameBufferRef::new(ptr)
    }

    /// Check if all pixels in the data window are present in the input file
    ///
    pub fn is_complete(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepScanLineInputPart_isComplete(&self.inner, &mut v);
        }

        v
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
            sys::Imf_DeepScanLineInputPart_readPixels(&mut self.inner, s1, s2)
                .into_result()?;
        }
        Ok(())
    }

    /// Read the sample counts for each pixel and place them in the sample
    /// count slice in the frame buffer.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if `s1` or `s2` are outside the data
    /// window.
    ///
    pub fn read_pixel_sample_counts(&mut self, s1: i32, s2: i32) -> Result<()> {
        unsafe {
            sys::Imf_DeepScanLineInputPart_readPixelSampleCounts(
                &mut self.inner,
                s1,
                s2,
            )
            .into_result()?;
        }

        Ok(())
    }

    /*
    pub fn into_reader(
        mut self,
        frames: Vec<DeepFrame>,
    ) -> Result<DeepScanLineInputPartReader> {
        // we need to know the union of all the frames passed in to know which
        // pixels to read sample counts for
        let mut data_window =
            [std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN];
        for f in &frames {
            data_window[0] = data_window[0].min(f.data_window[0]);
            data_window[1] = data_window[1].min(f.data_window[1]);
            data_window[2] = data_window[2].max(f.data_window[2]);
            data_window[3] = data_window[3].max(f.data_window[3]);
        }

        // create a Frame for the sample counts, which we'll need to
        // allocate storage for the DeepFrames
        let sample_count_frame =
            Frame::new::<u32, _, _>(&["sampleCounts"], data_window).unwrap();

        let mut frame_buffer = DeepFrameBuffer::new();

        // insert the sample count and all user-provided frames
        frame_buffer
            .set_sample_count_frame(sample_count_frame)
            .unwrap();

        for f in frames {
            frame_buffer.insert_deep_frame(f)?;
        }

        // read the sample counts and get a slice to them
        self.read_pixel_sample_counts(data_window[0], data_window[2])?;

        let DeepFrameBuffer {
            ref sample_count_frame,
            ref mut frames,
            ..
        } = frame_buffer;
        let counts = sample_count_frame.as_ref().unwrap().as_slice();

        // Now allocate sample storage for each frame's pixels
        let mut i = 0;
        for y in data_window[1]..=data_window[3] {
            for x in data_window[0]..=data_window[2] {
                let v = frames.as_mut().unwrap();
                for f in v {
                    let count = counts[i];
                    unsafe { f.allocate_pixel_storage(x, y, count) };
                }
                i += 1;
            }
        }

        Ok(DeepScanLineInputPartReader {
            inner: self.inner,
            frame_buffer,
        })
    }
    */
}
