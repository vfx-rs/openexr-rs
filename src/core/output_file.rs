use openexr_sys as sys;

use std::ffi::CString;
use std::path::Path;

use crate::core::{
    error::Error,
    frame_buffer::{FrameBuffer, FrameBufferRef},
    header::{Header, HeaderRef},
    input_file::InputFile,
    input_part::InputPart,
    preview_image::PreviewRgba,
};

type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(transparent)]
pub struct OutputFile(pub(crate) *mut sys::Imf_OutputFile_t);

impl OutputFile {
    /// Opens the file and writes the file header.
    ///
    /// The file header is also copied into the OutputFile object,
    /// and can later be accessed via the `header()` method.
    /// Dropping this OutputFile object automatically closes
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
    ) -> Result<OutputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut _inner = std::ptr::null_mut();
        unsafe {
            sys::Imf_OutputFile_ctor(
                &mut _inner,
                c_filename.as_ptr(),
                header.0.as_ref(),
                num_threads,
            )
            .into_result()?;
        }

        Ok(OutputFile(_inner))
    }

    /// Get the filename this file is writing to.
    ///
    pub fn file_name(&self) -> &str {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_OutputFile_fileName(self.0, &mut ptr)
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
            sys::Imf_OutputFile_header(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_OutputFile_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the OutputFile object.
    ///
    /// The current frame buffer is the source of the pixel
    /// data written to the file.  The current frame buffer
    /// must be set at least once before [`OutputFile::write_pixels`] is
    /// called.  The current frame buffer can be changed
    /// after each call to [`OutputFile::write_pixels`].
    ///
    /// ## Errors
    /// * [`Error::InvalidArgument`] - If the pixel type of the
    /// [`Channel`](crate::core::channel_list::Channel)s in the [`Header`]
    /// do not match the types in the frame buffer, or if the sampling rates do
    /// not match.
    ///
    pub fn set_frame_buffer(
        &mut self,
        frame_buffer: &FrameBuffer,
    ) -> Result<()> {
        unsafe {
            sys::Imf_OutputFile_setFrameBuffer(self.0, frame_buffer.ptr)
                .into_result()?;
        }

        Ok(())
    }

    /// Get a reference to the frame buffer.
    ///
    pub fn frame_buffer(&self) -> FrameBufferRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_OutputFile_frameBuffer(self.0, &mut ptr);
            if ptr.is_null() {
                panic!(
                    "Received null ptr from sys::Imf_OutputFile_frameBuffer"
                );
            }

            FrameBufferRef::new(ptr)
        }
    }

    /// Write pixel data.
    ///
    /// Retrieves the next `num_scan_lines` worth of data from
    /// the current frame buffer, starting with the scan line indicated by
    /// [`OutputFile::current_scan_line`] and stores the data in the output file, and
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
    pub unsafe fn write_pixels(&self, num_scan_lines: i32) -> Result<()> {
        sys::Imf_OutputFile_writePixels(self.0, num_scan_lines)
            .into_result()?;
        Ok(())
    }

    /// Access to the current scan line:
    ///
    /// Returns the y coordinate of the first scan line
    /// that will be read from the current frame buffer during the next
    /// call to [`OutputFile::write_pixels`].
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
            sys::Imf_OutputFile_currentScanLine(self.0, &mut v);
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
            sys::Imf_OutputFile_copyPixels_from_file(self.0, file.0)
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
            sys::Imf_OutputFile_copyPixels_from_part(self.0, &mut file.inner)
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
            sys::Imf_OutputFile_updatePreviewImage(
                self.0,
                new_pixels.as_ptr() as *const sys::Imf_PreviewRgba_t,
            )
            .into_result()?;
        }

        Ok(())
    }
}

impl Drop for OutputFile {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_OutputFile_dtor(self.0);
        }
    }
}

#[cfg(test)]
#[test]
fn write_outputfile1() {
    use crate::tests::load_ferris;
    use crate::{
        core::{channel_list::CHANNEL_HALF, frame_buffer::Slice, PixelType},
        rgba::rgba::Rgba,
    };

    let (pixels, width, height) = load_ferris();

    let mut header = Header::from_dimensions(width, height);

    header.channels_mut().insert("R", &CHANNEL_HALF);
    header.channels_mut().insert("G", &CHANNEL_HALF);
    header.channels_mut().insert("B", &CHANNEL_HALF);
    header.channels_mut().insert("A", &CHANNEL_HALF);

    let mut frame_buffer = FrameBuffer::new();

    frame_buffer
        .insert(
            "R",
            &Slice::builder(
                PixelType::Half,
                &pixels[0].r as *const _ as *const u8,
                width as i64,
                height as i64,
            )
            .x_stride(std::mem::size_of::<Rgba>())
            .build()
            .unwrap(),
        )
        .unwrap();

    frame_buffer
        .insert(
            "G",
            &Slice::builder(
                PixelType::Half,
                &pixels[0].g as *const _ as *const u8,
                width as i64,
                height as i64,
            )
            .x_stride(std::mem::size_of::<Rgba>())
            .build()
            .unwrap(),
        )
        .unwrap();

    frame_buffer
        .insert(
            "B",
            &Slice::builder(
                PixelType::Half,
                &pixels[0].b as *const _ as *const u8,
                width as i64,
                height as i64,
            )
            .x_stride(std::mem::size_of::<Rgba>())
            .build()
            .unwrap(),
        )
        .unwrap();

    frame_buffer
        .insert(
            "A",
            &Slice::builder(
                PixelType::Half,
                &pixels[0].a as *const _ as *const u8,
                width as i64,
                height as i64,
            )
            .x_stride(std::mem::size_of::<Rgba>())
            .build()
            .unwrap(),
        )
        .unwrap();

    let mut file =
        OutputFile::new("write_outputfile1.exr", &header, 1).unwrap();
    file.set_frame_buffer(&frame_buffer).unwrap();
    unsafe { file.write_pixels(height).unwrap() };
}

