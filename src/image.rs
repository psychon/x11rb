//! Utility code for working with images.
//!
//! This module contains the [`Image`] struct which represents an arbitrary image in
//! `ImageFormat::ZPixmap`. If you do not know what `ZPixmap` means, then rest assured that you do
//! not want to know the details. It suffices to know that the values of the individual pixels are
//! saved one after another in memory.
//!
//! An [`Image`] can 

// For future readers:
//
// - Z-Pixmap means that the pixels are right next to each other. I.e. first you have the value of
//   the pixel at (0, 0), then (1, 0), .... At the end of the row, there might be some padding
//   before the next row begins.
// - XY-Bitmap consists of individual bits. The bits are assigned colors via a GC's foreground and
//   background color when uploading to the X11 server.
// - XY-Pixmap consists of bit planes. This means you first get the first bit of each pixel,
//   stuffed together into bytes. After all of the first pixels are represented, the second plane
//   begins with the second bit of each pixel etc.

use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};

use crate::connection::Connection;
use crate::cookie::VoidCookie;
use crate::errors::{ConnectionError, ParseError, ReplyError};
use crate::protocol::xproto::{Drawable, Format, Gcontext, ImageFormat, ImageOrder, Setup, GetImageRequest, GetImageReply, PutImageRequest};

// Compute the stride based on some information of the image
fn compute_stride(width: u16, bits_per_pixel: BitsPerPixel, scanline_pad: ScanlinePad) -> usize {
    let value = usize::from(width) * usize::from(bits_per_pixel);
    scanline_pad.round_to_multiple(value) / 8
}

// Find the format with the given depth in `setup.pixmap_formats`.
fn find_format(setup: &Setup, depth: u8) -> Result<&Format, ParseError>
{
    setup.pixmap_formats
        .iter()
        .find(|f| f.depth == depth)
        .ok_or(ParseError::InvalidValue)
}

macro_rules! number_enum {
    {
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant_name:ident = $value:literal,
            )*
        }
    } => {
        $(#[$meta])*
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        $vis enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant_name = $value,
            )*
        }

        impl TryFrom<u8> for $enum_name {
            type Error = ParseError;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Self::$variant_name),)*
                    _ => Err(ParseError::InvalidValue),
                }
            }
        }

        impl From<$enum_name> for u8 {
            fn from(value: $enum_name) -> u8 {
                match value {
                    $($enum_name::$variant_name => $value,)*
                }
            }
        }

        impl From<$enum_name> for usize {
            fn from(value: $enum_name) -> usize {
                u8::from(value).into()
            }
        }
    }
}

number_enum! {
    /// The padding of scanlines.
    ///
    /// Each line of an image is padded to a multiple of some value. This value is the scanline
    /// padding, which this enum represents.
    pub enum ScanlinePad {
        /// Padding to multiples of 8 bits, i.e. no padding.
        Pad8 = 8,
        /// Padding to multiples of 16 bits, i.e. the next even number of bytes.
        Pad16 = 16,
        /// Padding to multiples of 32 bits, i.e. the next multiple of 4.
        Pad32 = 32,
    }
}

impl ScanlinePad {
    fn round_to_multiple(self, value: usize) -> usize {
        let value = value + usize::from(self) - 1;
        value - value % usize::from(self)
    }
}

number_enum! {
    /// The number of bits required to store one pixel.
    ///
    /// This value is only about the size of one pixel in memory. Other names for it include
    /// `bits_per_pixel` or `bpp`. It may be larger than the number of meaningful bits for a pixel
    /// value, which is its `depth`.
    pub enum BitsPerPixel {
        /// Each pixel takes one bit of memory.
        B1 = 1,
        /// Each pixel takes four bits of memory.
        B4 = 4,
        /// Each pixel takes one byte of memory.
        B8 = 8,
        /// Each pixel takes two bytes of memory.
        B16 = 16,
        /// Each pixel takes three bytes of memory.
        ///
        /// This is most likely not what you want to use, even if you have RGB data with 8 bits per
        /// channel. This corresponds to `depth=24`, but is almost always stored as four bytes
        /// per pixel, which is `BitsPerPixel::B32`.
        B24 = 24,
        /// Each pixel takes four bytes of memory.
        B32 = 32,
    }
}

