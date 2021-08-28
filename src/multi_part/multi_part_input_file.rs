use openexr_sys as sys;

use std::ffi::CString;
use std::path::Path;

use crate::core::{error::Error, header::HeaderRef, version::Version};

type Result<T, E = Error> = std::result::Result<T, E>;

/// Manages writing multi-part images.
///
/// Multi-part images are essentially just containers around multiple
/// [`InputFile`](crate::core::input_file::InputFile)s.
///
/// Certain attributes are shared between all parts:
/// * `displayWindow`
/// * `pixelAspectRatio`
/// * `timeCode`
/// * `chromaticities`
///
#[repr(transparent)]
pub struct MultiPartInputFile(pub(crate) *mut sys::Imf_MultiPartInputFile_t);

impl MultiPartInputFile {
    /// Create a new `MultiPartInputFile` to read the file at `filename`.
    ///
    /// # Errors
    /// * [`Error::Base`] - if any error occurs.
    ///
    pub fn new<P: AsRef<Path>>(
        filename: P,
        num_threads: i32,
        reconstruct_chunk_offset_table: bool,
    ) -> Result<MultiPartInputFile> {
        let c_filename = CString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        )
        .expect("Internal null bytes in filename");

        let mut ptr = std::ptr::null_mut();

        unsafe {
            sys::Imf_MultiPartInputFile_ctor(
                &mut ptr,
                c_filename.as_ptr(),
                num_threads,
                reconstruct_chunk_offset_table,
            )
            .into_result()?;
        }

        Ok(MultiPartInputFile(ptr))
    }

    /// The number of parts in the file
    ///
    pub fn parts(&self) -> i32 {
        let mut v = 0;
        unsafe {
            sys::Imf_MultiPartInputFile_parts(self.0, &mut v)
                .into_result()
                .expect("MultiPartInputFile::parts");
        }

        v
    }

    /// Get a reference to the [`Header`](crate::core::header::Header) for part `n`.
    ///
    /// # Errors
    /// * [`Error::OutOfRange`] - if `n` does not index a part in the file.
    ///
    pub fn header(&self, n: i32) -> Result<HeaderRef> {
        // OpenEXR itself doesn't handle this
        // https://github.com/AcademySoftwareFoundation/openexr/issues/1031
        if n < 0 || n >= self.parts() {
            Err(Error::OutOfRange)
        } else {
            let mut ptr = std::ptr::null();
            unsafe {
                sys::Imf_MultiPartInputFile_header(self.0, &mut ptr, n)
                    .into_result()
                    .expect("MultiPartInputFile::header");
            }

            Ok(HeaderRef::new(ptr))
        }
    }

    /// Get the file format version
    ///
    pub fn version(&self) -> Version {
        let mut v = 0;
        unsafe {
            sys::Imf_MultiPartInputFile_version(self.0, &mut v)
                .into_result()
                .expect("MultiPartInputFile::version");
        }

        Version::from_c_int(v)
    }

    /// Check whether the entire chunk offset table for the part is written
    /// correctly.
    ///
    pub fn part_complete(&self, part: i32) -> bool {
        let mut v = false;
        unsafe {
            sys::Imf_MultiPartInputFile_partComplete(self.0, &mut v, part)
                .into_result()
                .expect("MultiPartInputFile::partComplete");
        }

        v
    }

    /// Flushes the internal part cache.
    ///
    /// Intended for debugging purposes, but can be used to temporarily reduce
    /// memory overhead, or to switch between types (e.g. TiledInputPart, to
    /// DeepScanlineInputPart to InputPart).
    ///
    pub fn flush_part_cache(&mut self) {
        unsafe {
            sys::Imf_MultiPartInputFile_flushPartCache(self.0)
                .into_result()
                .expect("MultiPartInputFile::flushPartCache");
        }
    }
}

impl Drop for MultiPartInputFile {
    fn drop(&mut self) {
        unsafe { sys::Imf_MultiPartInputFile_dtor(self.0) };
    }
}

#[cfg(test)]
#[test]
fn read_multipartinputfile1() -> Result<()> {
    use crate::multi_part::multi_part_input_file::MultiPartInputFile;
    use std::path::PathBuf;

    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("ferris-multipart.exr");

    let file = MultiPartInputFile::new(path, 4, true)?;

    assert_eq!(file.parts(), 2);

    assert_eq!(file.header(0).unwrap().name().unwrap(), "left");
    assert_eq!(file.header(1).unwrap().name().unwrap(), "right");

    assert_eq!(file.version().is_multi_part(), true);
    assert_eq!(file.version().is_tiled(), false);

    Ok(())
}
