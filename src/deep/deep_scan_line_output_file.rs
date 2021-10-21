use openexr_sys as sys;

use std::ffi::CString;
use std::path::Path;

use crate::{
    core::{
        error::Error,
        header::{Header, HeaderRef},
        preview_image::PreviewRgba,
    },
    deep::{
        deep_frame_buffer::{DeepFrameBuffer, DeepFrameBufferRef},
        deep_scan_line_input_file::DeepScanLineInputFile,
        deep_scan_line_input_part::DeepScanLineInputPart,
    },
};

type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(transparent)]
pub struct DeepScanLineOutputFile(
    pub(crate) *mut sys::Imf_DeepScanLineOutputFile_t,
);

impl DeepScanLineOutputFile {
    /// Opens the file and writes the file header.
    ///
    /// The file header is also copied into the DeepScanLineOutputFile object,
    /// and can later be accessed via the `header()` method.
    /// Dropping this DeepScanLineOutputFile object automatically closes
    /// the file.
    ///
    /// # Errors
    /// * [`Error::Base`] - If the file could not be opened or the [`Header`]
    /// is invalid
    ///
    pub fn new<P: AsRef<Path>>(
        filename: P,
        header: &Header,
        num_threads: i32,
    ) -> Result<DeepScanLineOutputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut _inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepScanLineOutputFile_ctor(
                &mut _inner,
                c_filename.as_ptr(),
                header.0.as_ref(),
                num_threads,
            )
            .into_result()?;
        }

        Ok(DeepScanLineOutputFile(_inner))
    }

    /// Get the filename this file is writing to.
    ///
    pub fn file_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepScanLineOutputFile_fileName(self.0, &mut ptr)
                .into_result()
                .unwrap();
            std::ffi::CStr::from_ptr(ptr)
                .to_str()
                .expect("Invalid UTF-8")
        }
    }

    /// Access to the file [`Header`]
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepScanLineOutputFile_header(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_DeepScanLineOutputFile_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the OutputFile object.
    ///
    /// The current frame buffer is the source of the pixel
    /// data written to the file.  The current frame buffer
    /// must be set at least once before [`write_pixels()`](DeepScanLineOutputFile::write_pixels) is
    /// called.  The current frame buffer can be changed
    /// after each call to [`write_pixels()`](DeepScanLineOutputFile::write_pixels).
    ///
    /// ## Errors
    /// * [`Error::InvalidArgument`] - If the pixel type of the
    /// [`Channel`](crate::core::channel_list::Channel)s in the [`Header`]
    /// do not match the types in the frame buffer, or if the sampling rates do
    /// not match.
    ///
    pub fn set_frame_buffer(
        &mut self,
        frame_buffer: &DeepFrameBuffer,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepScanLineOutputFile_setFrameBuffer(
                self.0,
                frame_buffer.ptr,
            )
            .into_result()?;
        }

        Ok(())
    }

    /// Get a reference to the frame buffer.
    ///
    pub fn frame_buffer(&self) -> DeepFrameBufferRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepScanLineOutputFile_frameBuffer(self.0, &mut ptr);
            if ptr.is_null() {
                panic!(
                    "Received null ptr from sys::Imf_DeepScanLineOutputFile_frameBuffer"
                );
            }

            DeepFrameBufferRef::new(ptr)
        }
    }

    /// Write pixel data.
    ///
    /// Retrieves the next `num_scan_lines` worth of data from
    /// the current frame buffer, starting with the scan line indicated by
    /// [`current_scan_line()`](DeepScanLineOutputFile::current_scan_line) and stores the data in the output file, and
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
    /// [`DeepSlice`](crate::deep::deep_frame_buffer::DeepSlice) in
    /// the [`DeepFrameBuffer`]. You must ensure the the [`DeepFrameBuffer`] attached to this file by
    /// [`set_frame_buffer()`](DeepScanLineInputFile::set_frame_buffer) has valid slices
    /// for the channels to be written.
    ///
    ///
    pub unsafe fn write_pixels(&self, num_scan_lines: i32) -> Result<()> {
        sys::Imf_DeepScanLineOutputFile_writePixels(self.0, num_scan_lines)
            .into_result()?;
        Ok(())
    }

    /// Access to the current scan line:
    ///
    /// Returns the y coordinate of the first scan line
    /// that will be read from the current frame buffer during the next
    /// call to [`write_pixels()`](DeepScanLineOutputFile::write_pixels).
    ///
    /// If `line_order() == LineOrder::IncreasingY`:
    ///
    /// The current scan line before the first call to write_pixels()
    /// is header().data_window().min.y.  After writing each scan line,
    /// the current scan line is incremented by 1.
    ///
    /// If `line_order() == LineOrder::DecreasingY`:
    ///
    /// The current scan line before the first call to
    /// [`write_pixels()`](DeepScanLineOutputFile::write_pixels)
    /// is header().data_window().max.y.  After writing each scan line,
    /// the current scan line is decremented by 1.
    ///
    pub fn current_scan_line(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepScanLineOutputFile_currentScanLine(self.0, &mut v);
        }
        v
    }

    /// Shortcut to copy all pixels from a [`DeepScanLineInputFile`] into this file,
    /// without uncompressing and then recompressing the pixel data.
    ///
    /// This file's header must be compatible with the [`DeepScanLineInputFile`]'s
    /// header:  The two header's "dataWindow", "compression",
    /// "lineOrder" and "channels" attributes must be the same.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If the headers do not match
    /// * [`Error::LogicError`] - If scan lines have already been written to this file.
    ///
    pub fn copy_pixels_from_file(
        &mut self,
        file: &DeepScanLineInputFile,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepScanLineOutputFile_copyPixels_from_file(
                self.0, file.0,
            )
            .into_result()?;
        }
        Ok(())
    }

    /// Shortcut to copy all pixels from a [`DeepScanLineInputPart`] into this file,
    /// without uncompressing and then recompressing the pixel data.
    ///
    /// This file's header must be compatible with the [`DeepScanLineInputPart`]'s
    /// header:  The two header's "dataWindow", "compression",
    /// "lineOrder" and "channels" attributes must be the same.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - If the headers do not match
    /// * [`Error::LogicError`] - If scan lines have already been written to this file.
    ///
    pub fn copy_pixels_from_part(
        &mut self,
        file: &mut DeepScanLineInputPart,
    ) -> Result<()> {
        unsafe {
            sys::Imf_DeepScanLineOutputFile_copyPixels_from_part(
                self.0,
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
            sys::Imf_DeepScanLineOutputFile_updatePreviewImage(
                self.0,
                new_pixels.as_ptr() as *const sys::Imf_PreviewRgba_t,
            )
            .into_result()?;
        }

        Ok(())
    }
}