/// The description of an image.
#[derive(Debug, PartialEq, Eq)]
pub struct Image<'a> {
    /// Width in pixels.
    width: u16,

    /// Height in pixels.
    height: u16,

    /// Right padding on each scanline.
    scanline_pad: ScanlinePad,

    /// Color depth in bits.
    depth: u8,

    /// Storage per pixel in bits. Must be >= depth.
    bits_per_pixel: BitsPerPixel,

    /// Byte order of components.
    ///
    /// This is the nibble order when bits_per_pixel is 4.
    byte_order: ImageOrder,

    /// The image data.
    data: Cow<'a, [u8]>,
}

impl<'a> ToOwned for Image<'a> {
    // TODO: Can this somehow be turned into Image<'static>?
    type Owned = Self;

    fn to_owned(&self) -> Self::Owned {
        Image {
            width: self.width,
            height: self.height,
            scanline_pad: self.scanline_pad,
            depth: self.depth,
            bits_per_pixel: self.bits_per_pixel,
            byte_order: self.byte_order,
            data: self.data.to_owned(),
        }
    }
}

impl<'a> Image<'a> {
    /// The width in pixels.
    pub fn width(&self) -> u16 {
        self.width
    }

    /// The height in pixels.
    pub fn height(&self) -> u16 {
        self.height
    }

    /// The padding on the right side of each scanline.
    ///
    /// Each scanline is rounded up to some multiple of the `scanline_pad`.
    pub fn scanline_pad(&self) -> ScanlinePad {
        self.scanline_pad
    }

    /// Color depth in bits.
    pub fn depth(&self) -> u8 {
        self.depth
    }

    /// Number of bits required to store one pixel.
    ///
    /// This is always `>= depth`.
    pub fn bits_per_pixel(&self) -> BitsPerPixel {
        self.bits_per_pixel
    }

    /// Order in which bytes are stored in memory.
    ///
    /// If `bits_per_pixel()` is smaller than 8, then this is the order in which bits are packed
    /// into bytes.
    pub fn byte_order(&self) -> ImageOrder {
        self.byte_order
    }

    /// Raw pixel data.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Mutable access to the raw pixel data.
    ///
    /// If the `Image` was constructed with `Cow::Borrowed`-access to its pixel data, then a copy
    /// is made when this method is called.
    pub fn data_mut(&mut self) -> &mut [u8] {
        self.data.to_mut()
    }

    /// Construct a new image from existing data.
    ///
    /// This constructs a new `Image` from given `data` without copying this data around. The other
    /// parameters describe the format that the image data is in.
    ///
    /// See [`Image::allocate`] for a variant that allocates memory for you and
    /// [`Image::allocate_native`] for allocating an image that is in an X11 server's native
    /// format.
    ///
    /// # Errors
    ///
    /// The only possible error is that `data.len()` is too short for an image as described by the
    /// other parameters.
    pub fn new(
        width: u16,
        height: u16,
        scanline_pad: ScanlinePad,
        depth: u8,
        bits_per_pixel: BitsPerPixel,
        byte_order: ImageOrder,
        data: Cow<'a, [u8]>,
    ) -> Result<Self, ParseError> {
        let stride = compute_stride(width, bits_per_pixel, scanline_pad);
        let expected_size = usize::from(height) * stride;
        if data.len() < expected_size {
            Err(ParseError::InsufficientData)
        } else {
            Ok(Self {
                width,
                height,
                scanline_pad,
                depth,
                bits_per_pixel,
                byte_order,
                data
            })
        }
    }

    /// Construct a new, empty image.
    ///
    /// This function allocates memory for a new image in the format that is described by the
    /// parameters.
    ///
    /// See [`Image::new`] for a variant that wraps an existing in-memory image in an `Image` and
    /// [`Image::allocate_native`] for allocating an image that is in an X11 server's native
    /// format.
    pub fn allocate(
        width: u16,
        height: u16,
        scanline_pad: ScanlinePad,
        depth: u8,
        bits_per_pixel: BitsPerPixel,
        byte_order: ImageOrder,
    ) -> Self {
        let stride = compute_stride(width, bits_per_pixel, scanline_pad);
        let data = Cow::Owned(vec![0; usize::from(height) * stride]);
        Self {
            width,
            height,
            scanline_pad,
            depth,
            bits_per_pixel,
            byte_order,
            data,
        }
    }

    /// Construct a new, empty image.
    ///
    /// This function allocates memory for a new image in the format that the X11 server expects.
    /// The image will have size `width`x`height` and a depth of `depth`.
    ///
    /// See [`Image::new`] for a variant that wraps an existing in-memory image in an `Image` and
    /// [`Image::allocate`] for allocating an image that is in a more general format, not
    /// necessarily what the X11 server wants.
    pub fn allocate_native(
        width: u16,
        height: u16,
        depth: u8,
        setup: &Setup,
    ) -> Result<Self, ParseError> {
        let format = find_format(setup, depth.into())?;
        Ok(Self::allocate(
            width,
            height,
            format.scanline_pad.try_into()?,
            depth,
            format.bits_per_pixel.try_into()?,
            ImageOrder::MSBFirst,
        ))
    }

