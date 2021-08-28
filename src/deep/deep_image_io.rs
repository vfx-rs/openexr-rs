use openexr_sys as sys;

use crate::{
    core::{cppstd::CppString, error::Error, header::Header, DataWindowSource},
    deep::deep_image::DeepImage,
};

use std::path::Path;

type Result<T, E = Error> = std::result::Result<T, E>;

/// Saves `image` in an OpenEXR file at path `filename`.  The file will be tiled
/// if the image has more than one level, otherwise the file will be scan-line
/// based.
///
/// # Errors
/// * [`Error::Base`] - if any error occurs
//
pub fn save_deep_image<P: AsRef<Path>>(
    filename: P,
    image: &DeepImage,
) -> Result<()> {
    unsafe {
        let s = CppString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        );

        sys::Imf_saveDeepImage(s.0, image.0).into_result()?;
    }

    Ok(())
}

/// Saves `image` in an OpenEXR file at path `filename`.  
///
/// The file will be tiled if the image has more than one level, or if `header`
/// is given and contains a tile description attribute; otherwise the file will
/// be scan-line based.
///
/// The header in `image` and the provided `header are merged into the output.
/// The channel list, level mode and level rounding mode are always taken from
/// `image` rather than `header`.
/// If `data_window_source`, is [`DataWindowSource::Image`], then the data
/// window from `image` is used, otherwise if `data_window_source` is set to
/// [`DataWindowSource::Header`], then the data window in the output file is
/// the intersection of the data window in `header` and the data window in `image`.
///
/// # Errors
/// * [`Error::Base`] - if any error occurs
//
pub fn save_deep_image_with_header<P: AsRef<Path>>(
    filename: P,
    header: &Header,
    image: &DeepImage,
    data_window_source: DataWindowSource,
) -> Result<()> {
    unsafe {
        let s = CppString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        );

        sys::Imf_saveDeepImage_with_header(
            s.0,
            header.0.as_ref() as *const sys::Imf_Header_t,
            image.0,
            data_window_source.into(),
        )
        .into_result()?;
    }

    Ok(())
}

/// Loads a deep image from `filename`.
///
/// # Errors
/// * [`Error::InvalidArgument`] - if the given file is not an OpenEXR file,
/// not a deep file, or is a multi-part file.
/// * [`Error::Base`] - if any other error occurs
//
pub fn load_deep_image<P: AsRef<Path>>(filename: P) -> Result<DeepImage> {
    unsafe {
        let s = CppString::new(
            filename
                .as_ref()
                .to_str()
                .expect("Invalid bytes in filename"),
        );

        #[allow(unused_mut)]
        let mut img = DeepImage::default();
        sys::Imf_loadDeepImage(s.0, img.0).into_result()?;

        Ok(img)
    }
}

#[cfg(test)]
#[test]
fn test_deep_image_io() -> Result<(), Box<dyn std::error::Error>> {
    use std::path::PathBuf;
    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("deep_cyl.exr");

    let im = load_deep_image(&path)?;
    save_deep_image("test_deep_image_io.exr", &im)?;

    Ok(())
}