impl Drop for DeepScanLineOutputFile {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_DeepScanLineOutputFile_dtor(self.0);
        }
    }
}

#[cfg(test)]
#[test]
fn test_write_deep1() -> Result<()> {
    use crate::{
        core::{
            channel_list::{CHANNEL_FLOAT, CHANNEL_HALF},
            frame_buffer::Frame,
            header::ImageType,
            Compression, LineOrder,
        },
        deep::deep_frame_buffer::{DeepFrameBuffer, DeepSlice},
    };
    use half::f16;

    let width = 256;
    let height = 256;
    let num_pixels = (width * height) as usize;

    let data_window = [0, 0, width - 1, height - 1];

    let mut header = Header::with_dimensions(
        width,
        height,
        1.0,
        [0.0f32, 0.0],
        1.0,
        LineOrder::IncreasingY,
        Compression::No,
    )?;

    header.set_image_type(ImageType::DeepScanline);
    header.channels_mut().insert("Z", &CHANNEL_FLOAT);
    header.channels_mut().insert("A", &CHANNEL_HALF);

    let mut frame_buffer = DeepFrameBuffer::new();
    let sample_counts = vec![1u32; num_pixels];
    let sample_count_frame =
        Frame::with_vec(&["sampleCounts"], sample_counts, data_window)?;
    frame_buffer.set_sample_count_frame(sample_count_frame)?;

    let (mut z_pixels, mut a_pixels): (Vec<Vec<f32>>, Vec<Vec<f16>>) =
        crate::tests::deep_testpattern(width, height);

    let mut z_ptrs: Vec<*mut f32> =
        z_pixels.iter_mut().map(|v| v.as_mut_ptr()).collect();
    let mut a_ptrs: Vec<*mut f16> =
        a_pixels.iter_mut().map(|v| v.as_mut_ptr()).collect();

    // set slices
    frame_buffer.insert(
        "Z",
        &DeepSlice::from_sample_ptr(z_ptrs.as_mut_ptr(), width).build()?,
    )?;

    frame_buffer.insert(
        "A",
        &DeepSlice::from_sample_ptr(a_ptrs.as_mut_ptr(), width).build()?,
    )?;

    let mut file = DeepScanLineOutputFile::new("write_deep1.exr", &header, 4)?;
    file.set_frame_buffer(&frame_buffer)?;
    unsafe {
        file.write_pixels(height)?;
    }

    Ok(())
}
