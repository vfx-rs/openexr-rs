use crate::{
    core::{error::Error, frame_buffer::FrameBuffer},
    deep::{
        deep_scan_line_input_file::DeepScanLineInputFile,
        deep_scan_line_input_part::DeepScanLineInputPart,
    },
};
use imath_traits::Bound2;
use openexr_sys as sys;

type Result<T, E = Error> = std::result::Result<T, E>;

/// Allows to composite multiple deep parts/files together into a single
/// frame_buffer
///
/// ```no_run
/// # fn cds() -> Result<(), openexr::Error> {
/// use openexr::prelude::*;
/// use openexr::deep::{CompositeDeepScanLine, DeepScanLineInputFile};
///
/// // Open the two input deeps we want to composite
/// let file_a = DeepScanLineInputFile::new("a.exr", 4)?;
/// let data_window_a = *file_a.header().data_window::<[i32; 4]>();
///
/// let file_b = DeepScanLineInputFile::new("b.exr", 4)?;
/// let data_window_b = *file_b.header().data_window::<[i32; 4]>();
///
/// // Get the union of their data windows
/// let data_window = [
///     data_window_a[0].min(data_window_b[0]),
///     data_window_a[1].min(data_window_b[1]),
///     data_window_a[2].max(data_window_b[2]),
///     data_window_a[3].max(data_window_b[3]),
/// ];
///
/// // Create a frame buffer to hold the composited image
/// let mut frame_buffer = FrameBuffer::new();
///
/// frame_buffer.insert_frame(Frame::new::<Rgba, _, _>(
///     &["R", "G", "B", "A"],
///     data_window,
/// )?)?;
///
/// frame_buffer.insert_frame(Frame::new::<f32, _, _>(&["Z"], data_window)?)?;
///
/// // Create the CompositeDeepScanLine, add the files, set the output
/// // frame buffer and read the pixels to perform the compositing.
/// let mut cds = CompositeDeepScanLine::new();
/// cds.add_source_file(&file_a)?;
/// cds.add_source_file(&file_b)?;
/// cds.set_frame_buffer(&frame_buffer);
///
/// // If this call completes successfully, `frame_buffer` will hold the
/// // composited R, G, B, A channels.
/// cds.read_pixels(data_window[1], data_window[3])?;
/// # Ok(())
/// # }
/// ```
pub struct CompositeDeepScanLine<'a, 'p> {
    pub(crate) ptr: *mut sys::Imf_CompositeDeepScanLine_t,
    parts: Vec<&'a DeepScanLineInputPart<'p>>,
    files: Vec<&'a DeepScanLineInputFile>,
    frame_buffer: Option<&'a FrameBuffer>,
}

impl<'a, 'p> CompositeDeepScanLine<'a, 'p> {
    /// Creates a Default CompositeDeepScanLine
    ///
    pub fn new() -> CompositeDeepScanLine<'a, 'p> {
        CompositeDeepScanLine::default()
    }

    /// Add a source part
    ///
    /// ## Errors
    /// * [`Error::InvalidArgument`] - If the part is missing deep Z or alpha
    /// channels, or if the display window does not match previously added sources
    ///
    pub fn add_source_part(
        &mut self,
        part: &'a mut DeepScanLineInputPart<'p>,
    ) -> Result<()> {
        unsafe {
            sys::Imf_CompositeDeepScanLine_addSource_part(
                self.ptr,
                &mut part.inner,
            )
            .into_result()?;
        }
        // keep a reference to enforce that the part must live at least as long
        // as we do
        self.parts.push(part);

        Ok(())
    }

    /// Add a source file
    ///
    /// ## Errors
    /// * [`Error::InvalidArgument`] - If the file is missing deep Z or alpha
    /// channels, or if the display window does not match previously added sources
    ///
    pub fn add_source_file(
        &mut self,
        file: &'a DeepScanLineInputFile,
    ) -> Result<()> {
        unsafe {
            sys::Imf_CompositeDeepScanLine_addSource_file(self.ptr, file.0)
                .into_result()?;
        }
        // keep a reference to enforce that the file must live at least as long
        // as we do
        self.files.push(file);

        Ok(())
    }

    /// Set the frame buffer to composite into. The buffer specified must be
    /// large enough to handle the unifed data window of all sources
    ///
    pub fn set_frame_buffer(&mut self, frame_buffer: &'a FrameBuffer) {
        unsafe {
            sys::Imf_CompositeDeepScanLine_setFrameBuffer(
                self.ptr,
                frame_buffer.ptr,
            )
            .into_result()
            .unwrap();
        }
        self.frame_buffer = Some(frame_buffer);
    }

