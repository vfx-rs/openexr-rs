use openexr_sys as sys;

use crate::{
    core::{error::Error, header::HeaderRef},
    deep::deep_frame_buffer::{DeepFrameBuffer, DeepFrameBufferRef},
};

use std::ffi::CString;
use std::path::Path;

type Result<T, E = Error> = std::result::Result<T, E>;

#[repr(transparent)]
pub struct DeepScanLineInputFile(
    pub(crate) *mut sys::Imf_DeepScanLineInputFile_t,
);

impl DeepScanLineInputFile {
    /// Open the file at path `filename` and read the header.
    ///
    /// # Errors
    /// * [`Error::Base`] - if the file cannot be opened
    ///
    pub fn new<P: AsRef<Path>>(
        filename: P,
        num_threads: i32,
    ) -> Result<DeepScanLineInputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null_mut();
        unsafe {
            sys::Imf_DeepScanLineInputFile_ctor(
                &mut ptr,
                c_filename.as_ptr(),
                num_threads,
            )
            .into_result()?;
        }

        Ok(DeepScanLineInputFile(ptr))
    }

    /// Access to the file [`Header`](crate::core::header::Header)
    ///
    pub fn header(&self) -> HeaderRef {
        unsafe {
            let mut ptr = std::ptr::null();
            sys::Imf_DeepScanLineInputFile_header(self.0, &mut ptr);
            if ptr.is_null() {
                panic!("Received null ptr from sys::Imf_DeepScanLineInputFile_header");
            }

            HeaderRef::new(ptr)
        }
    }

    /// Access to the file format version
    ///
    pub fn version(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_DeepScanLineInputFile_version(self.0, &mut v);
        }
        v
    }

    /// Set the current frame buffer -- copies the FrameBuffer
    /// object into the DeepScanLineInputFile object.
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
            sys::Imf_DeepScanLineInputFile_setFrameBuffer(
                self.0,
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
            sys::Imf_DeepScanLineInputFile_frameBuffer(self.0, &mut ptr);
        }

        DeepFrameBufferRef::new(ptr)
    }

    /// Check if all pixels in the data window are present in the input file
    ///
    pub fn is_complete(&self) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_DeepScanLineInputFile_isComplete(self.0, &mut v);
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
            sys::Imf_DeepScanLineInputFile_readPixels(self.0, s1, s2)
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
            sys::Imf_DeepScanLineInputFile_readPixelSampleCounts(
                self.0, s1, s2,
            )
            .into_result()?;
        }

        Ok(())
    }
}

pub struct DeepScanLineInputFileReader {
    pub(crate) inner: *mut sys::Imf_DeepScanLineInputFile_t,
    frame_buffer: DeepFrameBuffer,
}

