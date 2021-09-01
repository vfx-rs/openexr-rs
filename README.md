# OpenEXR

The openexr crate provides high-level bindings for the [ASWF OpenEXR library](https://github.com/AcademySoftwareFoundation/openexr),
which allows reading and writing files in the OpenEXR format (EXR standing
for **EX**tended **R**ange). The OpenEXR format is the de-facto standard
image storage format of the motion-picture industry.

The purpose of EXR format is to accurately and efficiently represent high-dynamic-range scene-linear image data and associated metadata, with strong support for multi-part, multi-channel use cases.

OpenEXR is widely used in host application software where accuracy is critical, such as photorealistic rendering, texture access, image compositing, deep compositing, and DI.
OpenEXR is a project of the [Academy Software Foundation](https://www.aswf.io/). The format and library were originally developed by Industrial Light & Magic and first released in 2003. Weta Digital, Walt Disney Animation Studios, Sony Pictures Imageworks, Pixar Animation Studios, DreamWorks, and other studios, companies, and individuals have made contributions to the code base.

OpenEXR is included in the [VFX Reference Platform](https://vfxplatform.com/).

The openexr crate is maintained by [the vfx-rs project](https://github.com/vfx-rs).

# Quick Start

To use the included C++ OpenEXR source:

```bash
cargo add openexr
cargo build
```

While this method is supported and easiest to get starter, it is strongly 
recommended that you build and install the C++ library separately and build
the crate against it like so:

```bash
cargo add openexr
CMAKE_PREFIX_PATH=/path/to/cmake/configs cargo build
```

Note that you must take care to ensure that the version of OpenEXR you are
pointing it to is the same as that for this version of the crate, otherwise
you will encounter linker errors since all OpenEXR symbols are versioned.

The [`prelude`](crate::prelude) pulls in the set of types that you
need for basic file I/O of RGBA and arbitrary channel images:

```no_run
use openexr::prelude::*;

fn write_rgba1(filename: &str, pixels: &[Rgba], width: i32, height: i32)
-> Result<(), Box<dyn std::error::Error>> {
    let header = Header::from_dimensions(width, height);
    let mut file = RgbaOutputFile::new(
        filename,
        &header,
        RgbaChannels::WriteRgba,
        1,
    )?;

    file.set_frame_buffer(&pixels, 1, width as usize)?;
    file.write_pixels(height)?;

    Ok(())
}

fn read_rgba1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use imath_traits::Zero;

    let mut file = RgbaInputFile::new(path, 1).unwrap();
    // Note that windows in OpenEXR are ***inclusive*** bounds, so a
    // 1920x1080 image has window [0, 0, 1919, 1079].
    let data_window: [i32; 4] = *file.header().data_window();
    let width = data_window.width() + 1;
    let height = data_window.height() + 1;

    let mut pixels = vec![Rgba::zero(); (width * height) as usize];
    file.set_frame_buffer(&mut pixels, 1, width as usize)?;
    file.read_pixels(0, height - 1)?;

    Ok(())
}
```

Beyond that, types related to deep images are in the [`deep`](crate::deep)
module, and tiled images are in the [`tiled`](crate::tiled) module.

The [Reading and Writing OpenEXR Image Files](crate::doc::reading_and_writing_image_files)
document is a great place to start to explore the full functionality of the
crate. It contains example usage for nearly everything.

# Math Crate Interoperability
OpenEXR (and much of the rest of the VFX ecosystem) relies on Imath for basic
math primitives like vectors and bounding boxes.

Rust already has several mature crates for linear algebra targetting graphics
such as [cgmath](https://crates.io/crates/cgmath), [nalgebra](https://crates.io/crates/nalgebra), [nalgebra-glm](https://crates.io/crates/nalgebra-glm) and [glam](https://crates.io/crates/glam). Rather than adding yet another
contender to this crowded field, we instead provide a set of traits that allow
any of these crates to be used with openexr in the form of [imath-traits](https://crates.io/crates/imath-traits). By default, these traits are implemented for arrays and slices, so you will find that the examples in this documentation will tend to use e.g. `[i32; 4]` for bounding boxes:

```no_run
# use openexr::prelude::*;
# fn read_rgba1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
#   use imath_traits::Zero;
    let mut file = RgbaInputFile::new(path, 1).unwrap();
    let data_window = file.header().data_window::<[i32; 4]>().clone();
    let width = data_window.width() + 1;
    let height = data_window.height() + 1;
#    Ok(())
# }
```

To use your preffered math crate instead, simply enable the corresponding feature on openexr,
which will be `imath_<name>`, for example:

```bash
cargo build --features=imath_cgmath
```

Now you can use types from that crate together with openexr seamlessly. In
the case that the math crate does not provide a bounding box type, one will
be available as `imath_traits::Box2i` and `imath_traits::Box3i`.

```no_run
# use openexr::prelude::*;
#[cfg(feature = "imath_cgmath")]
# fn read_rgba1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
#   use imath_traits::Zero;
    use imath_traits::Box2i;

    let mut file = RgbaInputFile::new(path, 1).unwrap();
    let data_window: Box2i = *file.header().data_window();
    let width = data_window.width() + 1;
    let height = data_window.height() + 1;
#    Ok(())
# }
```

# Features
* High dynamic range and color precision.
* Support for 16-bit floating-point, 32-bit floating-point, and 32-bit integer pixels.
* Multiple image compression algorithms, both lossless and lossy. Some of the included codecs can achieve 2:1 lossless compression ratios on images with film grain. The lossy codecs have been tuned for visual quality and decoding performance.
* Extensibility. New image attributes (strings, vectors, integers, etc.) can be added to OpenEXR image headers without affecting backward compatibility with existing OpenEXR applications.
* Support for stereoscopic image workflows and a generalization to multi-views.
* Flexible support for deep data: pixels can store a variable-length list of samples and, thus, it is possible to store multiple values at different depths for each pixel. Hard surfaces and volumetric data representations are accommodated.
* Multipart: ability to encode separate, but related, images in one file. This allows for access to individual parts without the need to read other parts in the file.

# Long-Form Documentation

The following documents give a more in-depth view of different parts of the
openexr crate:

* [Reading and Writing Image Files](crate::doc::reading_and_writing_image_files) - A
tutorial-style guide to the main image reading and writing interfaces.
* [Technical Introduction](crate::doc::technical_introduction) - A technical overview of the
OpenEXR format and its related concepts.
* [Interpreting Deep Pixels](crate::doc::interpreting_deep_pixels) - An in-depth look at how
deep pixels are stored and how to manipulate their samples.
* [Multi-View OpenEXR](crate::doc::multi_view_open_exr) - Representation of multi-view images
in OpenEXR files.

# Building the documentation

To build the full documentation including long-form docs and KaTeX equations, use the following
command:

```bash
cargo +nightly doc --no-deps --features=long-form-docs
```
Note this is done automatically for docs.rs when publishing.

To run the doctests in the long-form docs (i.e. make sure the code examples
compile correctly) run:
```bash
cargo +nightly test --features=long-form-docs
```

This should no longer be necessary once Rust 1.54 is released.