    /// Retrieve the frame buffer
    ///
    pub fn frame_buffer(&self) -> Option<&'a FrameBuffer> {
        self.frame_buffer
    }

    /// Read scanlines from the sources from `start` to `end`, compositing them
    /// and storing them in the previously provided frame buffer
    ///
    /// ## Errors
    /// * [`Error::OutOfRange`] - If the start or end scanlines are outside
    /// of the data window.
    ///
    pub fn read_pixels(&mut self, start: i32, end: i32) -> Result<()> {
        let dw = *self.data_window::<[i32; 4]>();
        if start < dw[1] || end > dw[3] {
            Err(Error::OutOfRange)
        } else {
            unsafe {
                sys::Imf_CompositeDeepScanLine_readPixels(self.ptr, start, end)
                    .into_result()?;
            }

            Ok(())
        }
    }

    /// The number of sources added
    ///
    pub fn num_sources(&self) -> i32 {
        let mut num = 0;
        unsafe {
            sys::Imf_CompositeDeepScanLine_sources(self.ptr, &mut num)
                .into_result()
                .unwrap();
        }
        num
    }

    /// Get the data window
    ///
    /// If multiple parts are specified, this will be the union of each of their
    /// data windows.
    ///
    pub fn data_window<B>(&self) -> &B
    where
        B: Bound2<i32>,
    {
        let mut ptr = std::ptr::null();
        unsafe {
            sys::Imf_CompositeDeepScanLine_dataWindow(self.ptr, &mut ptr);
            &*(ptr as *const B)
        }
    }

    // TODO: implement set_compositing()
}

impl<'a, 'p> Default for CompositeDeepScanLine<'a, 'p> {
    fn default() -> CompositeDeepScanLine<'a, 'p> {
        unsafe {
            let mut ptr = std::ptr::null_mut();
            sys::Imf_CompositeDeepScanLine_ctor(&mut ptr);

            CompositeDeepScanLine {
                ptr,
                parts: Vec::new(),
                files: Vec::new(),
                frame_buffer: None,
            }
        }
    }
}

#[cfg(test)]
#[test]
fn composite_deep1() -> Result<()> {
    use crate::{
        core::{
            channel_list::{CHANNEL_FLOAT, CHANNEL_HALF},
            frame_buffer::Frame,
            header::Header,
            output_file::OutputFile,
        },
        rgba::rgba::Rgba,
    };

    use std::path::PathBuf;

    let path_plane = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("deep_plane.exr");

    let path_cyl = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("deep_cyl.exr");

    let file_plane = DeepScanLineInputFile::new(&path_plane, 4)?;
    let header_plane = file_plane.header();
    let data_window_plane = *header_plane.data_window::<[i32; 4]>();

    let file_cyl = DeepScanLineInputFile::new(&path_cyl, 4)?;
    let header_cyl = file_cyl.header();
    let data_window_cyl = *header_cyl.data_window::<[i32; 4]>();

    let data_window = [
        data_window_plane[0].min(data_window_cyl[0]),
        data_window_plane[1].min(data_window_cyl[1]),
        data_window_plane[2].max(data_window_cyl[2]),
        data_window_plane[3].max(data_window_cyl[3]),
    ];

    let mut frame_buffer = FrameBuffer::new();

    frame_buffer.insert_frame(Frame::new::<Rgba, _, _>(
        &["R", "G", "B", "A"],
        data_window,
    )?)?;

    frame_buffer.insert_frame(Frame::new::<f32, _, _>(&["Z"], data_window)?)?;

    let mut cds = CompositeDeepScanLine::new();
    cds.add_source_file(&file_plane)?;
    cds.add_source_file(&file_cyl)?;
    cds.set_frame_buffer(&frame_buffer);
    cds.read_pixels(data_window[1], data_window[3])?;

    let mut header = Header::new(
        data_window,
        *header_plane.display_window(),
        1.0,
        [0.0f32; 2],
        1.0,
        sys::LineOrder::IncreasingY,
        sys::Compression::Zips,
    )?;

    for c in &["R", "G", "B", "A"] {
        header.channels_mut().insert(c, &CHANNEL_HALF);
    }
    header.channels_mut().insert("Z", &CHANNEL_FLOAT);

    let mut file = OutputFile::new("composite_deep1.exr", &header, 6).unwrap();
    file.set_frame_buffer(&frame_buffer).unwrap();
    unsafe {
        file.write_pixels(data_window[3] - data_window[1] + 1)
            .unwrap()
    };

    Ok(())
}