#[cfg(test)]
#[test]
fn write_outputfile2() {
    use crate::core::{channel_list::CHANNEL_HALF, frame_buffer::Frame};
    use crate::tests::load_ferris;

    let (pixels, width, height) = load_ferris();

    let mut header = Header::from_dimensions(width, height);

    let channel_names = ["R", "G", "B", "A"];
    for c in &channel_names {
        header.channels_mut().insert(c, &CHANNEL_HALF);
    }

    let mut frame_buffer = FrameBuffer::new();

    let frame = Frame::with_vec(
        &channel_names,
        pixels,
        *header.data_window::<[i32; 4]>(),
    )
    .unwrap();

    frame_buffer.insert_frame(frame).unwrap();

    let mut file =
        OutputFile::new("write_outputfile2.exr", &header, 6).unwrap();
    file.set_frame_buffer(&frame_buffer).unwrap();
    unsafe { file.write_pixels(height).unwrap() };
}

#[cfg(test)]
#[test]
fn write_gz1() -> Result<(), Box<dyn std::error::Error>> {
    use crate::core::{
        channel_list::{CHANNEL_FLOAT, CHANNEL_HALF},
        frame_buffer::Slice,
        PixelType,
    };
    use half::f16;

    let g: Vec<f16> = [0.0f32, 0.2, 0.4, 0.6, 0.8, 1.0]
        .iter()
        .map(|f| f16::from_f32(*f))
        .collect();
    let z = [1.0f32, 0.8, 0.6, 0.4, 0.2, 0.0];

    let width = 6;
    let height = 1;
    let _filename = "write_gz1.exr";

    let mut header = Header::from_dimensions(width, height);
    header.channels_mut().insert("G", &CHANNEL_HALF);
    header.channels_mut().insert("Z", &CHANNEL_FLOAT);

    let mut frame_buffer = FrameBuffer::new();

    frame_buffer.insert(
        "G",
        &Slice::builder(
            PixelType::Half,
            g.as_ptr() as *const u8,
            width as i64,
            height as i64,
        )
        .build()
        .unwrap(),
    )?;

    frame_buffer.insert(
        "Z",
        &Slice::builder(
            PixelType::Float,
            z.as_ptr() as *const u8,
            width as i64,
            height as i64,
        )
        .build()
        .unwrap(),
    )?;

    let mut file = OutputFile::new("write_gz1.exr", &header, 1).unwrap();
    file.set_frame_buffer(&frame_buffer).unwrap();
    unsafe { file.write_pixels(height).unwrap() };

    Ok(())
}

#[cfg(test)]
#[test]
fn write_gz2() -> Result<(), Box<dyn std::error::Error>> {
    use crate::core::{
        channel_list::{CHANNEL_FLOAT, CHANNEL_HALF},
        frame_buffer::Slice,
        PixelType,
    };
    use half::f16;

    let g: Vec<f16> = [0.0f32, 0.2, 0.4, 0.6, 0.8, 1.0]
        .iter()
        .map(|f| f16::from_f32(*f))
        .collect();
    let z = [1.0f32, 0.8, 0.6, 0.4, 0.2, 0.0];

    let width = 6;
    let height = 1;
    let _filename = "write_gz1.exr";
    let data_window = [2, 0, 4, 0];

    let mut header = Header::from_dimensions(width, height);
    header.channels_mut().insert("G", &CHANNEL_HALF);
    header.channels_mut().insert("Z", &CHANNEL_FLOAT);
    *header.data_window_mut() = data_window;

    let mut frame_buffer = FrameBuffer::new();

    frame_buffer.insert(
        "G",
        &Slice::builder(
            PixelType::Half,
            g.as_ptr() as *const u8,
            width as i64,
            height as i64,
        )
        .build()
        .unwrap(),
    )?;

    frame_buffer.insert(
        "Z",
        &Slice::builder(
            PixelType::Float,
            z.as_ptr() as *const u8,
            width as i64,
            height as i64,
        )
        .build()
        .unwrap(),
    )?;

    let mut file = OutputFile::new("write_gz2.exr", &header, 1).unwrap();
    file.set_frame_buffer(&frame_buffer).unwrap();
    unsafe {
        file.write_pixels(data_window[3] - data_window[1] + 1)
            .unwrap()
    };

    Ok(())
}
