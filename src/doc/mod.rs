//! # Documentation
//!
//! This module contains markdown conversions of the long-form PDF documentation
//! available on <https://www.openexr.com>, giving tutorial- and white-paper-
//! style articles on all aspects of the OpenEXR library.
//!
//! * [Reading and Writing Image Files](crate::doc::reading_and_writing_image_files) - A
//! tutorial-style guide to the main image reading and writing interfaces.
//! * [Technical Introduction](crate::doc::technical_introduction) - A technical overview of the
//! OpenEXR format and its related concepts.
//! * [Interpreting Deep Pixels](crate::doc::interpreting_deep_pixels) - An in-depth look at how
//! deep pixels are stored and how to manipulate their samples.
//! * [Multi-View OpenEXR](crate::doc::multi_view_open_exr) - Representation of multi-view images
//! in OpenEXR files.
//!

#[cfg(feature = "long-form-docs")]
use embed_doc_image::embed_doc_image;

cfg_if::cfg_if! {
if #[cfg(feature = "long-form-docs")] {
    #[doc = include_str!("reading_and_writing_image_files.md")]
    #[embed_doc_image("env_latlong", "src/doc/images/rawif_env_latlong.png")]
    #[embed_doc_image("env_cubemap", "src/doc/images/rawif_env_cubemap.png")]
    pub mod reading_and_writing_image_files {}

    #[doc = include_str!("technical_introduction.md")]
    #[embed_doc_image("ti_windows", "src/doc/images/ti_windows.png")]
    #[embed_doc_image("ti_windows2", "src/doc/images/ti_windows2.png")]
    #[embed_doc_image("ti_image3", "src/doc/images/ti_image3.png")]
    #[embed_doc_image("ti_image4", "src/doc/images/ti_image4.png")]
    pub mod technical_introduction {}

    pub mod open_exr_file_layout {}

    #[doc = include_str!("interpreting_deep_pixels.md")]
    #[embed_doc_image("point_sample", "src/doc/images/idi_point_sample.png")]
    #[embed_doc_image("volume_sample", "src/doc/images/idi_volume_sample.png")]
    #[embed_doc_image(
        "alpha_and_colour",
        "src/doc/images/idi_alpha_and_colour.png"
    )]
    #[embed_doc_image(
        "opaque_volume_samples",
        "src/doc/images/idi_opaque_volume_samples.png"
    )]
    #[embed_doc_image("tidy1a", "src/doc/images/idi_tidy1a.png")]
    #[embed_doc_image("tidy1b", "src/doc/images/idi_tidy1b.png")]
    #[embed_doc_image("tidy2a", "src/doc/images/idi_tidy2a.png")]
    #[embed_doc_image("tidy3", "src/doc/images/idi_tidy3.png")]
    #[embed_doc_image("whole_pixel", "src/doc/images/idi_whole_pixel.png")]
    pub mod interpreting_deep_pixels {}

    pub mod multi_view_open_exr;
}
}
