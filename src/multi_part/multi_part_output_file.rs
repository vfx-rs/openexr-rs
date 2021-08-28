use openexr_sys as sys;

use std::ffi::CString;
use std::path::Path;

use crate::core::{
    error::Error,
    header::{HeaderRef, HeaderSlice},
};

type Result<T, E = Error> = std::result::Result<T, E>;

/// Manages writing multi-part images.
///
/// Multi-part files are essentially just containers around multiple
/// [`OutputFile`](crate::core::output_file::OutputFile)s
///
/// Certain attributes are shared between all parts:
/// * `displayWindow`
/// * `pixelAspectRatio`
/// * `timeCode`
/// * `chromaticities`
///
#[repr(transparent)]
pub struct MultiPartOutputFile(pub(crate) *mut sys::Imf_MultiPartOutputFile_t);

impl MultiPartOutputFile {
    /// Creates a new multi-part output file from a slice of slice of headers
    /// with the given filename. If `override_shared_attributes` is `true`, then
    /// mismatching attributes between headers will be overriden with the value
    /// from the first header. If `false` then [`Error::InvalidArgument`] is
    /// returned instead.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if `override_shared_attributes` is `false`
    /// and there is a mismatch between headers' common attributes, or if each
    /// header does not have a unique name.
    /// * [`Error::Base`] - if the file cannot be opened.
    ///
    pub fn new<P: AsRef<Path>>(
        filename: P,
        headers: &HeaderSlice,
        override_shared_attributes: bool,
        num_threads: i32,
    ) -> Result<MultiPartOutputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_MultiPartOutputFile_ctor(
                &mut ptr,
                c_filename.as_ptr(),
                headers.0.as_ptr(),
                headers.0.len() as i32,
                override_shared_attributes,
                num_threads,
            )
            .into_result()?;
        }

        Ok(MultiPartOutputFile(ptr))
    }

    /// The number of parts in the file
    ///
    pub fn parts(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_MultiPartOutputFile_parts(self.0, &mut v);
        }
        v
    }

    /// Return a [`Header`](crate::core::header::Header) for part `n`.
    ///
    /// Due to enforcing attribute sharing, the attributes of the returned
    /// [`Header`](crate::core::header::Header) may be different from the matching one passed in to the
    /// constructor.
    ///
    /// # Errors
    /// * [`Error::InvalidArgument`] - if `n` does not correspond to a part in
    /// the file.
    ///
    pub fn header(&self, n: i32) -> Result<HeaderRef> {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_MultiPartOutputFile_header(self.0, &mut ptr, n)
                .into_result()?;
        }
        Ok(HeaderRef::new(ptr))
    }
}

impl Drop for MultiPartOutputFile {
    fn drop(&mut self) {
        unsafe {
            sys::Imf_MultiPartOutputFile_dtor(self.0);
        }
    }
}

#[cfg(test)]
#[test]
fn write_multipartoutputfile1() {
    use crate::{
        core::{
            channel_list::Channel,
            frame_buffer::{FrameBuffer, Slice},
            header::{Header, ImageType},
            output_part::OutputPart,
            Compression, PixelType,
        },
        rgba::rgba::Rgba,
    };

    let (pixels, width, height) = crate::tests::load_ferris();

    let channel = Channel {
        type_: PixelType::Half.into(),
        x_sampling: 1,
        y_sampling: 1,
        p_linear: true,
    };

    let mut header_array = Header::new_array(2);
    let mut i = 0;
    for mut header in header_array.iter_mut() {
        header.set_compression(Compression::Zip);

        header.channels_mut().insert("R", &channel);
        header.channels_mut().insert("G", &channel);
        header.channels_mut().insert("B", &channel);
        header.channels_mut().insert("A", &channel);

        header.set_image_type(ImageType::Scanline);

        header.set_dimensions(width, height);

        if i == 0 {
            header.set_name("left");
            i += 1;
        } else {
            header.set_name("right");
        }
    }

    let mut frame_buffer_left = FrameBuffer::new();

    frame_buffer_left
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

    frame_buffer_left
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

    frame_buffer_left
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

    frame_buffer_left
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

    let mut frame_buffer_right = FrameBuffer::new();

    frame_buffer_right
        .insert(
            "B",
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

    frame_buffer_right
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

    frame_buffer_right
        .insert(
            "R",
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

    frame_buffer_right
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

    let file = MultiPartOutputFile::new(
        "write_multipartoutputfile1.exr",
        &header_array,
        true,
        4,
    )
    .unwrap();

    let mut part_left = OutputPart::new(&file, 0).unwrap();
    let mut part_right = OutputPart::new(&file, 1).unwrap();

    part_left.set_frame_buffer(&frame_buffer_left).unwrap();
    part_right.set_frame_buffer(&frame_buffer_right).unwrap();

    unsafe {
        part_left.write_pixels(height).unwrap();
        part_right.write_pixels(height).unwrap();
    }
}
