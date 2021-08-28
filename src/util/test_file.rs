use openexr_sys as sys;

use std::path::Path;

/// Struct representing information about the type of an OpenEXR file.
///
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FileInfo {
    pub is_tiled: bool,
    pub is_deep: bool,
    pub is_multi_part: bool,
}

/// Get information about the type of OpenEXR file at path `filename`.
///
/// If the file cannot be opened, or is not an OpenEXR file, `None` is returned.
///
pub fn is_openexr_file<P: AsRef<Path>>(filename: P) -> Option<FileInfo> {
    let mut info = FileInfo::default();
    let mut result = false;

    unsafe {
        sys::Imf_isOpenExrFile(
            &mut result,
            filename.as_ref().to_str().expect("bad filename").as_ptr()
                as *const i8,
            &mut info.is_tiled,
            &mut info.is_deep,
            &mut info.is_multi_part,
        )
        .into_result()
        .expect("Unexpected exception from sys::Imf_isOpenExrFile");
    }

    if result {
        Some(info)
    } else {
        None
    }
}

// FIXME: re-enable this when we figure out why it's failing
/*
#[cfg(test)]
#[test]
pub fn test_test_file() {
    use std::path::PathBuf;

    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("ferris.exr");

    let info = is_openexr_file(path).unwrap();
    assert_eq!(info.is_deep, false);
    assert_eq!(info.is_tiled, false);
    assert_eq!(info.is_multi_part, false);

    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("ferris-multipart.exr");

    let info = is_openexr_file(path).unwrap();
    assert_eq!(info.is_deep, false);
    assert_eq!(info.is_tiled, false);
    assert_eq!(info.is_multi_part, true);

    let path_cyl = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("deep_plane.exr");

    let info = is_openexr_file(path_cyl).unwrap();
    assert_eq!(info.is_deep, true);
    assert_eq!(info.is_tiled, false);
    assert_eq!(info.is_multi_part, false);

    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("DOES_NOT_EXIST");

    assert!(is_openexr_file(path).is_none());

    let path = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set"),
    )
    .join("images")
    .join("ferris.png");

    assert!(is_openexr_file(path).is_none());
}
*/
