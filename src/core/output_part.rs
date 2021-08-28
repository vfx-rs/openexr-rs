use openexr_sys as sys;

use crate::{
    core::{
        error::Error,
        frame_buffer::{FrameBuffer, FrameBufferRef},
        header::HeaderRef,
        input_file::InputFile,
        input_part::InputPart,
        preview_image::PreviewRgba,
    },
    multi_part::multi_part_output_file::MultiPartOutputFile,
};

type Result<T, E = Error> = std::result::Result<T, E>;

use std::marker::PhantomData;

#[repr(transparent)]
pub struct OutputPart<'a> {
    pub(crate) inner: sys::Imf_OutputPart_t,
    phantom: std::marker::PhantomData<&'a MultiPartOutputFile>,
}

impl<'a> OutputPart<'a> {
    /// Get an interface to the part `part_number` of the [`MultiPartOutputFile`]
    /// `multi_part_file`.
    ///
    pub fn new(
        multi_part_file: &MultiPartOutputFile,
        part_number: i32,
    ) -> Result<OutputPart> {
        let mut part = sys::Imf_OutputPart_t::default();
        unsafe {
            sys::Imf_OutputPart_ctor(&mut part, multi_part_file.0, part_number)
                .into_result()?;
        }

        Ok(OutputPart {
            inner: part,
            phantom: PhantomData,
        })
    }

    /// Get the filename this file is writing to.
    ///
    pub fn file_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_OutputPart_fileName(&self.inner, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }

    /// Access to the file [`Header`](crate::core::header::Header)
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_OutputPart_header(&self.inner, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_OutputPart_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the OutputPart object.
    ///
    /// The current frame buffer is the source of the pixel
    /// data written to the file.  The current frame buffer
    /// must be set at least once before [`OutputPart::write_pixels`] is
    /// called.  The current frame buffer can be changed
    /// after each call to [`OutputPart::write_pixels`].
    ///
    /// ## Errors
    /// * [`Error::InvalidArgument`] - If the pixel type of the
    /// [`Channel`](crate::core::channel_list::Channel)s in the [`Header`](crate::core::header::Header)
    /// do not match the types in the frame buffer, or if the sampling rates do
    /// not match.
    ///
    pub fn set_frame_buffer(
        &mut self,
        frame_buffer: &FrameBuffer,
    ) -> Result<()> {
        unsafe {
            sys::Imf_OutputPart_setFrameBuffer(
                &mut self.inner,
                frame_buffer.ptr,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Get a reference to the frame buffer.
    ///
    pub fn frame_buffer(&self) -> FrameBufferRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_OutputPart_frameBuffer(&self.inner, &mut ptr);
            if ptr.is_null() {
                panic!(
                    "Received null ptr from sys::Imf_OutputPart_frameBuffer"
                );
            }

            FrameBufferRef::new(ptr)
        }
    }

    /// Write pixel data.
    ///
    /// Retrieves the next `num_scan_lines` worth of data from
    /// the current frame buffer, starting with the scan line indicated by
    /// [`OutputPart::current_scan_line`] and stores the data in the output file, and
    /// progressing in the direction indicated by `header().line_order()`.
    ///
    /// To produce a complete and correct file, exactly m scan lines must
    /// be written, where m is equal to
    /// `header().data_window().max.y - header().data_window().min.y + 1`.
    ///
    /// # Errors
    /// * [`Error::Io`] - If an I/O error occurs while writing the image data
    /// * [`Error::InvalidArgument`] - If `current_scan_line() + num_scan_lines`
    /// is outside the data window.
    /// * [`Error::Base`] - If any other error occurs.
    ///
    /// # Safety
    /// This method is wildly unsafe as on the C++ side it's reading from
    /// pointers offset from the base pointers supplied by the
    /// [`crate::core::frame_buffer::Slice`] in
    /// the [`FrameBuffer`]. In particular, by setting an
    /// [`crate::core::frame_buffer::SliceBuilder::origin`] you can cause `write_pixels`
    /// to read from arbitrary memory locations.
    ///
    pub unsafe fn write_pixels(&mut self, num_scan_lines: i32) -> Result<()> {
        sys::Imf_OutputPart_writePixels(&mut self.inner, num_scan_lines)
            .into_result()?;
        Ok(())
    }

    /// Access to the current scan line:
    ///
    /// Returns the y coordinate of the first scan line
    /// that will be read from the current frame buffer during the next
    /// call to [`OutputPart::write_pixels`].
    ///
    /// If `line_order() == INCREASING_Y`:
    ///
    /// The current scan line before the first call to write_pixels()
    /// is header().data_window().min.y.  After writing each scan line,
    /// the current scan line is incremented by 1.
    ///
    /// If `line_order() == DECREASING_Y`:
    ///
    /// The current scan line before the first call to write_pixels()
    /// is header().data_window().max.y.  After writing each scan line,
    /// the current scan line is decremented by 1.
    ///
    pub fn current_scan_line(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_OutputPart_currentScanLine(&self.inner, &mut v);
        }
        v
    }

    /// Shortcut to copy all pixels from an [`InputFile`] into this file,
    /// without uncompressing and then recompressing the pixel data.
    ///
    /// This file's header must be compatible with the [`InputFile`]'s
    /// header:  The two header's "dataWindow", "compression",
    /// "lineOrder" and "channels" attributes must be the same.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If the headers do not match
    /// * [`Error::LogicError`] - If scan lines have already been written to this file.
    ///
    pub fn copy_pixels_from_file(&mut self, file: &InputFile) -> Result<()> {
        unsafe {
            sys::Imf_OutputPart_copyPixels_from_file(&mut self.inner, file.0)
                .into_result()?;
        }
        Ok(())
    }

    /// Shortcut to copy all pixels from an [`InputPart`] into this file,
    /// without uncompressing and then recompressing the pixel data.
    ///
    /// This file's header must be compatible with the [`InputPart`]'s
    /// header:  The two header's "dataWindow", "compression",
    /// "lineOrder" and "channels" attributes must be the same.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If the headers do not match
    /// * [`Error::LogicError`] - If scan lines have already been written to this file.
    ///
    pub fn copy_pixels_from_part(
        &mut self,
        file: &mut InputPart,
    ) -> Result<()> {
        unsafe {
            sys::Imf_OutputPart_copyPixels_from_part(
                &mut self.inner,
                &mut file.inner,
            )
            .into_result()?;
        }
        Ok(())
    }

    /// Updating the preview image:
    ///
    /// Supplies a new set of pixels for the
    /// preview image attribute in the file's header.
    ///
    /// This is necessary because images are
    /// often stored in a file incrementally, a few scan lines at a
    /// time, while the image is being generated.  Since the preview
    /// image is an attribute in the file's header, it gets stored in
    /// the file as soon as the file is opened, but we may not know
    /// what the preview image should look like until we have written
    /// the last scan line of the main image.
    ///
    /// # Errors
    /// * [`Error::LogicError`] - If the header does not contain a preview image
    /// * [`Error::Base`] - If any other error occurs
    ///
    pub fn update_preview_image(
        &mut self,
        new_pixels: &[PreviewRgba],
    ) -> Result<()> {
        unsafe {
            sys::Imf_OutputPart_updatePreviewImage(
                &mut self.inner,
                new_pixels.as_ptr() as *const sys::Imf_PreviewRgba_t,
            )
            .into_result()?;
        }

        Ok(())
    }
}