#[cfg(test)]
#[test]
fn read_deep1() {
    use crate::{
        core::frame_buffer::Frame, deep::deep_frame_buffer::DeepSlice,
    };
    use half::f16;
    use itertools::izip;
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::path::PathBuf;

    let path_cyl = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("deep_plane.exr");

    let mut file = DeepScanLineInputFile::new(&path_cyl, 4).unwrap();
    let header = file.header();

    let data_window = *header.data_window::<[i32; 4]>();

    let data_width = data_window[2] - data_window[0] + 1;
    let data_height = data_window[3] - data_window[1] + 1;

    let offset = (-data_window[0] - data_window[1] * data_width) as isize;

    let mut data_z = vec![
        std::ptr::null_mut() as *mut f32;
        (data_width * data_height) as usize
    ];
    let mut data_zback = vec![
        std::ptr::null_mut() as *mut f32;
        (data_width * data_height) as usize
    ];
    let mut data_r = vec![
        std::ptr::null_mut() as *mut f16;
        (data_width * data_height) as usize
    ];
    let mut data_g = vec![
        std::ptr::null_mut() as *mut f16;
        (data_width * data_height) as usize
    ];
    let mut data_b = vec![
        std::ptr::null_mut() as *mut f16;
        (data_width * data_height) as usize
    ];
    let mut data_a = vec![
        std::ptr::null_mut() as *mut f16;
        (data_width * data_height) as usize
    ];

    let mut frame_buffer = DeepFrameBuffer::new();

    let sample_count_frame =
        Frame::new::<u32, _, _>(&["sampleCounts"], data_window).unwrap();

    frame_buffer
        .set_sample_count_frame(sample_count_frame)
        .unwrap();

    // add slices for all the data we're interested in
    unsafe {
        frame_buffer
            .insert(
                "Z",
                &DeepSlice::builder(
                    sys::PixelType::Float,
                    data_z.as_mut_ptr().offset(offset) as *mut i8,
                )
                .x_stride(std::mem::size_of::<*const f32>())
                .y_stride(
                    std::mem::size_of::<*const f32>() * data_width as usize,
                )
                .sample_stride(std::mem::size_of::<f32>())
                .build()
                .unwrap(),
            )
            .unwrap();

        frame_buffer
            .insert(
                "ZBack",
                &DeepSlice::builder(
                    sys::PixelType::Float,
                    data_zback.as_mut_ptr().offset(offset) as *mut i8,
                )
                .x_stride(std::mem::size_of::<*const f32>())
                .y_stride(
                    std::mem::size_of::<*const f32>() * data_width as usize,
                )
                .sample_stride(std::mem::size_of::<f32>())
                .build()
                .unwrap(),
            )
            .unwrap();

        frame_buffer
            .insert(
                "R",
                &DeepSlice::builder(
                    sys::PixelType::Half,
                    data_r.as_mut_ptr().offset(offset) as *mut i8,
                )
                .x_stride(std::mem::size_of::<*const f16>())
                .y_stride(
                    std::mem::size_of::<*const f16>() * data_width as usize,
                )
                .sample_stride(std::mem::size_of::<f16>())
                .build()
                .unwrap(),
            )
            .unwrap();

        frame_buffer
            .insert(
                "G",
                &DeepSlice::builder(
                    sys::PixelType::Half,
                    data_g.as_mut_ptr().offset(offset) as *mut i8,
                )
                .x_stride(std::mem::size_of::<*const f16>())
                .y_stride(
                    std::mem::size_of::<*const f16>() * data_width as usize,
                )
                .sample_stride(std::mem::size_of::<f16>())
                .build()
                .unwrap(),
            )
            .unwrap();

        frame_buffer
            .insert(
                "B",
                &DeepSlice::builder(
                    sys::PixelType::Half,
                    data_b.as_mut_ptr().offset(offset) as *mut i8,
                )
                .x_stride(std::mem::size_of::<*const f16>())
                .y_stride(
                    std::mem::size_of::<*const f16>() * data_width as usize,
                )
                .sample_stride(std::mem::size_of::<f16>())
                .build()
                .unwrap(),
            )
            .unwrap();

        frame_buffer
            .insert(
                "A",
                &DeepSlice::builder(
                    sys::PixelType::Half,
                    data_a.as_mut_ptr().offset(offset) as *mut i8,
                )
                .x_stride(std::mem::size_of::<*const f16>())
                .y_stride(
                    std::mem::size_of::<*const f16>() * data_width as usize,
                )
                .sample_stride(std::mem::size_of::<f16>())
                .build()
                .unwrap(),
            )
            .unwrap();
    }

    // set the frame buffer and read the sample counts channel
    file.set_frame_buffer(&frame_buffer).unwrap();
    file.read_pixel_sample_counts(data_window[1], data_window[3])
        .unwrap();

    let sample_count_frame = frame_buffer.sample_count_frame.take().unwrap();

    // pre-allocate the memory for the sample data using the sample counts
    // channel
    for (sc, z, zback, r, g, b, a) in izip!(
        sample_count_frame.as_slice::<u32>(),
        &mut data_z,
        &mut data_zback,
        &mut data_r,
        &mut data_g,
        &mut data_b,
        &mut data_a
    ) {
        let sz = *sc as usize;

        unsafe {
            *z = System.alloc(Layout::array::<f32>(sz).unwrap()) as *mut f32;
            *zback =
                System.alloc(Layout::array::<f32>(sz).unwrap()) as *mut f32;
            *r = System.alloc(Layout::array::<f16>(sz).unwrap()) as *mut f16;
            *g = System.alloc(Layout::array::<f16>(sz).unwrap()) as *mut f16;
            *b = System.alloc(Layout::array::<f16>(sz).unwrap()) as *mut f16;
            *a = System.alloc(Layout::array::<f16>(sz).unwrap()) as *mut f16;
        }
    }

    frame_buffer
        .set_sample_count_frame(sample_count_frame)
        .unwrap();

    // read the data
    file.read_pixels(data_window[1], data_window[3]).unwrap();
}
