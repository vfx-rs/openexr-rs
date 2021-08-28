# Reading and Writing OpenEXR Image Files.

-   [Document Purpose and Audience](#document-purpose-and-audience)
-   [Scan Line Based and Tiled OpenEXR
    files](#scan-line-based-and-tiled-openexr-files)
    -   [Multi-Part and Deep Data](#multi-part-and-deep-data)
-   [Using the RGBA-only Interface for Scan Line Based
    Files](#using-the-rgba-only-interface-for-scan-line-based-files)
    -   [Writing an RGBA Image File](#writing-an-rgba-image-file)
    -   [Writing a Cropped RGBA Image](#writing-a-cropped-rgba-image)
    -   [Storing Custom Attributes](#storing-custom-attributes)
    -   [Reading an RGBA Image File](#reading-an-rgba-image-file)
    -   [Reading an RGBA Image File in
        Chunks](#reading-an-rgba-image-file-in-chunks)
    -   [Reading Custom Attributes](#reading-custom-attributes)
    -   [Luminance/Chroma and Gray-Scale
        Images](#luminancechroma-and-gray-scale-images)
-   [Using the General Interface for Scan Line Based
    Files](#using-the-general-interface-for-scan-line-based-files)
    -   [Writing an Image File](#writing-an-image-file)
    -   [Writing a Cropped Image](#writing-a-cropped-image)
    -   [Reading an Image File](#reading-an-image-file)
    -   [Interleaving Image Channels in the Frame
        Buffer](#interleaving-image-channels-in-the-frame-buffer)
    -   [Which Channels are in a File?](#which-channels-are-in-a-file)
    -   [Layers](#layers)
-   [Tiles, Levels and Level Modes](#tiles-levels-and-level-modes)
-   [Using the RGBA-only Interface for Tiled
    Files](#using-the-rgba-only-interface-for-tiled-files)
    -   [Writing a Tiled RGBA Image File with One Resolution
        Level](#writing-a-tiled-rgba-image-file-with-one-resolution-level)
    -   [Writing a Tiled RGBA Image File with Mipmap
        Levels](#writing-a-tiled-rgba-image-file-with-mipmap-levels)
    -   [Writing a Tiled RGBA Image File with Ripmap
        Levels](#writing-a-tiled-rgba-image-file-with-ripmap-levels)
    -   [Reading a Tiled RGBA Image
        File](#reading-a-tiled-rgba-image-file)
-   [Using the General Interface for Tiled
    Files](#using-the-general-interface-for-tiled-files)
    -   [Writing a Tiled Image File](#writing-a-tiled-image-file)
    -   [Reading a Tiled Image File](#reading-a-tiled-image-file)
-   [Deep Data Files (New in 2.0)](#deep-data-files-new-in-20)
    -   [Writing a Deep Scan Line File](#writing-a-deep-scan-line-file)
    -   [Reading a Deep Scan Line File](#reading-a-deep-scan-line-file)
    -   [Writing a Deep Tiled File](#writing-a-deep-tiled-file)
    -   [Reading a Deep Tiled File](#reading-a-deep-tiled-file)
-   [Threads](#threads)
    -   [Library Thread-Safety](#library-thread-safety)
    -   [Multithreaded I/O](#multithreaded-io)
    -   [Multithreaded I/O, Multithreaded Application
        Program](#multithreaded-io-multithreaded-application-program)
<!---
-   [Low-Level I/O](#low-level-io)
    -   [Custom Low-Level File I/O](#custom-low-level-file-io)
    -   [Memory-Mapped I/O](#memory-mapped-io)
-->
-   [Miscellaneous](#miscellaneous)
    -   [Is this an OpenEXR File?](#is-this-an-openexr-file)
    -   [Is this File Complete?](#is-this-file-complete)
    -   [Preview Images](#preview-images)
    -   [Environment Maps](#environment-maps)

# Document Purpose and Audience

This document shows how to write Rust code that reads and writes OpenEXR
3.0 image files.

The text assumes that the reader is familiar with OpenEXR terms like
"channel", "attribute", "data window" or "deep data". For an explanation
of those terms see the [Technical Introduction to OpenEXR](crate::doc::technical_introduction) document.

Most of the code examples below are standalone examples in the `examples` directory and can be run as usual.

A description of the file structure and format is provided in *OpenEXR
File Layout*.

# Scan Line Based and Tiled OpenEXR files

In an OpenEXR file, pixel data can be stored either as scan lines or as
tiles. Files that store pixels as tiles can also store multi-resolution
images. For each of the two storage formats (scan line or tile-based),
the IlmImf library supports two reading and writing interfaces:

1.  The first, fully general, interface allows access to arbitrary
    channels, and supports many different in-memory pixel data layouts.
2.  The second interface is easier to use, but limits access to f16 RGBA channels, and provides fewer
    options for laying out pixels in memory.

The interfaces for reading and writing OpenEXR files are implemented in
the following eight structs:

|                    | Tiles                                                                | Scan lines                                           | Scan lines and tiles                               |
|--------------------|----------------------------------------------------------------------|------------------------------------------------------|----------------------------------------------------|
| Arbitrary channels | [`TiledInputFile`](crate::tiled::tiled_input_file::TiledInputFile)          |                                                      | [`InputFile`](crate::core::input_file::InputFile)        |
|                    | [`TiledOutputFile`](crate::tiled::tiled_output_file::TiledOutputFile)       | [`OutputFile`](crate::core::output_file::OutputFile)       |                                                    |
| RGBA only          | [`TiledRgbaInputFile`](crate::tiled::tiled_rgba_file::TiledRgbaInputFile)   |                                                      | [`RgbaInputFile`](crate::rgba::rgba_file::RgbaInputFile) |
|                    | [`TiledRgbaOutputFile`](crate::tiled::tiled_rgba_file::TiledRgbaOutputFile) | [`RgbaOutputFile`](crate::rgba::rgba_file::RgbaOutputFile) |                                                    |

The classes for reading scan line based images ([`InputFile`](crate::core::input_file::InputFile) and
[`RgbaInputFile`](crate::core::rgba_file::RgbaInputFile)) can also be used to read tiled image files. This way,
programs that do not need support for tiled or multi-resolution images
can always use the rather straightforward scan line interfaces, without
worrying about complications related to tiling and multiple resolutions.
When a multi-resolution file is read via a scan line interface, only the
highest-resolution version of the image is accessible.

## Multi-Part and Deep Data 

The procedure for writing multi-part and deep data files is similar to
writing scan line and tile. Though there is no simplified interface,
such as the RGBA-only interface.

This table describes the significant differences between writing
single-part scan line and tile files and writing multi-part and deep
data files.

<table>
<colgroup>
<col style="width: 33%" />
<col style="width: 33%" />
<col style="width: 33%" />
</colgroup>
<tbody>
<tr class="odd odd">
<td>Feature</td>
<td>scan line and tile</td>
<td>Multi-part and deep data</td>
</tr>
<tr class="even even">
<td>Channel names may be reserved</td>
<td>Some channel names are reserved in practice, but were never formally defined.</td>
<td><p>Channel name “sample count” is reserved for a pixel sample count slice in frame buffer.</p>
<p><strong>Note:</strong> The name “sample count” (all lowercase) is subject to change.</p></td>
</tr>
<tr class="odd odd">
<td>Multiple parts</td>
<td>Single-part format is intended for storing a single multichannel image</td>
<td><p>Multi-part files support multiple independent parts. This allows storing multiple views in the same file for stereo images, storing multiple resolutions in different parts. It is possible to include one or more scan line, tile, deep scan line or deep tile format images within a multi-part file.</p>
<p>Custom data formats can also be used to store additional parts, but this is outside the scope of this document.</p></td>
</tr>
<tr class="even even">
<td>Backwards-compatible low-level io available</td>
<td>The new formats share the same abstract low-level IO as OpenEXR 1.7. It is therefore possible to use the same libraries to implement low level IO to read both formats.</td>
<td></td>
</tr>
</tbody>
</table>

# Using the RGBA-only Interface for Scan Line Based Files

## Writing an RGBA Image File

Writing a simple RGBA image file is fairly straightforward:

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
```

[`RgbaOutputFile`]: crate::rgba::rgba_file::RgbaOutputFile
[`Rgba`]: crate::rgba::rgba::Rgba

Construction of an [`RgbaOutputFile`] object, in line 1, creates an OpenEXR
header, sets the header\'s attributes, opens the file with the specified
name, and stores the header in the file. The header\'s display window
and data window are both set to `(0, 0) - (width-1, height-1)`. The
channel list contains four channels, R, G, B, and A, of type HALF (`f16`).

Line 2 specifies how the pixel data are laid out in memory. In our
example, `pixels` is a slice of [`Rgba`] of length `width * height`.

The elements of our slice are arranged so that the pixels of each scan
line are contiguous in memory. The  [`set_frame_buffer()`](crate::rgba::rgba_file::RgbaOutputFile::set_frame_buffer) method takes
three arguments, `pixels`, `x_stride`, and `y_stride`. To find the address
of pixel `(x, y)`, the [`RgbaOutputFile`] object computes

```c
pixels[x * x_stride + y * y_sride]
```

In this case, `x_stride` and `y_stride` are set to `1` and `width`, respectively, indicating that pixel `(x, y)` can be found at index

```c
pixels[1 * x + width * y]
```

The call to [`write_pixels()`](crate::rgba::rgba_file::RgbaOutputFile::write_pixels) in line 3, copies the image\'s pixels from
memory to the file. The argument to [`write_pixels()`](crate::rgba::rgba_file::RgbaOutputFile::write_pixels), `height`, specifies
how many scan lines worth of data are copied.

Finally, returning from function `write_rgba1()` drops the local [`RgbaOutputFile`] object, thereby closing the file.

Why do we have to tell the [`write_pixels()`](crate::rgba::rgba_file::RgbaOutputFile::write_pixels) function how many scan lines
we want to write? Shouldn\'t the [`RgbaOutputFile`] object be able to
derive the number of scan lines from the data window? OpenEXR
doesn\'t require writing all scan lines with a single [`write_pixels()`](crate::rgba::rgba_file::RgbaOutputFile::write_pixels)
call. Many programs want to write scan lines individually, or in small
blocks. For example, rendering computer-generated images can take a
significant amount of time, and many rendering programs want to store
each scan line in the image file as soon as all of the pixels for that
scan line are available. This way, users can look at a partial image
before rendering is finished. The OpenEXR crate allows writing the scan
lines in top-to-bottom or bottom-to-top direction. The direction is
defined by the file header\'s line order attribute ([`LineOrder::IncreasingY`](crate::core::LineOrder) or
[`LineOrder::DecreasingY`](crate::core::LineOrder)). By default, scan lines are written top to bottom
([`LineOrder::IncreasingY`](crate::core::LineOrder)).

## Writing a Cropped RGBA Image

Now we are going to store a cropped image in a file. For this example,
we assume that we have a frame buffer that is large enough to hold an
image with `width` by `height` pixels, but only part of the frame buffer
contains valid data. In the file\'s header, the size of the whole image
is indicated by the display window, `(0, 0) - (width-1, height-1)`, and
the data window specifies the region for which valid pixel data exist.
Only the pixels in the data window are stored in the file.

```no_run
use openexr::prelude::*;

fn write_rgba2(
    filename: &str, 
    pixels: &[Rgba], 
    data_window: &[i32; 4],
    display_window: &[i32; 4],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut header = Header::from_windows(
        *data_window,    
        *display_window,
    );

    let mut file = RgbaOutputFile::new(
        filename,
        &header,
        RgbaChannels::WriteRgba,
        1,
    )?;

    file.set_frame_buffer(&pixels, 1, (data_window.width() + 1) as usize)?;
    file.write_pixels(data_window.height() + 1)?;

    Ok(())
}
```

The code above is similar to that in [Writing an RGBA Image File](#writing-an-rgba-image-file), where the whole image was stored in the file. Two things are different, however: When the [`RgbaOutputFile`] object is created, the data window and the display window are explicitly specified rather than being derived from the image's width and height. The number of scan lines stored in the file by [`write_pixels()`](crate::rgba::rgba_file::RgbaOutputFile::write_pixels) is equal to the height of the data window instead of the height of the whole image. Since we are using the default [`LineOrder::IncreasingY`](crate::core::LineOrder) direction for storing the scan lines in the file, [`write_pixels()`](crate::rgba::rgba_file::RgbaOutputFile::write_pixels) starts at the top of the data window, at y coordinate `data_window.min.y`, and proceeds toward the bottom, at y coordinate `data_window.max.y`.

Even though we are storing only part of the image in the file, the frame buffer is still large enough to hold the whole image. In order to save memory, a smaller frame buffer could have been allocated, just big enough to hold the contents of the data window. Assuming that the pixels were still stored in contiguous scan lines, with the *pixels* pointer pointing to the pixel at the upper left corner of the data window, at coordinates *(dataWindow.min.x, dataWindow.min.y)*, the arguments to the *setFrameBuffer()* call would have to be to be changed as follows:

int dwWidth = dataWindow.max.x - dataWindow.min.x + 1;

file.setFrameBuffer

(pixels - dataWindow.min.x - dataWindow.min.y \* dwWidth, 1, dwWidth);

With these settings, evaluation of

base + x \* xStride + y \* yStride

for pixel *(dataWindow.min.x, dataWindow.min.y)* produces

pixels - dataWindow.min.x - dataWindow.min.y \* dwWidth

\+ dataWindow.min.x \* 1

\+ dataWindow.min.y \* dwWidth

= pixels -

\- dataWindow.min.x

\- dataWindow.min.y \* (dataWindow.max.x - dataWindow.min.x + 1)

\+ dataWindow.min.x

\+ dataWindow.min.y \* (dataWindow.max.x - dataWindow.min.x + 1)

= pixels,

which is exactly what we want. Similarly, calculating the addresses for
pixels *(dataWindow.min.x+1, dataWindow.min.y)* and *(dataWindow.min.x,
dataWindow.min.y+1)* yields *pixels+1* and *pixels+dwWidth*,
respectively.

## Storing Custom Attributes

We will now to store an image in a file, and we will add two extra
attributes to the image file header: a string, called `"comments"`, and
a 4×4 matrix, called `"cameraTransform"`.

```no_run
use openexr::prelude::*;
use openexr::core::attribute::{CppStringAttribute, M44fAttribute};

fn write_rgba3(
    pixels: &[Rgba], 
    width: i32, 
    height: i32, 
    comments: &str, 
    xform: &[f32; 16]
) -> Result<(), Box<dyn std::error::Error>> {
    let mut header = Header::from_dimensions(width, height);
    header.insert("comments", &CppStringAttribute::from_value(comments))?;
    header.insert("cameraTransform", &M44fAttribute::from_value(xform))?;

    let mut file = RgbaOutputFile::new(
        "write_rgba1.exr",
        &header,
        RgbaChannels::WriteRgba,
        1,
    )?;

    file.set_frame_buffer(&pixels, 1, width as usize)?;
    file.write_pixels(height)?;

    Ok(())
}
```

In order to make it easier to exchange data between programs written by
different people, the IlmImf library defines a set of standard
attributes for commonly used data, such as colorimetric information,
time and place where an image was recorded, or the owner of an image
file\'s content. For the current list of standard attributes, see the
[`standard_attributes`](crate::core::standard_attributes) module. The list is expected to grow over
time as OpenEXR users identify new types of data they would like to
represent in a standard format. If you need to store some piece of
information in an OpenEXR file header, it is probably a good idea to
check if a suitable standard attribute exists, before you define a new
attribute.

## Reading an RGBA Image File

Reading an RGBA image is almost as easy as writing one:

```no_run
use openexr::prelude::*;

fn read_rgba1(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use imath_traits::Zero;

    let mut file = RgbaInputFile::new(path, 1).unwrap();
    let data_window = file.header().data_window::<[i32; 4]>().clone();
    let width = data_window[2] - data_window[0] + 1;
    let height = data_window[3] - data_window[1] + 1;

    let mut pixels = vec![Rgba::zero(); (width * height) as usize];
    file.set_frame_buffer(&mut pixels, 1, width as usize)?;
    file.read_pixels(0, height - 1)?;

    Ok(())
}
```
[`RgbaInputFile`]: crate::rgba::rgba_file::RgbaInputFile

Constructing an [`RgbaInputFile`] object, passing the name of the file to
the constructor, opens the file and reads the file\'s header.

After asking the [`RgbaInputFile`] object for the file\'s data window, we
allocate a buffer for the pixels. The number of scan lines in the buffer is equal
to the height of the data window, and the number of pixels per scan line
is equal to the width of the data window. The pixels are represented as
[`Rgba`] structs.

Note that we ignore the display window in this example; in a program
that wanted to place the pixels in the data window correctly in an
overall image, the display window would have to be taken into account.

Just as for writing a file, calling [`RgbaInputFile`](crate::rgba::rgba_file::RgbaInputFile::set_frame_buffer) tells the
[`RgbaInputFile`] object how to access individual pixels in the buffer.
(See also [Writing a Cropped RGBA
Image](#writing-a-cropped-rgba-image).

Calling [`read_pixels()`](crate::rgba::rgba_file::RgbaInputFile::read_pixels) copies the pixel data from the file into the
buffer. If one or more of the R, G, B, and A channels are missing in the
file, the corresponding field in the pixels is filled with an
appropriate default value. The default value for R, G and B is 0.0, or
black; the default value for A is 1.0, or opaque.

Finally, returning from function `read_rgba1()` drops the local
[`RgbaInputFile`] object, thereby closing the file.

Unlike the [`RgbaOutputFile`]'s [`write_pixels()`](crate::rgba::rgba_file::RgbaOutputFile::write_pixels) method, [`read_pixels()`](crate::rgba::rgba_file::RgbaInputFile::read_pixels)
has two arguments. Calling `read_pixels(y1, y2)` copies the pixels for all
scan lines with y coordinates from `y1` to `y2` into the frame buffer.
This allows access to the the scan lines in any order. The image can be
read all at once, one scan line at a time, or in small blocks of a few
scan lines. It is also possible to skip parts of the image.

Note that even though random access is possible, reading the scan lines
in the same order as they were written, is more efficient. Random access
to the file requires seek operations, which tend to be slow. Calling the
[`RgbaInputFile`]'s [`line_order`](crate::rgba::rgba_file::RgbaInputFile::line_order) method returns the order in which the
scan lines in the file were written ([`LineOrder::IncreasingY`](crate::core::LineOrder) or [`LineOrder::DecreasingY`](crate::core::LineOrder)).
If successive calls to [`read_pixels()`](crate::rgba::rgba_file::RgbaInputFile::read_pixels) access the scan lines in the right
order, the OpenEXR reads the file as fast as possible, without seek operations.

<!---
## Reading an RGBA Image File in Chunks

The following shows how to read an RGBA image in blocks of a few scan
lines. This is useful for programs that want to process high-resolution
images without allocating enough memory to hold the complete image.
These programs typically read a few scan lines worth of pixels into a
memory buffer, process the pixels, and store them in another file. The
buffer is then re-used for the next set of scan lines. Image operations
like color-correction or compositing (\"A over B\") are very easy to do
incrementally this way. With clever buffering of a few extra scan lines,
incremental versions of operations that require access to neighboring
pixels, like blurring or sharpening, are also possible.

void

readRgba2 (const char fileName\[\])

{

RgbaInputFile file (fileName);

Box2i dw = file.dataWindow();

int width = dw.max.x - dw.min.x + 1;

int height = dw.max.y - dw.min.y + 1;

Array2D\<Rgba> pixels (10, width);

while (dw.min.y \<= dw.max.y)

{

file.setFrameBuffer (&pixels\[0\]\[0\] - dw.min.x - dw.min.y \* width,

1, width);

file.readPixels (dw.min.y, min (dw.min.y + 9, dw.max.y));

// processPixels (pixels)

dw.min.y += 10;

}

}

Again, we open the file and read the file header by constructing an
*RgbaInputFile* object. Then we allocate a memory buffer that is just
large enough to hold ten complete scan lines. We call *readPixels()* to
copy the pixels from the file into our buffer, ten scan lines at a time.
Since we want to re-use the buffer for every block of ten scan lines, we
have to call *setFramebuffer()* before each *readPixels()* call, in
order to associate memory address *&pixels\[0\]\[0\]* first with pixel
coordinates *(dw.min.x, dw.min.y)*, then with *(dw.min.x, dw.min.y+10)*,
*(dw.min.x, dw.min.y+20)* and so on.

-->

## Reading Custom Attributes

In [Storing Custom Attributes](#storing-custom-attributes), we showed how to store custom
attributes in the image file header. Here we show how to test whether a
given file\'s header contains particular attributes, and how to read
those attributes\' values.

```no_run
use openexr::prelude::*;

fn read_header(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = RgbaInputFile::new(filename, 1)?;

    if let Some(attr) = file.header().find_typed_attribute_string("comments") {
        println!("comments: {}", attr.value())
    }

    if let Some(attr) = file.header().find_typed_attribute_m44f("cameraTransform") {
        println!("cameraTransform: {:?}", attr.value::<[f32; 16]>())
    }

    Ok(())
}
```

As usual, we open the file by constructing an [`RgbaInputFile`] object.
Calling [`find_typed_attribute_X(n)`](crate::core::header::Header::find_typed_attribute_string) searches the header for an attribute
with type `X` and name `n`. If a matching attribute is found,
[`find_typed_attribute_X(n)`](crate::core::header::Header::find_typed_attribute_string) returns [`Some`] containing a reference to the attribute. If the header
contains no attribute with name `n`, or if the header contains an
attribute with name `n`, but the attribute\'s type is not `X`,
[`find_typed_attribute_X(n)`](crate::core::header::Header::find_typed_attribute_string) returns [`None`]. Once we have references to the attributes
we were looking for, we can access their values by calling the
attributes\' [`value()`](crate::core::attribute::CppStringAttribute::value) methods.


## Luminance/Chroma and Gray-Scale Images

Writing an RGBA image file usually preserves the pixels without losing
any data; saving an image file and reading it back does not alter the
pixels\' R, G, B and A values. Most of the time, lossless data storage
is exactly what we want, but sometimes file space or transmission
bandwidth are limited, and we would like to reduce the size of our image
files. It is often acceptable if the numbers in the pixels change
slightly as long as the image still looks just like the original.

The RGBA interface supports storing RGB data in
luminance/chroma format. The R, G, and B channels are converted into a
luminance channel, Y, and two chroma channels, RY and BY. The Y channel
represents a pixel\'s brightness, and the two chroma channels represent
its color. The human visual system\'s spatial resolution for color is
much lower than the spatial resolution for brightness. This allows us to
reduce the horizontal and vertical resolution of the RY and BY channels
by a factor of two. The visual appearance of the image doesn\'t change,
but the image occupies only half as much space, even before data
compression is applied. (For every four pixels, we store four Y values,
one RY value, and one BY value, instead of four R, four G, and four B
values.)

When opening a file for writing, a program can select how it wants the
pixels to be stored. The constructors for [`RgbaOutputFile`] have an
`rgba_channels` argument, which determines the set of channels in the
file:

|    Value                    | Channels written          |
|-----------------------------|---------------------------|
| [`RgbaChannels::WriteRgba`] |  red, green, blue, alpha  |
| [`RgbaChannels::WriteYc`]   |  luminance, chroma        |
| [`RgbaChannels::WriteYca`]  |  luminance, chroma, alpha |
| [`RgbaChannels::WriteY`]    |  luminance only           |
| [`RgbaChannels::WriteYa`]   |  luminance, alpha         |

[`RgbaChannels::WriteRgba`]: crate::rgba::rgba::RgbaChannels::WriteRgba
[`RgbaChannels::WriteYc`]: crate::rgba::rgba::RgbaChannels::Yc
[`RgbaChannels::WriteYca`]: crate::rgba::rgba::RgbaChannels::Yca
[`RgbaChannels::WriteY`]: crate::rgba::rgba::RgbaChannels::Y
[`RgbaChannels::WriteYa`]: crate::rgba::rgba::RgbaChannels::Ya

[`RgbaChannels::WriteY`] and [`RgbaChannels::WriteYa`] provide an efficient way to store gray-scale
images. The chroma channels for a gray-scale image contain only zeroes,
so they can be omitted from the file.

When an image file is opened for reading, class [`RgbaInputFile`]
automatically detects luminance/chroma images and converts the pixels
back to RGB format.

# Using the General Interface for Scan Line Based Files

## Writing an Image File

This example demonstrates how to write an OpenEXR image file with two
channels: one channel, of type [`PixelType::Half`], is called G, and the other, of
type [`PixelType::Float`], is called Z. The size of the image is `width` by `height`
pixels. The data for the two channels are supplied in two separate
buffers, `g` and `z`. Within each buffer, the pixels of each
scan line are contiguous in memory.

[`PixelType`]: crate::core::PixelType
[`PixelType::Half`]: crate::core::PixelType::Half
[`PixelType::Float`]: crate::core::PixelType::Float

```no_run
use openexr::prelude::*;
use half::f16;

fn write_gz1(
    filename: &str,
    g: &[f16],
    z: &[f32],
    width: i32,
    height: i32,
) -> Result<(), Box<dyn std::error::Error>> {

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
        .x_stride(std::mem::size_of::<f16>())
        .build()?
    )?;

    frame_buffer.insert(
        "Z",
        &Slice::builder(
            PixelType::Float,
            z.as_ptr() as *const u8,
            width as i64,
            height as i64,
        )
        .x_stride(std::mem::size_of::<f32>())
        .build()?
    )?;

    let mut file = OutputFile::new("write_gz1.exr", &header, 1).unwrap();
    file.set_frame_buffer(&frame_buffer).unwrap();
    unsafe { file.write_pixels(height).unwrap() };

    Ok(())
}
```

First, an OpenEXR header is created, and the header\'s display
window and data window are both set to `(0, 0) - (width-1, height-1)` by the [`with_dimensions`](crate::core::header::Header::with_dimensions) constructor.

Next the names and types of the channels that are to be stored are specified on the header.

[`FrameBuffer`]: crate::core::frame_buffer::FrameBuffer
[`Slice`]: crate::core::frame_buffer::Slice
[`SliceBuilder`]: crate::core::frame_buffer::SliceBuilder
[`OutputFile`]: crate::core::output_file::OutputFile

Then a [`FrameBuffer`] is constructed and two [`Slice`]s are added to it. [`Slice`] describes the memory layout of a single channel. The constructor takes as arguments the [`PixelType`] of the [`Slice`], a pointer to the start of the [`Slice`]'s data window, and the width and height of the [`Slice`]. No further arguments are required in this example so the [`Slice`] is built immediately.

Note that unlike the C++ API, the strides of the pixels are automatically computed from the pixel type. You only need to specify the [`x_stride`](crate::core::frame_buffer::SliceBuilder::x_stride) and/or [`y_stride`](crate::core::frame_buffer::SliceBuilder::y_stride) if your pixels are not densely packed elements of the specified type (e.g. you are passing an RGBA slice and ony want to write out the R channel).

Next, constructing an [`OutputFile`] object opens the file with the
specified name, and stores the header in the file.

The [`write_pixels()`](crate::core::output_file::OutputFile::write_pixels) call copies the image's pixels from
memory into the file. As in the RGBA-only interface, the argument to
[`write_pixels()`](crate::core::output_file::OutputFile::write_pixels) specifies how many scan lines are copied into the file.
(See [Writing an RGBA Image File](#writing-and-rgba-image-file))

If the image file contains a channel for which the [`FrameBuffer`] object
has no corresponding [`Slice`], then the pixels for that channel in the
file are filled with zeroes. If the [`FrameBuffer`] object contains a
[`Slice`] for which the file has no channel, then the [`Slice`] is ignored.

Returning from function `write_gz1()` destroys the local [`OutputFile`]
object and closes the file.

## Writing a Cropped Image

Writing a cropped image using the general interface is analogous to
writing a cropped image using the RGBA-only interface, as shown in
[Writing a Cropped RGBA Image](#writing-a-cropped-rgba-image). In the file's header the data
window is set explicitly instead of being generated automatically from
the image's width and height. The number of scan lines that are stored
in the file is equal to the height of the data window, instead of the
height of the entire image. As in [Writing a Cropped RGBA Image](#writing-a-cropped-rgba-image), the example code below assumes
that the memory buffers for the pixels are large enough to hold `width`
by `height` pixels, but only the region that corresponds to the data
window will be stored in the file. For smaller memory buffers with room
only for the pixels in the data window, the `base`, `x_stride()` and
`y_stride()` arguments for the [`FrameBuffer`] object's [`Slice`]s would have to
be adjusted accordingly. (Again, see [Writing a Cropped RGBA Image](#writing-a-cropped-rgba-image).

```no_run
use openexr::prelude::*;
use half::f16;

fn write_gz2(
    filename: &str,
    g: &[f16],
    z: &[f32],
    width: i32,
    height: i32,
    data_window: &[i32; 4],
) -> Result<(), Box<dyn std::error::Error>> {

    let mut header = Header::from_dimensions(width, height);
    header.channels_mut().insert("G", &CHANNEL_HALF);
    header.channels_mut().insert("Z", &CHANNEL_FLOAT);
    *header.data_window_mut() = *data_window;

    let mut frame_buffer = FrameBuffer::new();

    frame_buffer.insert(
        "G",
        &Slice::builder(
            PixelType::Half,
            g.as_ptr() as *const u8,
            width as i64,
            height as i64,
        )
        .x_stride(std::mem::size_of::<f16>())
        .build()?
    )?;

    frame_buffer.insert(
        "Z",
        &Slice::builder(
            PixelType::Float,
            z.as_ptr() as *const u8,
            width as i64,
            height as i64,
        )
        .x_stride(std::mem::size_of::<f32>())
        .build()?
    )?;

    let mut file = OutputFile::new("write_gz1.exr", &header, 1).unwrap();
    file.set_frame_buffer(&frame_buffer).unwrap();
    unsafe { file.write_pixels(data_window[3] - data_window[1] + 1).unwrap() };

    Ok(())
}

```

## Reading an Image File

[`InputFile`]: crate::core::input_file::InputFile

In this example, we read an OpenEXR image file using the [`InputFile`] general interface. We assume that the file contains two
channels, R, and G, of type [`PixelType::Half`], and one channel, Z, of type [`PixelType::Float`].
If one of those channels is not present in the image file, the
corresponding memory buffer for the pixels will be filled with an
appropriate default value.

void

readGZ1 (const char fileName\[\],

Array2D\<half> &rPixels,

Array2D\<half> &gPixels,

Array2D\<float> &zPixels,

int &width, int &height)

{

InputFile file (fileName);

Box2i dw = file.header().dataWindow();

width = dw.max.x - dw.min.x + 1;

height = dw.max.y - dw.min.y + 1;

rPixels.resizeErase (height, width);

gPixels.resizeErase (height, width);

zPixels.resizeErase (height, width);

FrameBuffer frameBuffer;

frameBuffer.insert (\"R\", // name

Slice (HALF, // type

(char \*) (&rPixels\[0\]\[0\] - // base

dw.min.x -

dw.min.y \* width),

sizeof (rPixels\[0\]\[0\]) \* 1, // xStride

sizeof (rPixels\[0\]\[0\]) \* width,// yStride

1, 1, // x/y sampling

0.0)); // fillValue

frameBuffer.insert (\"G\", // name

Slice (HALF, // type

(char \*) (&gPixels\[0\]\[0\] - // base

dw.min.x -

dw.min.y \* width),

sizeof (gPixels\[0\]\[0\]) \* 1, // xStride

sizeof (gPixels\[0\]\[0\]) \* width,// yStride

1, 1, // x/y sampling

0.0)); // fillValue

frameBuffer.insert (\"Z\", // name

Slice (FLOAT, // type

(char \*) (&zPixels\[0\]\[0\] - // base

dw.min.x -

dw.min.y \* width),

sizeof (zPixels\[0\]\[0\]) \* 1, // xStride

sizeof (zPixels\[0\]\[0\]) \* width,// yStride

1, 1, // x/y sampling

FLT_MAX)); // fillValue

file.setFrameBuffer (frameBuffer);

file.readPixels (dw.min.y, dw.max.y);

}

First, we open the file with the specified name, by constructing an
*InputFile* object.

Using the *Array2D* class template, we allocate memory buffers for the
image\'s R, G and Z channels. The buffers are big enough to hold all
pixels in the file\'s data window.

Next, we create a *FrameBuffer* object, which describes our buffers to
the IlmImf library. For each image channel, we add a slice to the
*FrameBuffer*.

As usual, the slice\'s *type*, *xStride*, and *yStride* describe the
corresponding buffer\'s layout. For the R channel, pixel
*(dw.min.x, dw.min.y)* is at address *&rPixels\[0\]\[0\]*. By setting
the *type*, *xStride* and *yStride* of the corresponding *Slice* object
as shown above, evaluating

base + x \* xStride + y \* yStride

for pixel *(dw.min.x, dw.min.y)* produces

(char\*)(&rPixels\[0\]\[0\] - dw.min.x - dw.min.y \* width)

\+ dw.min.x \* sizeof (rPixels\[0\]\[0\]) \* 1

\+ dw.min.y \* sizeof (rPixels\[0\]\[0\]) \* width

= (char\*)&rPixels\[0\]\[0\]

\- dw.min.x \* sizeof (rPixels\[0\]\[0\])

\- dw.min.y \* sizeof (rPixels\[0\]\[0\]) \* width

\+ dw.min.x \* sizeof (rPixels\[0\]\[0\])

\+ dw.min.y \* sizeof (rPixels\[0\]\[0\]) \* width

= &rPixels\[0\]\[0\]*.*

The address calculations for pixels *(dw.min.x+1, dw.min.y)* and
*(dw.min.x, dw.min.y+1)* produce *&rPixels\[0\]\[0\]+1* and
*&rPixels\[0\]\[0\]+width*, which is equivalent to *&rPixels\[0\]\[1\]*
and *&rPixels\[1\]\[0\]*.

Each *Slice* has a *fillValue*. If the image file does not contain an
image channel for the *Slice*, then the corresponding memory buffer will
be filled with the *fillValue*.

The *Slice\'s* remaining two parameters, *xSampling* and *ySampling* are
used for images where some of the channels are subsampled, for instance,
the RY and BY channels in luminance/chroma images. (See
*[Luminance/Chroma and Gray-Scale
Images](#Luminance/Chroma%20and%20Gray-Scale%20Images)*, on page
[9](#Luminance/Chroma%20and%20Gray-Scale%20Images).) Unless an image
contains subsampled channels, *xSampling* and *ySampling* should always
be set to 1. For details see header files *ImfFrameBuffer.h* and
*ImfChannelList.h*.

After describing our memory buffers\' layout, we call *readPixels()* to
copy the pixel data from the file into the buffers. Just as with the
RGBA-only interface, *readPixels()* allows random-access to the scan
lines in the file. (See *[Reading an RGBA Image File in
Chunks](#Reading%20an%20RGBA%20Image%20File%20in%20Chunks)*, on page
[8](#Reading%20an%20RGBA%20Image%20File%20in%20Chunks).)

## Interleaving Image Channels in the Frame Buffer

Here is a variation of the previous example. We are reading an image
file, but instead of storing each image channel in a separate memory
buffer, we interleave the channels in a single buffer. The buffer is an
array of structs, which are defined like this:

typedef struct GZ

{

half g;

float z;

};

The code to read the file is almost the same as before; aside from
reading only two instead of three channels, the only difference is how
*base*, *xStride* and *yStride* for the *Slice*s in the *FrameBuffer*
object are computed:

void

readGZ2 (const char fileName\[\],

Array2D\<GZ> &pixels,

int &width, int &height)

{

InputFile file (fileName);

Box2i dw = file.header().dataWindow();

width = dw.max.x - dw.min.x + 1;

height = dw.max.y - dw.min.y + 1;

int dx = dw.min.x;

int dy = dw.min.y;

pixels.resizeErase (height, width);

FrameBuffer frameBuffer;

frameBuffer.insert (\"G\",

Slice (HALF,

(char \*) &pixels\[-dy\]\[-dx\].g,

sizeof (pixels\[0\]\[0\]) \* 1,

sizeof (pixels\[0\]\[0\]) \* width));

frameBuffer.insert (\"Z\",

Slice (FLOAT,

(char \*) &pixels\[-dy\]\[-dx\].z,

sizeof (pixels\[0\]\[0\]) \* 1,

sizeof (pixels\[0\]\[0\]) \* width));

file.setFrameBuffer (frameBuffer);

file.readPixels (dw.min.y, dw.max.y);

}

## Which Channels are in a File?

In functions `read_gz1()` and `read_gz2()`, above, we simply assumed that
the files we were trying to read contained a certain set of channels. We
relied on the IlmImf library to do \"something reasonable\" in case our
assumption was not true. Sometimes we want to know exactly what channels
are in an image file before reading any pixels, so that we can do what
we think is appropriate.

The file\'s header contains the file\'s channel list. Using iterators
similar to those in the C++ Standard Template Library, we can iterate
over the channels:

```no_run
use std::path::PathBuf;
use openexr::prelude::*;

// Open the `InputFile` and read the header
let file = InputFile::new("image.exr", 4).unwrap();

for (name, channel) in file.header().channels().iter() {
    println!("{}", name);
}

```

Channels can also be accessed by name with the [`ChannelList::get()`](crate::core::channel_list::ChannelList::get) method, which returns an [`Option`] indiciating if the channel is present in the header or not.

```no_run
use std::path::PathBuf;
use openexr::prelude::*;

// Open the `InputFile` and read the header
let file = InputFile::new("image.exr", 4).unwrap();

if let Some(channel) = file.header().channels().get("R") {
    println!("R channel type is {:?}", channel.type_);
}

```

## Layers

In an image file with many channels it is sometimes useful to group the
channels into *layers*, that is, into sets of channels that logically
belong together. Grouping channels into layers is done using a naming
convention: channel `C` in layer `L` is called `L.C`.

For example, a computer-generated picture of a 3D scene may contain a
separate set of R, G and B channels for the light that originated at
each one of the light sources in the scene. Every set of R, G, and B
channels is in its own layer. If the layers are called `light1`, `light2`,
`light3`, etc., then the full names of the channels in this image are
`light1.R`, `light1.G`, `light1.B`, `light2.R`, `light2.G`, `light2.B`, `light3.R`,
and so on.

Layers can be nested; for instance, `light1.specular.R` refers to the R
channel in the specular sub-layer of layer `light1`.

Channel names that do not contain a `.`, or that contain a `.` only
at the beginning or at the end are not considered to be part of any
layer.

[`ChannelList`]: crate::core::channel_list::ChannelList

Class [`ChannelList`] has two member functions that support per-layer
access to channels: [`layers()`](crate::core::channel_list::ChannelList::layers) returns the names of all layers in a
[`ChannelList`], and [`channels_in_layer()`](crate::core::channel_list::ChannelList::channels_in_layer) returns an iterator over the channels in the
corresponding layer.

The following code shows how to iterate over all the channels in a particular layer:

```no_run
use openexr::prelude::*;

let mut list = ChannelList::new();
let channel = Channel {
    type_: PixelType::Half.into(),
    x_sampling: 1,
    y_sampling: 1,
    p_linear: true,
};

list.insert("diffuse.R", &channel);
list.insert("diffuse.G", &channel);
list.insert("diffuse.B", &channel);

list.insert("specular.R", &channel);
list.insert("specular.G", &channel);
list.insert("specular.B", &channel);

assert_eq!(
    list.channels_in_layer("diffuse")
        .map(|(name, _)| name )
        .collect::<Vec<&str>>(),
    ["diffuse.B", "diffuse.G", "diffuse.R"]
)
```

# Tiles, Levels and Level Modes

A single tiled OpenEXR file can hold multiple versions of an image, each
with a different resolution. Each version is called a *level*. A tiled
file\'s *level mode* defines how many levels are stored in the file.
There are three different level modes:

| Mode | Explanation |
| --- | --- |
| [`LevelMode::OneLevel`](crate::core::LevelMode::OneLevel) | The file contains a single level. |
| [`LevelMode::MipmapLevels`](crate::core::LevelMode::MipmapLevels) |   The file contains multiple levels. The first level holds the image at full resolution. Each successive level is half the resolution of the previous level in x and y direction. The last level contains only a single pixel. `MipmapLevels` files are used for texture-mapping and similar applications. |
| [`LevelMode::RipmapLevels`](crate::core::LevelMode::RipmapLevels) |  Like `MipmapLevels`, but with more levels. The levels include all combinations of reducing the resolution of the image by powers of two independently in x and y direction. Used for texture mapping, like `MipmapLevels`. The additional levels in a `RipmapLevels` file can help to accelerate anisotropic filtering during texture lookups. |

In `MipmapLevels` and `RipmapLevels` mode, the size (width or height)
of each level is computed by halving the size of the level with the next
higher resolution. If the size of the higher-resolution level is odd,
then the size of the lower-resolution level must be rounded up or down
in order to avoid arriving at a non-integer width or height. The
rounding direction is determined by the file\'s *level size rounding
mode*.

Within each level, the pixels of the image are stored in a
two-dimensional array of tiles. The tiles in an OpenEXR file can be any
rectangular shape, but all tiles in a file have the same size. This
means that lower-resolution levels contain fewer, rather than smaller,
tiles.

An OpenEXR file\'s level mode and rounding mode, and the size of the
tiles are stored in an attribute in the file header. The value of this
attribute is a [`TileDescription`](crate::core::TileDescription) object:

```
pub struct TileDescription {
    pub x_size: u32,
    pub y_size: u32,
    pub mode: LevelMode,
    pub rounding_mode: LevelRoundingMode,
}

pub enum LevelMode {
    OneLevel = 0,
    MipmapLevels = 1,
    RipmapLevels = 2,
    NumLevelmodes = 3,
}

pub enum LevelRoundingMode {
    RoundDown = 0,
    RoundUp = 1,
    NumRoundingmodes = 2,
}
```

# Using the RGBA-only Interface for Tiled Files

## Writing a Tiled RGBA Image File with One Resolution Level

Writing a tiled RGBA image with a single level is easy:

```no_run
use openexr::prelude::*;

fn write_tiled_rgba1(
    filename: &str,
    pixels: &[Rgba],
    width: i32,
    height: i32,
    tile_width: i32,
    tile_height: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let header = Header::from_dimensions(width, height);

    let mut file = TiledRgbaOutputFile::new(
        filename,
        &header,
        RgbaChannels::WriteRgba,
        tile_width,
        tile_height,
        LevelMode::OneLevel,
        LevelRoundingMode::RoundDown,
        1,
    )?;

    file.set_frame_buffer(&pixels, 1, width as usize)?;
    file.write_tiles(
        0,
        file.num_x_tiles(0)? - 1,
        0,
        file.num_y_tiles(0)? - 1,
        0,
        0,
    )?;
    
    Ok(())
}
```
[`TiledRgbaOutputFile`]: crate::tiled::tiled_rgba_file::TiledRgbaOutputFile;

Opening the file and defining the pixel data layout in memory are done
in almost the same way as for scan line based files:

Construction of the [`TiledRgbaOutputFile`] object, with `TiledRgbaOutputFile::new()` creates an OpenEXR header, sets the header's attributes, opens the file with the
specified name, and stores the header in the file. The header's display
window and data window are both set to `(0, 0) - (width-1, height-1)`.
The size of each tile in the file will be `tile_width` by `tile_height`
pixels. The channel list contains four channels, R, G, B, and A, of type
[`PixelType::Half`].

[`set_frame_buffer()`](crate::tiled::tiled_rgba_file::TiledRgbaOutputFile::set_frame_buffer) sets the `pixels` slice as the source for the image data, specifying the `x_stride` as 1 and `y_stride` as the width of the image, since the slice is denssely packed.

[`write_tiles()`](crate::tiled::tiled_rgba_file::TiledRgbaOutputFile::write_tiles) copies the pixels into the file. The first four arguments specify the x and y ranges of tiles to write, and we can use [`TiledRgbaOutputFile`]'s [`num_x_tiles()`](crate::tiled::tiled_rgba_file::TiledRgbaOutputFile::num_x_tiles) and [`num_y_tiles()`](crate::tiled::tiled_rgba_file::TiledRgbaOutputFile::num_y_tiles) to specify the full set of tiles in the image.

<!---
This simple method works well when enough memory is available to
allocate a frame buffer for the entire image. When allocating a frame
buffer for the whole image is not desirable, for example because the
image is very large, a smaller frame buffer can be used. Even a frame
buffer that can hold only a single tile is sufficient, as demonstrated
in the following example:

void

writeTiledRgbaONE2 (const char fileName\[\],

int width, int height,

int tileWidth, int tileHeight)

{

TiledRgbaOutputFile out (fileName,

width, height, // image size

tileWidth, tileHeight, // tile size

ONE_LEVEL, // level mode

ROUND_DOWN, // rounding mode

WRITE_RGBA); // channels in file // 1

Array2D\<Rgba> pixels (tileHeight, tileWidth); // 2

for (int tileY = 0; tileY \< out.numYTiles (); ++tileY) // 3

{

for (int tileX = 0; tileX \< out.numXTiles (); ++tileX) // 4

{

Box2i range = out.dataWindowForTile (tileX, tileY); // 5

generatePixels (pixels, width, height, range); // 6

out.setFrameBuffer (&pixels\[-range.min.y\]\[-range.min.x\],

1, // xStride

tileWidth); // yStride // 7

out.writeTile (tileX, tileY); // 8

}

}

}

In line 2 we allocate a *pixels* array with *tileWidth\*tileHeight*
elements, which is just enough for one tile. Line 5 computes the data
window range for each tile, that is, the set of pixel coordinates
covered by the tile. The *generatePixels()* function, in line 6, fills
the *pixels* array with one tile\'s worth of image data. The same
*pixels* array is reused for all tiles. We must call *setFrameBuffer()*,
in line 7, before writing each tile so that the pixels in the array are
accessed properly in the *writeTile()* call in line 8. Again, the
address arithmetic to access the pixels is the same as for scan line
based files. The values for the *base*, *xStride*, and *yStride*
arguments to the *setFrameBuffer()* call must be chosen so that
evaluating the expression

base + x \* xStride + y \* yStride

produces the address of the pixel with coordinates *(x,y)*.

## Writing a Tiled RGBA Image File with Mipmap Levels

In order to store a multi-resolution image in a file, we can allocate a
frame buffer large enough for the highest-resolution level, *(0,0)*, and
reuse it for all levels:

void

writeTiledRgbaMIP1 (const char fileName\[\],

int width, int height,

int tileWidth, int tileHeight)

{

TiledRgbaOutputFile out (fileName,

width, height,

tileWidth, tileHeight,

MIPMAP_LEVELS,

ROUND_DOWN,

WRITE_RGBA); // 1

Array2D\<Rgba> pixels (height, width); // 2

out.setFrameBuffer (&pixels\[0\]\[0\], 1, width); // 3

for (int level = 0; level \< out.numLevels (); ++level) // 4

{

generatePixels (pixels, width, height, level); // 5

out.writeTiles (0, out.numXTiles (level) -- 1, // 6

0, out.numYTiles (level) -- 1,

level);

}

}

The main difference here is the use of *MIPMAP_LEVELS* in line 1 for the
*TiledRgbaOutputFile* constructor. This signifies that the file will
contain multiple levels, each level being a factor of 2 smaller in both
dimensions than the previous level. Mipmap images contain *n* levels,
with level numbers

(0,0), (1,1), \... (n-1,n-1),

where

n = floor (log (max (width, height)) / log (2)) + 1

if the level size rounding mode is *ROUND_DOWN*, or

n = ceil (log (max (width, height)) / log (2)) + 1

if the level size rounding mode is *ROUND_UP*. Note that even though
level numbers are pairs of integers, *(lx,ly)*, only levels where *lx*
equals *ly* are used in *MIPMAP_LEVELS* files.

Line 2 allocates a *pixels* array with *width* by *height* pixels, big
enough to hold the highest-resolution level.

In order to store all tiles in the file, we must loop over all levels in
the image (line 4). *numLevels()* returns the number of levels, *n*, in
our mipmapped image. Since the tile sizes remain the same in all levels,
the number of tiles in both dimensions varies between levels.
*numXTiles()* and *numYTiles()* take a level number as an optional
argument, and return the number of tiles in the x or y direction for the
corresponding level. Line 5 fills the *pixels* array with appropriate
data for each level, and line 6 stores the pixel data in the file.

As with *ONE_LEVEL* images, we can choose to only allocate a frame
buffer for a single tile and reuse it for all tiles in the image:

void

writeTiledRgbaMIP2 (const char fileName\[\],

int width, int height,

int tileWidth, int tileHeight)

{

TiledRgbaOutputFile out (fileName,

width, height,

tileWidth, tileHeight,

MIPMAP_LEVELS,

ROUND_DOWN,

WRITE_RGBA);

Array2D\<Rgba> pixels (tileHeight, tileWidth);

for (int level = 0; level \< out.numLevels (); ++level)

{

for (int tileY = 0; tileY \< out.numYTiles (level); ++tileY)

{

for (int tileX = 0; tileX \< out.numXTiles (level); ++tileX)

{

Box2i range = out.dataWindowForTile (tileX, tileY, level);

generatePixels (pixels, width, height, range, level);

out.setFrameBuffer (&pixels\[-range.min.y\]\[-range.min.x\],

1, // xStride

tileWidth); // yStride

out.writeTile (tileX, tileY, level);

}

}

}

}

The structure of this code is the same as for writing a *ONE_LEVEL*
image using a tile-sized frame buffer, but we have to loop over more
tiles. Also, *dataWindowForTile()* takes an additional level argument to
determine the pixel range for the tile at the specified level.

## Writing a Tiled RGBA Image File with Ripmap Levels

The ripmap level mode allows for storing all combinations of reducing
the resolution of the image by powers of two independently in both
dimensions. Ripmap files contains *nx\*ny* levels, with level numbers:

(0, 0), (1, 0), \... (nx-1, 0),

(0, 1), (1, 1), \... (nx-1, 1),

\...

(0,ny-1), (1,ny-1), \... (nx-1,ny-1)

where

nx = floor (log (width) / log (2)) + 1

ny = floor (log (height) / log (2)) + 1

if the level size rounding mode is *ROUND_DOWN*, or

nx = ceil (log (width) / log (2)) + 1

ny = ceil (log (height) / log (2)) + 1

if the level size rounding mode is *ROUND_UP*.

With a frame buffer that is large enough to hold level *(0,0)*, we can
write a ripmap file like this:

void

writeTiledRgbaRIP1 (const char fileName\[\],

int width, int height,

int tileWidth, int tileHeight)

{

TiledRgbaOutputFile out (fileName,

width, height,

tileWidth, tileHeight,

RIPMAP_LEVELS,

ROUND_DOWN,

WRITE_RGBA);

Array2D\<Rgba> pixels (height, width);

out.setFrameBuffer (&pixels\[0\]\[0\], 1, width);

for (int yLevel = 0; yLevel \< out.numYLevels (); ++yLevel)

{

for (int xLevel = 0; xLevel \< out.numXLevels (); ++xLevel)

{

generatePixels (pixels, width, height, xLevel, yLevel);

out.writeTiles (0, out.numXTiles (xLevel) - 1,

0, out.numYTiles (yLevel) -- 1,

xLevel,

yLevel);

}

}

}

As for *ONE_LEVEL* and *MIPMAP_LEVELS* files, the frame buffer doesn\'t
have to be large enough to hold a whole level. Any frame buffer big
enough to hold at least a single tile will work.

-->

## Reading a Tiled RGBA Image File

Reading a tiled RGBA image file is done similarly to writing one:

```no_run
use imath_traits::Zero;
use openexr::prelude::*;

fn read_tiled_rgba1(
    filename: &str,
) -> Result<Vec<Rgba>, Box<dyn std::error::Error>> {
    let mut file = TiledRgbaInputFile::new(filename, 1)?;
    let data_window = file.header().data_window::<[i32; 4]>().clone();
    let width = data_window[2] - data_window[0] + 1;
    let height = data_window[3] - data_window[1] + 1;

    let mut pixels = vec![Rgba::zero(); (width * height) as usize];
    file.set_frame_buffer(&mut pixels, 1, width as usize)?;

    file.read_tiles(
        0,
        file.num_x_tiles(0)? - 1,
        0,
        file.num_y_tiles(0)? - 1,
        0,
        0,
    )?;

    Ok(pixels)
}
```

[`TiledRgbaInputFile`]: crate::tiled::tiled_rgba_file::TiledRgbaInputFile

First we need to create a [`TiledRgbaInputFile`] object for the given file
name. We then retrieve information about the data window in order to
create an appropriately sized frame buffer, in this case large enough to
hold the whole image at level `(0,0)`. After we set the frame buffer, we
read the tiles from the file.

This example only reads the highest-resolution level of the image. It
can be extended to read all levels, for multi-resolution images, by also
iterating over all levels within the image, analogous to the examples in
[Writing a Tiled RGBA Image File with Mipmap
Levels](#writing-a-tiled-rgba-image-file-with-mipmap-levels)
and [Writing a Tiled RGBA Image File with Ripmap
Levels](#writing-a-tiled-rgba-image-file-with-ripmap-levels).

# Using the General Interface for Tiled Files

## Writing a Tiled Image File

This example is a variation of the one in *[Writing an Image
File](#Writing%20an%20Image%20File)*, on page
[10](#Writing%20an%20Image%20File). We are writing a *ONE_LEVEL* image
file with two channels, G, and Z, of type *HALF*, and *FLOAT*
respectively, but here the file is tiled instead of scan line based:

void

writeTiled1 (const char fileName\[\],

Array2D\<GZ> &pixels,

int width, int height,

int tileWidth, int tileHeight)

{

Header header (width, height); // 1

header.channels().insert (\"G\", Channel (HALF)); // 2

header.channels().insert (\"Z\", Channel (FLOAT)); // 3

header.setTileDescription

(TileDescription (tileWidth, tileHeight, ONE_LEVEL)); // 4

TiledOutputFile out (fileName, header); // 5

FrameBuffer frameBuffer; // 6

frameBuffer.insert (\"G\", // name // 7

Slice (HALF, // type // 8

(char \*) &pixels\[0\]\[0\].g, // base // 9

sizeof (pixels\[0\]\[0\]) \* 1, // xStride // 10

sizeof (pixels\[0\]\[0\]) \* width)); // yStride // 11

frameBuffer.insert (\"Z\", // name // 12

Slice (FLOAT, // type // 13

(char \*) &pixels\[0\]\[0\].z, // base // 14

sizeof (pixels\[0\]\[0\]) \* 1, // xStride // 15

sizeof (pixels\[0\]\[0\]) \* width)); // yStride // 16

out.setFrameBuffer (frameBuffer); // 17

out.writeTiles (0, out.numXTiles() - 1, 0, out.numYTiles() - 1); // 18

}

As one would expect, the code here is very similar to the code in
*[Writing an Image File](#Writing%20an%20Image%20File)* on page
[10](#Writing%20an%20Image%20File). The file\'s header is created in
line 1, while lines 2 and 3 specify the names and types of the image
channels that will be stored in the file. An important addition is line
4, where we define the size of the tiles and the level mode. In this
example we use *ONE_LEVEL* for simplicity. Line 5 opens the file and
writes the header. Lines 6 through 17 tell the *TiledOutputFile* object
the location and layout of the pixel data for each channel. Finally,
line 18 stores the tiles in the file.

## Reading a Tiled Image File

Reading a tiled file with the general interface is virtually identical
to reading a scan line based file, as shown in *[Interleaving Image
Channels in the Frame
Buffer](#Interleaving%20Image%20Channels%20in%20the%20Frame%20Buffer)*,
on page
[14](#Interleaving%20Image%20Channels%20in%20the%20Frame%20Buffer); only
the last three lines are different. Instead of reading all scan lines at
once with a single function call, here we must iterate over all tiles we
want to read.

void

readTiled1 (const char fileName\[\],

Array2D\<GZ> &pixels,

int &width, int &height)

{

TiledInputFile in (fileName);

Box2i dw = in.header().dataWindow();

width = dw.max.x - dw.min.x + 1;

height = dw.max.y - dw.min.y + 1;

int dx = dw.min.x;

int dy = dw.min.y;

pixels.resizeErase (height, width);

FrameBuffer frameBuffer;

frameBuffer.insert (\"G\",

Slice (HALF,

(char \*) &pixels\[-dy\]\[-dx\].g,

sizeof (pixels\[0\]\[0\]) \* 1,

sizeof (pixels\[0\]\[0\]) \* width));

frameBuffer.insert (\"Z\",

Slice (FLOAT,

(char \*) &pixels\[-dy\]\[-dx\].z,

sizeof (pixels\[0\]\[0\]) \* 1,

sizeof (pixels\[0\]\[0\]) \* width));

in.setFrameBuffer (frameBuffer);

in.readTiles (0, in.numXTiles() - 1, 0, in.numYTiles() - 1);

}

In this example we assume that the file we want to read contains two
channels, G and Z, of type *HALF* and *FLOAT* respectively. If the file
contains other channels, we ignore them. We only read the
highest-resolution level of the image. If the input file contains more
levels (*MIPMAP_LEVELS* or *MIPMAP_LEVELS*), we can access the extra
levels by calling a four-argument version of the *readTile()* function:

in.readTile (tileX, tileY, levelX, levelY);

or by calling a six-argument version of *readTiles()*:

in.readTiles (tileXMin, tileXMax, tileYMin, tileYMax, levelX, levelY);

# Deep Data Files (New in 2.0) {#deep-data-files-new-in-20}

## Writing a Deep Scan Line File

This example creates an deep scan line file with two channels. It
demonstrates how to write a deep scan line file with two channels:

1.  type *FLOAT*, is called Z, and is used for storing sample depth, and
2.  type *HALF*, is called A and is used for storing sample opacity.

The size of the image is *width* by *height* pixels.

void writeDeepScanlineFile(const char filename\[\],

Box2i displayWindow,

Box2i dataWindow,

Array2D\< float\* \> &dataZ,

Array2D\< half\* \> &dataA,

Array2D\< unsigned int \> sampleCount)

{

int height = dataWindow.max.y - dataWindow.min.y + 1;

int width = dataWindow.max.x - dataWindow.min.x + 1;

Header header(displayWindow, dataWindow);

header.channels().insert(\"Z\", Channel(FLOAT));

header.channels().insert(\"A\", Channel(HALF));

header.setType(DEEPSCANLINE);

header.compression() = ZIP_COMPRESSION;

DeepScanLineOutputFile file(filename, header);

DeepFrameBuffer frameBuffer;

frameBuffer.insertSampleCountSlice (Slice (UINT,

(char \*) (&sampleCount\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (unsigned int) \* 1, // xStride

sizeof (unsigned int) \* width)); // yStride

frameBuffer.insert (\"Z\",

DeepSlice (FLOAT,

(char \*) (&dataZ\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (float \*) \* 1, // xStride for pointer array

sizeof (float \*) \* width, // yStride for pointer array

sizeof (float) \* 1)); // stride for Z data sample

frameBuffer.insert (\"A\",

DeepSlice (HALF,

(char \*) (&dataA\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (half \*) \* 1, // xStride for pointer array

sizeof (half \*) \* width, // yStride for pointer array

sizeof (half) \* 1)); // stride for A data sample

file.setFrameBuffer(frameBuffer);

file.readPixelSampleCounts (height);

for (int i = 0; i \< height; i++)

{

for (int j = 0; j \< width; j++)

{

sampleCount\[i\]\[j\] = getPixelSampleCount(i,j);

dataZ\[i\]\[j\] = new float\[sampleCount\[i\]\[j\]\];

dataA\[i\]\[j\] = new half\[sampleCount\[i\]\[j\]\];

// Generate data for dataZ and dataA.

}

file.writePixels(1);

}

for (int i = 0; i \< height; i++)

for (int j = 0; j \< width; j++)

{

delete\[\] dataZ\[i\]\[j\];

delete\[\] dataA\[i\]\[j\];

}

}

The interface for deep scan line files is similar to scan line files. We
added two new classes to deal with deep data: *DeepFrameBuffer* and
*DeepSlice*. *DeepFrameBuffer* only accepts *DeepSlice* as its input,
except that it accepts *Slice* for sample count slice. The first
difference we see from the previous version is:

header.setType(DEEPSCANLINE);

where we set the type of the header to a predefine string
*DEEPSCANLINE*, then we insert a sample count slice using
*insertSampleCountSlice()*. After that, we insert a *DeepSlice* with
deep z data. Notice that deep slices have three strides, one more than
non-deep slices. The first two strides are used for the pointers in the
array. Because the memory space for *Array2D* is contiguous, we can get
the strides easily. The third stride is used for pixel samples. Because
the data type is float (and we are not interleaving), the stride should
be sizeof(float). If we name the stride for deep data samples
*sampleStride*, then the memory address of the i-th sample of this
channel in pixel (x, y) is

base +

x \* xStride +

y \* yStride +

i \* sampleStride

Because we may not know the data until we are going to write it, the
deep data file must support postponed initialization, as shown in the
example code. Another approach would be to prepare all the data first,
and then write it all out at once.

Once the slices have been inserted, we get the sample count for each
pixel, via a user-supplied *getPixelSampleCount()* function, and
dynamically allocate memory for the Z and A channels. We then write to
file in a line-by-line fashion and finally free the the intermediate
data structures.

## Reading a Deep Scan Line File

An example of reading a deep scan line file created by previous code.

void readDeepScanlineFile(const char filename\[\],

Box2i& displayWindow,

Box2i& dataWindow,

Array2D\< float\* \>& dataZ,

Array2D\< half\* \>& dataA,

Array2D\< unsigned int \>& sampleCount)

{

DeepScanLineInputFile file(filename);

const Header& header = file.header();

dataWindow = header.dataWindow();

displayWindow = header.displayWindow();

int width = dataWindow.max.x - dataWindow.min.x + 1;

int height = dataWindow.max.y - dataWindow.min.y + 1;

sampleCount.resizeErase(height, width);

dataZ.resizeErase(height, width);

dataA.resizeErase(height, width);

DeepFrameBuffer frameBuffer;

frameBuffer.insertSampleCountSlice (Slice (UINT,

(char \*) (&sampleCount\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (unsigned int) \* 1, // xStride

sizeof (unsigned int) \* width)); // yStride

frameBuffer.insert (\"dataZ\",

DeepSlice (FLOAT,

(char \*) (&dataZ\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (float \*) \* 1, // xStride for pointer array

sizeof (float \*) \* width, // yStride for pointer array

sizeof (float) \* 1)); // stride for Z data sample

frameBuffer.insert (\"dataA\",

DeepSlice (HALF,

(char \*) (&dataA\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (half \*) \* 1, // xStride for pointer array

sizeof (half \*) \* width, // yStride for pointer array

sizeof (half) \* 1)); // stride for O data sample

file.setFrameBuffer(frameBuffer);

file.readPixelSampleCounts(dataWindow.min.y, dataWindow.max.y);

for (int i = 0; i \< height; i++)

for (int j = 0; j \< width; j++)

{

dataZ\[i\]\[j\] = new float\[sampleCount\[i\]\[j\]\];

dataA\[i\]\[j\] = new half\[sampleCount\[i\]\[j\]\];

}

file.readPixels(dataWindow.min.y, dataWindow.max.y);

for (int i = 0; i \< height; i++)

for (int j = 0; j \< width; j++)

{

delete\[\] dataZ\[i\]\[j\];

delete\[\] dataA\[i\]\[j\];

}

}

The interface for deep scan line files is similar to scan line files.
The main the difference is we use the sample count slice and deep data
slices. To do this, we added a new method to read the sample count table
from the file:

file.readPixelSampleCounts(dataWindow.min.y, dataWindow.max.y);

This method reads all pixel sample counts in the range
*\[dataWindow.min.y, dataWindow.max.y\]*, and stores the data to sample
count slice in framebuffer.

*ReadPixels()* supports for postponed memory allocation.

## Writing a Deep Tiled File

This example creates an deep tiled file with two channels. It
demonstrates how to write a deep tiled file with two channels:

1.  type *FLOAT*, is called Z, and is used for storing sample depth, and
2.  type *HALF*, is called A and is used for storing sample opacity.

The size of the image is *width* by *height* pixels.

void writeDeepTiledFile(const char filename\[\],

Box2i displayWindow,

Box2i dataWindow,

int tileSizeX, int tileSizeY)

{

int height = dataWindow.max.y - dataWindow.min.y + 1;

int width = dataWindow.max.x - dataWindow.min.x + 1;

Header header(displayWindow, dataWindow);

header.channels().insert(\"Z\", Channel(FLOAT));

header.channels().insert(\"A\", Channel(HALF));

header.setType(DEEPTILE);

header.setTileDescription(

TileDescription(tileSizeX, tileSizeY, MIPMAP_LEVELS));

Array2D\< unsigned int\* \> dataZ;

dataZ.resizeErase(height, width);

Array2D\< unsigned int\* \> dataA;

dataA.resizeErase(height, width);

Array2D\<unsigned int> sampleCount;

sampleCount.resizeErase(height, width);

DeepTiledOutputFile file(filename, header);

DeepFrameBuffer frameBuffer;

frameBuffer.insertSampleCountSlice (Slice (UINT,

(char \*) (&sampleCount\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (unsigned int) \* 1, // xStride

sizeof (unsigned int) \* width)); // yStride

frameBuffer.insert (\"Z\",

DeepSlice (FLOAT,

(char \*) (&dataZ\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (float \*) \* 1, // xStride for pointer array

sizeof (float \*) \* width, // yStride for pointer array

sizeof (float) \* 1)); // stride for samples

frameBuffer.insert (\"A\",

DeepSlice (HALF,

(char \*) (&dataA\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (half \*) \* 1, // xStride for pointer array

sizeof (half \*) \* width, // yStride for pointer array

sizeof (half) \* 1)); // stride for samples

file.setFrameBuffer(frameBuffer);

for (int l = 0; l \< file.numLevels(); l++)

{

for (int j = 0; j \< file.numYTiles(l); j++)

{

for (int i = 0; i \< file.numXTiles(l); i++)

{

getSampleCountForTile(i, j, sampleCount);

getSampleDataForTile(i, j, dataZ, dataA);

file.writeTile(i, j, l);

}

}

}

}

Here, getSampleCountForTile is a user-supplied function that sets each
item in sampleCount array to the correct sampleCount for each pixel in
the tile, and getSampleDataForTile is a user-supplied function that set
the pointers in dataZ and dataA arrays to point to the correct data

The interface for deep tiled files is similar to tiled files. The
differences are:

-   we set the type of the header to *DEEPTILE*
-   we use *insertSampleCountSlice()* to set sample count slice, and
-   we use *DeepSlice* instead of *Slice* to provide three strides
    needed by the library.

Also, we support postponed initialization.

## Reading a Deep Tiled File

An example of reading a deep tiled file created by code explained in the
*[Writing a Deep Tiled File](#Writing%20a%20Deep%20Tiled%20File)*
section, on page [26](#Writing%20a%20Deep%20Tiled%20File).

void readDeepTiledFile(const char filename\[\],

Box2i& displayWindow,

Box2i& dataWindow,

Array2D\< float\* \>& dataZ,

Array2D\< half\* \>& dataA,

Array2D\< unsigned int \>& sampleCount)

{

DeepTiledInputFile file(filename);

int width = dataWindow.max.x - dataWindow.min.x + 1;

int height = dataWindow.max.y - dataWindow.min.y + 1;

sampleCount.resizeErase(height, width);

dataZ.resizeErase(height, width);

dataA.resizeErase(height, width);

DeepFrameBuffer frameBuffer;

frameBuffer.insertSampleCountSlice (Slice (UINT,

(char \*) (&sampleCount\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (unsigned int) \* 1, // xStride

sizeof (unsigned int) \* width)); // yStride

frameBuffer.insert (\"Z\",

DeepSlice (FLOAT,

(char \*) (&dataZ\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (float \*) \* 1, // xStride for pointer array

sizeof (float \*) \* width, // yStride for pointer array

sizeof (float) \* 1)); // stride for samples

frameBuffer.insert (\"A\",

DeepSlice (HALF,

(char \*) (&dataA\[0\]\[0\]

\- dataWindow.min.x

\- dataWindow.min.y \* width),

sizeof (half \*) \* 1, // xStride for pointer array

sizeof (half \*) \* width, // yStride for pointer array

sizeof (half) \* 1)); // stride for samples

file.setFrameBuffer(frameBuffer);

int numXTiles = file.numXTiles(0);

int numYTiles = file.numYTiles(0);

file.readPixelSampleCounts(0, numXTiles - 1, 0, numYTiles - 1);

for (int i = 0; i \< height; i++)

for (int j = 0; j \< width; j++)

{

dataZ\[i\]\[j\] = new float\[sampleCount\[i\]\[j\]\];

dataA\[i\]\[j\] = new half\[sampleCount\[i\]\[j\]\];

}

file.readTiles(0, numXTiles - 1, 0, numYTiles -- 1);

// (after read data is processed, data must be freed:)

for (int i = 0; i \< height; i++)

for (int j = 0; j \< width; j++)

{

delete\[\] dataZ\[i\]\[j\];

delete\[\] dataA\[i\]\[j\];

}

}

This code demonstrates how to read the first level of a deep tiled file
created by code explained in the *[Writing a Deep Tiled
File](#Writing%20a%20Deep%20Tiled%20File)* section, on page
[26](#Writing%20a%20Deep%20Tiled%20File). The interface for deep tiled
files is similar to tiled files. The differences are:

-   we use *insertSampleCountSlice()* to set sample count slice
-   we use *DeepSlice* instead of *Slice* to provide three strides
    needed by the library, and
-   we use *readPixelSampleCounts()* to read in pixel sample counts into
    array.

Also we support postponed memory allocation.

In this example, entries in dataZ and dataA have been allocated by the
\'new\' calls must be deleted after use.

<!---
# Threads

## Library Thread-Safety

The IlmImf library is thread-safe. In a multithreaded application
program, multiple threads can concurrently read and write distinct
OpenEXR files. In addition, accesses to a single shared file by multiple
application threads are automatically serialized. In other words, each
thread can independently create, use and destroy its own input and
output file objects. Multiple threads can also share a single input or
output file. In the latter case the IlmImf library uses mutual exclusion
to ensure that only one thread at a time can access the shared file.

## Multithreaded I/O

The IlmImf library supports multithreaded file input and output where
the library creates its own worker threads that are independent of the
application program\'s threads. When an application thread calls
*readPixels()*, *readTiles()*, *writePixels()* or *writeTiles()* to read
or write multiple scan lines or tiles at once, the library\'s worker
threads process the tiles or scanlines in parallel.

During startup, the application program must enable multithreading by
calling function *setGlobalThreadCount()*. This tells the IlmImf library
how many worker threads it should create. (As a special case, setting
the number of worker threads to zero reverts to single-threaded
operation; reading and writing image files happens entirely in the
application thread that calls the IlmImf library.)

The application program should read or write as many scan lines or tiles
as possible in each call to *readPixels()*, *readTiles()*,
*writePixels()* or *writeTiles()*. This allows the library to break up
the work into chunks that can be processed in parallel. Ideally the
application reads or writes the entire image using a single read or
write call. If the application reads or writes the image one scan line
or tile at a time, the library reverts to single-threaded file I/O.

The following function writes an RGBA file using four concurrent worker
threads:

void

writeRgbaMT (const char fileName\[\],

const Rgba \*pixels,

int width,

int height)

{

setGlobalThreadCount (4);

RgbaOutputFile file (fileName, width, height, WRITE_RGBA);

file.setFrameBuffer (pixels, 1, width);

file.writePixels (height);

}

Except for the call to *setGlobalThreadCount()*, function
*writeRgbaMT()* is identical to function *writeRgba1()* in *[Writing an
RGBA Image File](#Writing%20an%20RGBA%20Image%20File)*, on page
[4](#Writing%20an%20RGBA%20Image%20File), but on a computer with
multiple processors *writeRgbaMT()* writes files significantly faster
than *writeRgba1()*.

## Multithreaded I/O, Multithreaded Application Program

Function *setGlobalThreadCount()* creates a global pool of worker
threads inside the IlmImf library. If an application program has
multiple threads, and those threads read or write several OpenEXR files
at the same time, then the worker threads must be shared among the
application threads. By default each file will attempt to use the entire
worker thread pool for itself. If two files are read or written
simultaneously by two application threads, then it is possible that all
worker threads perform I/O on behalf of one of the files, while I/O for
the other file is stalled.

In order to avoid this situation, the constructors for input and output
file objects take an optional *numThreads* argument. This gives the
application program more control over how many threads will be kept busy
reading or writing a particular file.

For example, we may have an application program that runs on a
four-processor computer. The program has one thread that reads files and
another one that writes files. We want to keep all four processors busy,
and we want to split the processors evenly between input and output.
Before creating the input and output threads, the application instructs
the IlmImf library to create four worker threads:

// main, before application threads are created:

setGlobalThreadCount (4);

In the input and output threads, input and output files are opened with
*numThreads* set to 2:

// application\'s input thread

InputFile in (fileName, 2);

\...

// application\'s output thread

OutputFile out (fileName, header, 2);

\...

This ensures that file input and output in the application\'s two
threads can proceed concurrently, without one thread stalling the
other\'s I/O.

An alternative approach for thread management of multithreaded
applications is provided for deep scanline input files. Rather than
calling *setFrameBuffer()*, the host application may call
*rawPixelData()* to load a chunk of scanlines into a host-application
managed memory store, then pass a DeepFrameBuffer object and the raw
data to *readPixelSampleCounts()* and *readPixels()*. Only the call to
rawPixelData blocks; decompressing the underlying data and copying it to
the framebuffer will happen on the host application\'s threads
independently. This strategy is generally **less efficient** than
reading multiple scanlines at the same time and allowing OpenEXR\'s
thread management to decode the file, but may prove effective when the
host application has many threads available, cannot avoid accessing
scanlines in a random order and wishes to avoid caching an entire
uncompressed image. For more details, refer to the inline comments in
ImfDeepScanLineInputFile.h

# Low-Level I/O

## Custom Low-Level File I/O

In all of the previous file reading and writing examples, we were given
a file name, and we relied on the constructors for our input file or
output file objects to open the file. In some contexts, for example, in
a plugin for an existing application program, we may have to read from
or write to a file that has already been opened. The representation of
the open file as a C or C++ data type depends on the application program
and on the operating system.

At its lowest level, the IlmImf library performs file I/O via objects of
type *IStream* and *OStream*. *IStream* and *OStream* are abstract base
classes. The IlmImf library contains two derived classes, *StdIFStream*
and *StdOFStream*, that implement reading from *std::ifstream* and
writing to *std::ofstream* objects. An application program can implement
alternative file I/O mechanisms by deriving its own classes from
*Istream* and *Ostream*. This way, OpenEXR images can be stored in
arbitrary file-like objects, as long as it is possible to support read,
write, seek and tell operations with semantics similar to the
corresponding *std::ifstream* and *std::ofstream* methods.

For example, assume that we want to read an OpenEXR image from a C stdio
file (of type *FILE \**) that has already been opened. To do this, we
derive a new class, *C_IStream*, from *IStream*. The declaration of
class *IStream* looks like this:

class IStream

{

public:

virtual \~IStream ();

virtual bool read (char c\[\], int n) = 0;

virtual Int64 tellg () = 0;

virtual void seekg (Int64 pos) = 0;

virtual void clear ();

const char \* fileName () const;

virtual bool isMemoryMapped () const;

virtual char \* readMemoryMapped (int n);

protected:

IStream (const char fileName\[\]);

private:

\...

};

Our derived class needs a public constructor, and it must override four
methods:

class C_IStream: public IStream

{

public:

C_IStream (FILE \*file, const char fileName\[\]):

IStream (fileName), \_file (file) {}

virtual bool read (char c\[\], int n);

virtual Int64 tellg ();

virtual void seekg (Int64 pos);

virtual void clear ();

private:

FILE \* \_file;

};

*read(c,n)* reads *n* bytes from the file, and stores them in array *c*.
If reading hits the end of the file before *n* bytes have been read, or
if an I/O error occurs, *read(c,n)* throws an exception. If *read(c,n)*
hits the end of the file after reading *n* bytes, it returns *false*,
otherwise it returns *true*:

bool

C_IStream::read (char c\[\], int n)

{

if (n != fread (c, 1, n, \_file))

{

// fread() failed, but the return value does not distinguish

// between I/O errors and end of file, so we call ferror() to

// determine what happened.

if (ferror (\_file))

Iex::throwErrnoExc();

else

throw Iex::InputExc (\"Unexpected end of file.\");

}

return feof (\_file);

}

*tellg()* returns the current reading position, in bytes, from the
beginning of the file. The next *read()* call will begin reading at the
indicated position:

Int64

C_IStream::tellg ()

{

return ftell (\_file);

}

*seekg(pos)* sets the current reading position to *pos* bytes from the
beginning of the file:

void

C_IStream::seekg (Int64 pos)

{

clearerr (\_file);

fseek (\_file, pos, SEEK_SET);

}

*clear()* clears any error flags that may be set on the file after a
*read()* or *seekg()* operation has failed:

void

C_IStream::clear ()

{

clearerr (\_file);

}

In order to read an RGBA image from an open C stdio file, we first make
a *C_IStream* object. Then we create an *RgbaInputFile*, passing the
*C_IStream* instead of a file name to the constructor. After that, we
read the image as usual (see *[Reading an RGBA Image
File](#Reading%20an%20RGBA%20Image%20File)*, on page
[7](#Reading%20an%20RGBA%20Image%20File)):

void

readRgbaFILE (FILE \*cfile,

const char fileName\[\],

Array2D\<Rgba> &pixels,

int &width,

int &height)

{

C_IStream istr (cfile, fileName);

RgbaInputFile file (istr);

Box2i dw = file.dataWindow();

width = dw.max.x - dw.min.x + 1;

height = dw.max.y - dw.min.y + 1;

pixels.resizeErase (height, width);

file.setFrameBuffer (&pixels\[0\]\[0\] - dw.min.x - dw.min.y \* width,
1, width);

file.readPixels (dw.min.y, dw.max.y);

}

## Memory-Mapped I/O

When the IlmImf library reads an image file, pixel data are copied
several times on their way from the file to the application\'s frame
buffer. For compressed files, the time spent copying is usually not
significant when compared to how long it takes to uncompress the data.
However, when uncompressed image files are being read from a fast file
system, it may be advantageous to eliminate one or two copy operations
by using memory-mapped I/O.

Memory-mapping establishes a relationship between a file and a
program\'s virtual address space, such that from the program\'s point of
view the file looks like an array of type *char*. The contents of the
array match the data in the file. This allows the program to access the
data in the file directly, bypassing any copy operations associated with
reading the file via a C++ *std::ifstream* or a C *FILE \**.

Classes derived from *IStream* can optionally support memory-mapped
input. In order to do this, a derived class must override two virtual
functions, *isMemoryMapped()* and *readMemoryMapped()*, in addition to
the functions needed for regular, non-memory-mapped input:

class MemoryMappedIStream: public IStream

{

public:

MemoryMappedIStream (const char fileName\[\]);

virtual \~MemoryMappedIStream ();

virtual bool isMemoryMapped () const;

virtual char \* readMemoryMapped (int n);

virtual bool read (char c\[\], int n);

virtual Int64 tellg ();

virtual void seekg (Int64 pos);

private:

char \* \_buffer;

Int64 \_fileLength;

Int64 \_readPosition;

};

The constructor for class *MemoryMappedIStream* maps the contents of the
input file into the program\'s address space. Memory mapping is not
portable across operating systems. The example shown here uses the POSIX
*mmap()* system call. On Windows files can be memory-mapped by calling
*CreateFileMapping()* and *MapViewOfFile()*:

MemoryMappedIStream::MemoryMappedIStream (const char fileName\[\]):

IStream (fileName),

\_buffer (0),

\_fileLength (0),

\_readPosition (0)

{

int file = open (fileName, O_RDONLY);

if (file \< 0)

THROW_ERRNO (\"Cannot open file \\\"\" \<\< fileName \<\< \"\\\".\");

struct stat stat;

fstat (file, &stat);

\_fileLength = stat.st_size;

\_buffer = (char \*) mmap (0, \_fileLength, PROT_READ, MAP_PRIVATE,
file, 0);

close (file);

if (\_buffer == MAP_FAILED)

THROW_ERRNO (\"Cannot memory-map file \\\"\" \<\< fileName \<\<
\"\\\".\");

}

The destructor frees the address range associated with the file by
un-mapping the file. The POSIX version shown here uses *munmap()*. A
Windows version would call *UnmapViewOfFile()* and *CloseHandle()*:

MemoryMappedIStream::\~MemoryMappedIStream ()

{

munmap (\_buffer, \_fileLength);

}

Function *isMemoryMapped()* returns *true* to indicate that
memory-mapped input is supported. This allows the IlmImf library to call
*readMemoryMapped()* instead of *read()*:

bool

MemoryMappedIStream::isMemoryMapped () const

{

return true;

}

*readMemoryMapped()* is analogous to *read()*, but instead of copying
data into a buffer supplied by the caller, *readMemoryMapped()* returns
a pointer into the memory-mapped file, thus avoiding the copy operation:

char \*

MemoryMappedIStream::readMemoryMapped (int n)

{

if (\_readPosition \>= \_fileLength)

throw Iex::InputExc (\"Unexpected end of file.\");

if (\_readPosition + n \> \_fileLength)

throw Iex::InputExc (\"Reading past end of file.\");

char \*data = \_buffer + \_readPosition;

\_readPosition += n;

return data;

}

The *MemoryMappedIStream* class must also implement the regular *read()*
function, as well as *tellg()* and *seekg()*:

bool

MemoryMappedIStream::read (char c\[\], int n)

{

if (\_readPosition \>= \_fileLength)

throw Iex::InputExc (\"Unexpected end of file.\");

if (\_readPosition + n \> \_fileLength)

throw Iex::InputExc (\"Reading past end of file.\");

memcpy (c, \_buffer + \_readPosition, n);

\_readPosition += n;

return \_readPosition \< \_fileLength;

}

Int64

MemoryMappedIStream::tellg ()

{

return \_readPosition;

}

void

MemoryMappedIStream::seekg (Int64 pos)

{

\_readPosition = pos;

}

Class *MemoryMappedIStream* does not need a *clear()* function. Since
the memory-mapped file has no error flags that need to be cleared, the
*clear()* method provided by class *IStream*, which does nothing, can be
re-used.

Memory-mapping a file can be faster than reading the file via a C++
*std::istream* or a C *FILE \**, but the extra speed comes at a cost. A
large memory-mapped file can occupy a significant portion of a
program\'s virtual address space. In addition, mapping and un-mapping
many files of varying sizes can severely fragment the address space.
After a while, the program may be unable to map any new files because
there is no contiguous range of free addresses that would be large
enough hold a file, even though the total amount of free space would be
sufficient. An application program that uses memory-mapped I/O should
manage its virtual address space in order to avoid fragmentation. For
example, the program can reserve several address ranges, each one large
enough to hold the largest file that the program expects to read. The
program can then explicitly map each new file into one of the reserved
ranges, keeping track of which ranges are currently in use.

-->

# Miscellaneous

## Is this an OpenEXR File?

Sometimes we want to test quickly if a given file is an OpenEXR file.
This can be done by looking at the beginning of the file: The first four
bytes of every OpenEXR file contain the 32-bit integer \"magic number\"
20000630 in little-endian byte order. After reading a file\'s first four
bytes via any of the operating system\'s standard file I/O mechanisms,
we can compare them with the magic number by explicitly testing if the
bytes contain the values *0x76*, *0x2f*, *0x31*, and *0x01*.

Given a file name, the following function returns *true* if the
corresponding file exists, is readable, and contains an OpenEXR image:

bool

isThisAnOpenExrFile (const char fileName\[\])

{

std::ifstream f (fileName, std::ios_base::binary);

char b\[4\];

f.read (b, sizeof (b));

return !!f && b\[0\] == 0x76 && b\[1\] == 0x2f && b\[2\] == 0x31 &&
b\[3\] == 0x01;

}

Using this function does not require linking with the IlmImf library.

Programs that are linked with the IlmImf library can determine if a
given file is an OpenEXR file by calling one of the following functions,
which are part of the library:

bool isOpenExrFile (const char fileName\[\], bool &isTiled);

bool isOpenExrFile (const char fileName\[\]);

bool isTiledOpenExrFile (const char fileName\[\]);

bool isOpenExrFile (IStream &is, bool &isTiled);

bool isOpenExrFile (IStream &is);

bool isTiledOpenExrFile (IStream &is);

## Is this File Complete?

Sometimes we want to test if an OpenEXR file is complete. The file may
be missing pixels, either because writing the file is still in progress
or because writing was aborted before the last scan line or tile was
stored in the file. Of course, we could test if a given file is complete
by attempting to read the entire file, but the input file classes in the
IlmImf library have an *isComplete()* method that is faster and more
convenient.

The following function returns *true* or *false*, depending on whether a
given OpenEXR file is complete or not:

bool

isComplete (const char fileName\[\])

{

InputFile in (fileName);

return in.isComplete();

}

## Preview Images

Graphical user interfaces for selecting image files often represent
files as small *preview* or *thumbnail* images. In order to make loading
and displaying the preview images fast, OpenEXR files support storing
preview images in the file headers.

A preview image is an attribute whose value is of type *PreviewImage*. A
*PreviewImage* object is an array of pixels of type *PreviewRgba*. A
pixel has four components, *r*, *g*, *b* and *a*, of type *unsigned
char*, where *r*, *g* and *b* are the pixel\'s red, green and blue
components, encoded with a gamma of 2.2. *a* is the pixel\'s alpha
channel; *r*, *g* and *b* should be premultiplied by *a*. On a typical
display with 8-bits per component, the preview image can be shown by
simply loading the *r*, *g* and *b* components into the display\'s frame
buffer. (No gamma correction or tone mapping is required.)

The code fragment below shows how to test if an OpenEXR file has a
preview image, and how to access a preview image\'s pixels:

RgbaInputFile file (fileName);

if (file.header().hasPreviewImage())

{

const PreviewImage &preview = file.header().previewImage();

for (int y = 0; y \< preview.height(); ++y)

for (int x = 0; x \< preview.width(); ++x)

{

const PreviewRgba &pixel = preview.pixel (x, y);

\...

}

}

Writing an OpenEXR file with a preview image is shown in the following
example. Since the preview image is an attribute in the file\'s header,
it is entirely separate from the main image. Here the preview image is a
smaller version of the main image, but this is not required; in some
cases storing an easily recognizable icon may be more appropriate. This
example uses the RGBA-only interface to write a scan line based file,
but preview images are also supported for files that are written using
the general interface, and for tiled files.

void

writeRgbaWithPreview1 (const char fileName\[\],

const Array2D\<Rgba> &pixels,

int width,

int height)

{

Array2D \<PreviewRgba> previewPixels; // 1

int previewWidth; // 2

int previewHeight; // 3

makePreviewImage (pixels, width, height, // 4

previewPixels, previewWidth, previewHeight);

Header header (width, height); // 5

header.setPreviewImage // 6

(PreviewImage (previewWidth, previewHeight, &previewPixels\[0\]\[0\]));

RgbaOutputFile file (fileName, header, WRITE_RGBA); // 7

file.setFrameBuffer (&pixels\[0\]\[0\], 1, width); // 8

file.writePixels (height); // 9

}

Lines 1 through 4 generate the preview image. Line 5 creates a header
for the image file. Line 6 converts the preview image into a
*PreviewImage* attribute, and adds the attribute to the header. Lines 7
through 9 store the header (with the preview image) and the main image
in a file.

Function *makePreviewImage()*, called in line 4, generates the preview
image by scaling the main image down to one eighth of its original width
and height:

void

makePreviewImage (const Array2D\<Rgba> &pixels,

int width,

int height,

Array2D\<PreviewRgba> &previewPixels,

int &previewWidth,

int &previewHeight)

{

const int N = 8;

previewWidth = width / N;

previewHeight = height / N;

previewPixels.resizeErase (previewHeight, previewWidth);

for (int y = 0; y \< previewHeight; ++y)

{

for (int x = 0; x \< previewWidth; ++x)

{

const Rgba &inPixel = pixels\[y \* N\]\[x \* N\];

PreviewRgba &outPixel = previewPixels\[y\]\[x\];

outPixel.r = gamma (inPixel.r);

outPixel.g = gamma (inPixel.g);

outPixel.b = gamma (inPixel.b);

outPixel.a = int (clamp (inPixel.a \* 255.f, 0.f, 255.f) + 0.5f);

}

}

}

To make this example easier to read, scaling the image is done by just
sampling every eighth pixel of every eighth scan line. This can lead to
aliasing artifacts in the preview image; for a higher-quality preview
image, the main image should be lowpass-filtered before it is
subsampled.

Function *makePreviewImage()* calls *gamma()* to convert the
floating-point red, green, and blue components of the sampled main image
pixels to *unsigned char* values. *gamma()* is a simplified version of
what the exrdisplay program does in order to show an OpenEXR image\'s
floating-point pixels on the screen (for details, see exrdisplay\'s
source code):

unsigned char

gamma (float x)

{

x = pow (5.5555f \* max (0.f, x), 0.4545f) \* 84.66f;

return (unsigned char) clamp (x, 0.f, 255.f);

}

*makePreviewImage()* converts the pixels\' alpha component to unsigned
char by by linearly mapping the range \[0.0, 1.0\] to \[0, 255\].

Some programs write image files one scan line or tile at a time, while
the image is being generated. Since the image does not yet exist when
the file is opened for writing, it is not possible to store a preview
image in the file\'s header at this time (unless the preview image is an
icon that has nothing to do with the main image). However, it is
possible to store a blank preview image in the header when the file is
opened. The preview image can then be updated as the pixels become
available. This is demonstrated in the following example:

void

writeRgbaWithPreview2 (const char fileName\[\],

int width,

int height)

{

Array \<Rgba> pixels (width);

const int N = 8;

int previewWidth = width / N;

int previewHeight = height / N;

Array2D \<PreviewRgba> previewPixels (previewHeight, previewWidth);

Header header (width, height);

header.setPreviewImage (PreviewImage (previewWidth, previewHeight));

RgbaOutputFile file (fileName, header, WRITE_RGBA);

file.setFrameBuffer (pixels, 1, 0);

for (int y = 0; y \< height; ++y)

{

generatePixels (pixels, width, height, y);

file.writePixels (1);

if (y % N == 0)

{

for (int x = 0; x \< width; x += N)

{

const Rgba &inPixel = pixels\[x\];

PreviewRgba &outPixel = previewPixels\[y / N\]\[x / N\];

outPixel.r = gamma (inPixel.r);

outPixel.g = gamma (inPixel.g);

outPixel.b = gamma (inPixel.b);

outPixel.a = int (clamp (inPixel.a \* 255.f, 0.f, 255.f) + 0.5f);

}

}

}

file.updatePreviewImage (&previewPixels\[0\]\[0\]);

}

## Environment Maps

An environment map is an image that represents an omnidirectional view
of a three-dimensional scene as seen from a particular 3D location.
Every pixel in the image corresponds to a 3D direction, and the data
stored in the pixel represent the amount of light arriving from this
direction. In 3D rendering applications, environment maps are often used
for image-based lighting techniques that appoximate how objects are
illuminated by their surroundings. Environment maps with enough dynamic
range to represent even the brightest light sources in the environment
are sometimes called \"light probe images.\"

[`Envmap`]: crate::core::EnvMap

In an OpenEXR file, an environment map is stored as a rectangular pixel
array, just like any other image, but an attribute in the file header
indicates that the image is an environment map. The attribute\'s value,
which is of type [`Envmap`], specifies the relation between 2D pixel
locations and 3D directions. [`Envmap`] is an enumeration type. Two values
are possible:

| | |
|---|---|
| [`Envmap::Latlong`](crate::core::EnvMap::Latlong) | <p>The environment is projected onto the image using polar coordinates (latitude and longitude). A pixel's x coordinate corresponds to its longitude, and the y coordinate corresponds to its latitude. The pixel in the upper left corner of the data window has latitude +π/2 and longitude +π; the pixel in the lower right corner has latitude -π/2 and longitude -π.</p><p> In 3D space, latitudes -π/2 and +π/2 correspond to the negative and positive y direction. Latitude 0, longitude 0 points in the positive z direction; latitude 0, longitude π/2 points in the positive x direction.</p><p>  ![Latlong map][env_latlong] </p> <p> For a latitude-longitude map, the size of the data window should be 2×N by N pixels (width by height), where N can be any integer greater than 0.</p>|
| [`Envmap::Cube`](crate::core::EnvMap::Cube) | <p>The environment is projected onto the six faces of an axis-aligned cube. The cube's faces are then arranged in a 2D image as shown below.</p><p> ![Cube map][env_cubemap] </p><p> For a cube map, the size of the data window should be N by 6×N pixels (width by height), where N can be any integer greater than 0. </p> | 

**Note:** Both kinds of environment maps contain redundant pixels: In a
latitude-longitude map, the top row and the bottom row of pixels
correspond to the map\'s north pole and south pole (latitudes +π/2 and
-π/2). In each of those two rows all pixels are the same. The leftmost
column and the rightmost column of pixels both correspond to the
meridian with longitude +π (or, equivalently, -π). The pixels in the
leftmost column are repeated in the rightmost column. In a cube-face
map, the pixels along each edge of a face are repeated along the
corresponding edge of the adjacent face. The pixel in each corner of a
face is repeated in the corresponding corners of the two adjacent faces.

The following code fragment tests if an OpenEXR file contains an
environment map, and if it does, which kind:

```no_run
use openexr::prelude::*;

pub fn test_envmap(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = RgbaInputFile::new(filename, 1)?;

    if let Some(e) = file.header().find_typed_attribute_envmap("envmap") {
        match e.value() {
            Envmap::Latlong => println!("latlong"),
            Envmap::Cube => println!("cubemap"),
            _ => unreachable!(),
        }
    }

    Ok(())
}
```

For each kind of environment map, openexr provides a set of
routines that convert from 3D directions to 2D floating-point pixel
locations and back. Those routines are useful in application programs
that create environment maps and in programs that perform map lookups.
For details, see the documentation for the [`envmap`](crate::core::envmap) module.