    /// The stride is the number of bytes that each row of pixel data occupies in memory.
    fn stride(&self) -> usize {
        compute_stride(self.width, self.bits_per_pixel, self.scanline_pad)
    }

    /// Get an image from the X11 server.
    ///
    /// This function sends a [`GetImage`](crate::protocol::xproto::GetImage) request, waits for
    /// its response and wraps it in a new `Image`.
    ///
    /// The returned image contains the rectangle with top left corner `(x, y)` and size `(width,
    /// height)` of the given `drawable`.
    pub fn get(
        conn: &impl Connection,
        drawable: Drawable,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<Self, ReplyError> {
        let reply = GetImageRequest {
            drawable,
            x,
            y,
            width,
            height,
            format: ImageFormat::ZPixmap,
            plane_mask: !0,
        }.send(conn)?.reply()?;
        Ok(Self::get_from_reply(&conn.setup(), width, height, reply)?)
    }

    /// Construct an `Image` from a `GetImageReply`.
    ///
    /// This function takes a `GetImageReply` and wraps it in an `Image`. The given `width` and
    /// `height` describe the corresponding values of the `GetImage` request that was used to get
    /// the `GetImageReply`.
    pub fn get_from_reply(
        setup: &Setup,
        width: u16,
        height: u16,
        reply: GetImageReply,
    ) -> Result<Self, ParseError> {
        let format = find_format(setup, reply.depth)?;
        Self::new(
            width,
            height,
            format.scanline_pad.try_into()?,
            reply.depth,
            format.bits_per_pixel.try_into()?,
            ImageOrder::MSBFirst,
            Cow::Owned(reply.data),
        )
    }

    /// Put an image to the X11 server.
    ///
    /// This function sends a [`PutImage`](crate::protocol::xproto::PutImage) request. This will
    /// upload this image to the given `drawable` to position `(dst_x, dst_y)`.
    pub fn put<'c, Conn: Connection>(
        &self,
        conn: &'c Conn,
        drawable: Drawable,
        gc: Gcontext,
        dst_x: i16,
        dst_y: i16,
    ) -> Result<VoidCookie<'c, Conn>, ConnectionError> {
        PutImageRequest {
            format: ImageFormat::ZPixmap,
            drawable,
            gc,
            width: self.width,
            height: self.height,
            dst_x,
            dst_y,
            left_pad: 0, // Must always be 0 for ZPixmap
            depth: self.depth.into(),
            data: Cow::Borrowed(&self.data),
        }.send(conn)
    }

    /// Convert this image into the native format of the X11 server.
    ///
    /// This function may need to copy the image, hence returns a `Cow`.
    pub fn native(&self, setup: &Setup) -> Result<Cow<'_, Self>, ParseError> {
        let format = find_format(setup, self.depth.into())?;
        let is_native =
            format.scanline_pad == self.scanline_pad.into() &&
            format.bits_per_pixel == self.bits_per_pixel.into() &&
            setup.image_byte_order == self.byte_order;
        if is_native {
            Ok(Cow::Borrowed(self))
        } else {
            let mut copy = Image::allocate(
                self.width,
                self.height,
                format.scanline_pad.try_into()?,
                self.depth,
                format.bits_per_pixel.try_into()?,
                ImageOrder::MSBFirst
            );
            // This is the slowest possible way to do this. But also the easiest one to implment.
            for y in 0..self.height {
                for x in 0..self.width {
                    copy.put_pixel(x, y, self.get_pixel(x, y))
                }
            }
            Ok(Cow::Owned(copy))
        }
    }

    /// Set a single pixel in this image.
    ///
    /// The pixel at position `(x, y)` will be set to the value `pixel`. `pixel` is truncated to
    /// this image's [`bits_per_pixel()`].
    ///
    /// If the image was constructed from a `Cow::Borrowed` access to its pixel data, this causes
    /// the whole pixel data to be copied. See [`Image::new`] and [`Image::data_mut`].
    pub fn put_pixel(&mut self, x: u16, y: u16, pixel: u32) {
        assert!(x < self.width);
        assert!(y < self.height);

        let row_start = usize::from(y) * self.stride();
        let x = usize::from(x);
        let data = self.data.to_mut();
        match self.bits_per_pixel {
            BitsPerPixel::B1 => todo!(), // FIXME
            BitsPerPixel::B4 => {
                let mut pixel = pixel & 0x0f;
                let odd_x = x % 2 == 1;
                let mask = if odd_x == (self.byte_order == ImageOrder::MSBFirst) {
                    pixel <<= 4;
                    0xf0
                } else {
                    0x0f
                };
                data[x / 2] = (data[x / 2] & !mask) | (pixel as u8);
            },
            BitsPerPixel::B8 => data[row_start + x] = pixel as u8,
            BitsPerPixel::B16 => {
                let (p0, p1) = match self.byte_order {
                    ImageOrder::LSBFirst => (pixel, pixel >> 8),
                    ImageOrder::MSBFirst => (pixel >> 8, pixel)
                };
                data[row_start + 2*x + 1] = p1 as u8;
                data[row_start + 2*x] = p0 as u8;
            },
            BitsPerPixel::B24 => {
                let (p0, p1, p2) = match self.byte_order {
                    ImageOrder::LSBFirst => (pixel, pixel >> 8, pixel >> 16),
                    ImageOrder::MSBFirst => (pixel >> 16, pixel >> 8, pixel)
                };
                data[row_start + 3*x + 2] = p2 as u8;
                data[row_start + 3*x + 1] = p1 as u8;
                data[row_start + 3*x] = p0 as u8;
            },
            BitsPerPixel::B32 => {
                let (p0, p1, p2, p3) = match self.byte_order {
                    ImageOrder::LSBFirst => (pixel, pixel >> 8, pixel >> 16, pixel >> 24),
                    ImageOrder::MSBFirst => (pixel >> 24, pixel >> 16, pixel >> 8, pixel)
                };
                data[row_start + 4*x + 3] = p3 as u8;
                data[row_start + 4*x + 2] = p2 as u8;
                data[row_start + 4*x + 1] = p1 as u8;
                data[row_start + 4*x] = p0 as u8;
            },
        }
    }

    /// Get the value of a single pixel.
    ///
    /// This function gets the value of the pixel at `(x, y)`.
    pub fn get_pixel(&self, x: u16, y: u16) -> u32 {
        assert!(x < self.width);
        assert!(y < self.height);

        let row_start = usize::from(y) * self.stride();
        let x = usize::from(x);
        // TODO Can this code (and the one in put_pixel) be simplified? E.g. handle B4 as a special
        // case and copy bits_per_pixel.into() / 8 bytes in other cases?
        match self.bits_per_pixel {
            BitsPerPixel::B1 => todo!(), // FIXME
            BitsPerPixel::B4 => {
                let byte = u32::from(self.data[x / 2]);
                let odd_x = x % 2 == 1;
                if odd_x == (self.byte_order == ImageOrder::MSBFirst) {
                    byte >> 4
                } else {
                    byte & 0x0f
                }
            },
            BitsPerPixel::B8 => self.data[row_start + x].into(),
            BitsPerPixel::B16 => {
                let p1 = u32::from(self.data[row_start + 2*x + 1]);
                let p0 = u32::from(self.data[row_start + 2*x]);
                match self.byte_order {
                    ImageOrder::LSBFirst => p0 | (p1 << 8),
                    ImageOrder::MSBFirst => p1 | (p0 << 8),
                }
            },
            BitsPerPixel::B24 => {
                let p2 = u32::from(self.data[row_start + 3*x + 2]);
                let p1 = u32::from(self.data[row_start + 3*x + 1]);
                let p0 = u32::from(self.data[row_start + 3*x]);
                match self.byte_order {
                    ImageOrder::LSBFirst => p0 | (p1 << 8) | (p2 << 16),
                    ImageOrder::MSBFirst => p2 | (p1 << 8) | (p0 << 16),
                }
            },
            BitsPerPixel::B32 => {
                let p3 = u32::from(self.data[row_start + 4*x + 3]);
                let p2 = u32::from(self.data[row_start + 4*x + 2]);
                let p1 = u32::from(self.data[row_start + 4*x + 1]);
                let p0 = u32::from(self.data[row_start + 4*x]);
                match self.byte_order {
                    ImageOrder::LSBFirst => p0 | (p1 << 8) | (p2 << 16) | (p3 << 24),
                    ImageOrder::MSBFirst => p3 | (p2 << 8) | (p1 << 16) | (p0 << 24),
                }
            },
        }
    }
}
