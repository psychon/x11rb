use std::borrow::Cow;
use std::convert::TryInto;

use x11rb::connection::Connection;
use x11rb::errors::{ParseError, ReplyOrIdError};
use x11rb::image::Image;
use x11rb::protocol::xproto::{
    AtomEnum, ConnectionExt, CreateGCAux, CreateWindowAux, PropMode, Screen, Setup, VisualClass,
    Visualid, Visualtype, Window, WindowClass,
};
use x11rb::protocol::Event;
use x11rb::wrapper::ConnectionExt as _;

x11rb::atom_manager! {
    Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
    }
}

/// The description of a single color component.
///
/// For example, in an RGB image, pixels are often saved as `0xRRGGBB`, where each letter
/// represents the respective color component. In the example, green has a `width` of 8 (it takes
/// up 8 bits) and a `shift` of `16` (there are 16 less significant bits beyond it). This info is
/// represented as a `ColorComponent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorComponent {
    /// Number of bits for the component
    width: u8,
    /// Offset in an u32 of the component
    shift: u8,
}

impl ColorComponent {
    /// Create a new color component with the given information.
    pub fn new(width: u8, shift: u8) -> Self {
        assert!(
            width < 16,
            "At most 16 bits for a single color component supported, got {}",
            width,
        );
        assert!(
            shift < 32,
            "Too large shift for color component, got {}, but pixels are 32 bit",
            shift,
        );
        assert!(
            shift + width <= 32,
            "Would overflow a 32 bit value when putting {} bits at offset {}",
            width,
            shift,
        );
        Self { width, shift }
    }

    /// Get the pixel mask representing this color component.
    ///
    /// The mask can be used to mask off other colors in a pixel value. Only the bits that
    /// correspond to this color component are set.
    /// ```
    /// let red = ColorComponent::new(8, 16);
    /// assert_eq(red.to_mask(), 0xff0000);
    /// ```
    fn to_mask(self) -> u32 {
        // Get a mask with 'width' set bits.
        let mask = (1u32 << self.width) - 1;
        // Shift the mask into the right position
        mask << self.shift
    }

    /// Create a new color component from a color mask.
    ///
    /// This turns a color mask into its individual components.
    /// ```
    /// let red1 = ColorComponent::new(8, 16);
    /// let red2 = ColorComponent::from_mask(0xff0000);
    /// ```
    pub fn from_mask(mask: u32) -> Self {
        let width = mask.count_ones();
        let shift = mask.trailing_zeros();
        // Both width and shift can be at most 32, which should fit into u8.
        let result = Self::new(width.try_into().unwrap(), shift.try_into().unwrap());
        assert_eq!(mask, result.to_mask(), "Malformed mask {}", mask);
        result
    }

    /// Get this color component out of a pixel value.
    ///
    /// This function gets a single pixel value and returns this color component's value in that
    /// pixel value, expanded to width 16.
    /// ```
    /// let red = ColorComponent::new(8, 16);
    /// assert_eq!(0xABAB, red.decode(0x78AB_4321));
    /// panic!("are these even run?");
    /// ```
    pub fn decode(self, pixel: u32) -> u16 {
        // Get the color component out
        let value = (pixel & self.to_mask()) >> self.shift;

        // Now expand from with `self.width` to width 16.
        let mut width = self.width;
        let mut value = value << (16 - width);
        // Add some low bits by using the high bits
        while width < 16 {
            value |= value >> width;
            width <<= 1;
        }
        value.try_into().unwrap()
    }

    /// Encode a color value according to this pixel format.
    ///
    /// ```
    /// let red = ColorComponent::new(8, 16);
    /// assert_eq!(0xAB0000, red.encode(0xABCD));
    /// ```
    pub fn encode(self, intensity: u16) -> u32 {
        // First truncate to width `self.width`, then place at the correct offset.
        (u32::from(intensity) >> (16 - self.width)) << self.shift
    }
}

/// A collection of color components describing the red, green, and blue components of a pixel.
///
/// A [`ColorComponent`] describes a single color component in an image. This structure describes
/// the `red`, `green`, and `blue` color components by containing a [`ColorComponent`] for each of
/// them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PixelLayout {
    red: ColorComponent,
    green: ColorComponent,
    blue: ColorComponent,
}

impl PixelLayout {
    /// Create a new pixel layout from the description of each component
    pub fn new(red: ColorComponent, green: ColorComponent, blue: ColorComponent) -> Self {
        Self { red, green, blue }
    }

    /// Create a new pixel layout
    pub fn from_visual_type(visual: Visualtype) -> Self {
        if visual.class != VisualClass::TrueColor && visual.class != VisualClass::DirectColor {
            panic!("Visual does not use decomposed colors: {:x?}", visual);
        }
        Self::new(
            ColorComponent::from_mask(visual.red_mask),
            ColorComponent::from_mask(visual.green_mask),
            ColorComponent::from_mask(visual.blue_mask),
        )
    }

    /// Get the depth of this pixel layout.
    ///
    /// The depth is the number of significant bits of each pixel value.
    pub fn depth(self) -> u8 {
        // TODO: I am not quite sure this implementation is correct. The protocol seems to allow
        // unused bits in the middle..?
        self.red.width + self.green.width + self.blue.width
    }

    /// Decode a pixel value into its red, green, and blue components.
    ///
    /// This function returns each component expanded to width 16.
    /// ```
    /// let layout = PixelLayout::new(
    ///     ColorComponent::new(8, 16),
    ///     ColorComponent::new(8, 8),
    ///     ColorComponent::new(8, 0),
    /// );
    /// assert_eq!((0xABAB, 0x4343, 0x2121), layout.decode(0x78AB_4321));
    /// ```
    pub fn decode(self, pixel: u32) -> (u16, u16, u16) {
        let red = self.red.decode(pixel);
        let green = self.green.decode(pixel);
        let blue = self.blue.decode(pixel);
        (red, green, blue)
    }

    /// Encode a color value according to this layout.
    ///
    /// ```
    /// let layout = PixelLayout::new(
    ///     ColorComponent::new(8, 16),
    ///     ColorComponent::new(8, 8),
    ///     ColorComponent::new(8, 0),
    /// );
    /// assert_eq!(0x78AB_4321, layout.encode((0xABAB, 0x4343, 0x2121)));
    /// ```
    pub fn encode(self, (red, green, blue): (u16, u16, u16)) -> u32 {
        let red = self.red.encode(red);
        let green = self.green.encode(green);
        let blue = self.blue.encode(blue);
        red | green | blue
    }
}

/// Convert the given `image` to another layout and storage.
///
/// This function gets an `image` with pixel values that can be interpreted according to
/// `input_layout`. It produces an image with pixel values according to `output_layout`. The
/// resulting image is stored in the native format of the X11 server described by `setup`.
fn convert_image<'image, 'data>(
    image: &'image Image<'data>,
    setup: &Setup,
    input_layout: PixelLayout,
    output_layout: PixelLayout,
) -> Result<Cow<'image, Image<'data>>, ParseError> {
    if input_layout == output_layout {
        image.native(setup)
    } else {
        // Yay, we get to convert the image :-(
        let (width, height) = (image.width(), image.height());
        let mut result = Image::allocate_native(width, height, output_layout.depth(), setup)?;
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let pixel = output_layout.encode(input_layout.decode(pixel));
                result.put_pixel(x, y, pixel);
            }
        }
        Ok(Cow::Owned(result))
    }
}

/// Create a window with the given image as background.
fn create_window(
    conn: &impl Connection,
    screen: &Screen,
    atoms: &Atoms,
    image: &Image,
) -> Result<Window, ReplyOrIdError> {
    let win_id = conn.generate_id()?;
    let pixmap_id = conn.generate_id()?;
    let gc_id = conn.generate_id()?;

    conn.create_gc(
        gc_id,
        screen.root,
        &CreateGCAux::default().graphics_exposures(0),
    )?;
    conn.create_pixmap(
        screen.root_depth,
        pixmap_id,
        screen.root,
        image.width(),
        image.height(),
    )?;
    image.put(conn, pixmap_id, gc_id, 0, 0)?;
    conn.free_gc(gc_id)?;

    conn.create_window(
        screen.root_depth,
        win_id,
        screen.root,
        0,
        0,
        image.width(),
        image.height(),
        0,
        WindowClass::InputOutput,
        0,
        &CreateWindowAux::default().background_pixmap(pixmap_id),
    )?;
    conn.free_pixmap(pixmap_id)?;

    conn.change_property32(
        PropMode::Replace,
        win_id,
        atoms.WM_PROTOCOLS,
        AtomEnum::ATOM,
        &[atoms.WM_DELETE_WINDOW],
    )?;

    Ok(win_id)
}

/// Check that the given visual is "as expected" (pixel values are 0xRRGGBB with RR/GG/BB being the
/// colors). Otherwise, this exits the process.
fn check_visual(screen: &Screen, id: Visualid) -> PixelLayout {
    // Find the information about the visual and at the same time check its depth.
    let visual_info = screen
        .allowed_depths
        .iter()
        .filter_map(|depth| {
            let info = depth.visuals.iter().find(|depth| depth.visual_id == id);
            info.map(|info| (depth.depth, info))
        })
        .next();
    let (depth, visual_type) = match visual_info {
        Some(info) => info,
        None => {
            eprintln!("Did not find the root visual's description?!");
            std::process::exit(1);
        }
    };
    // Check that the pixels have red/green/blue components that we can set directly.
    match visual_type.class {
        VisualClass::TrueColor | VisualClass::DirectColor => {}
        _ => {
            eprintln!(
                "The root visual is not true / direct color, but {:?}",
                visual_type,
            );
            std::process::exit(1);
        }
    }
    let result = PixelLayout::from_visual_type(*visual_type);
    assert_eq!(result.depth(), depth);
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the image
    let image = match std::env::args_os().nth(1) {
        None => {
            eprintln!(
                "Expected a file name of a PPM as argument, using a built-in default image instead"
            );
            ppm_parser::parse_ppm_bytes(&BUILTIN_IMAGE)?
        }
        Some(arg) => ppm_parser::parse_ppm_file(&arg)?,
    };

    let (conn, screen_num) = x11rb::connect(None)?;

    // The following is only needed for start_timeout_thread(), which is used for 'tests'
    let conn1 = std::sync::Arc::new(conn);
    let conn = &*conn1;

    let screen = &conn.setup().roots[screen_num];
    let pixel_layout = check_visual(screen, screen.root_visual);

    // Convert the image from the PPM format into the server's native format.
    let ppm_layout = PixelLayout::new(
        ColorComponent::new(8, 16),
        ColorComponent::new(8, 8),
        ColorComponent::new(8, 0),
    );
    let image = convert_image(&image, conn.setup(), ppm_layout, pixel_layout)?;

    let atoms = Atoms::new(conn)?.reply()?;
    let win_id = create_window(conn, screen, &atoms, &image)?;
    conn.map_window(win_id)?;

    util::start_timeout_thread(conn1.clone(), win_id);

    conn.flush()?;

    loop {
        let event = conn.wait_for_event().unwrap();
        match event {
            Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32 && event.window == win_id && data[0] == atoms.WM_DELETE_WINDOW
                {
                    println!("Window was asked to close");
                    return Ok(());
                }
            }
            Event::Error(err) => println!("Got an unexpected error: {:?}", err),
            ev => println!("Got an unknown event: {:?}", ev),
        }
    }
}

mod ppm_parser {
    use std::ffi::OsStr;
    use std::io::{Error as IOError, ErrorKind, Read, Result as IOResult};

    use x11rb::image::{BitsPerPixel, Image, ScanlinePad};
    use x11rb::protocol::xproto::ImageOrder;

    fn make_io_error(text: &'static str) -> IOError {
        IOError::new(ErrorKind::Other, text)
    }

    /// Read until the next b'\n'.
    fn read_to_end_of_line(input: &mut impl Read) -> IOResult<()> {
        let mut byte = [0; 1];
        loop {
            input.read_exact(&mut byte)?;
            if byte[0] == b'\n' {
                return Ok(());
            }
        }
    }

    /// Read a decimal number from the input.
    fn read_decimal(input: &mut impl Read) -> IOResult<u16> {
        let mut byte = [0; 1];

        // Skip leading whitespace and comments
        loop {
            input.read_exact(&mut byte)?;
            match byte[0] {
                b' ' | b'\t' | b'\r' => {}
                // Comment, skip a whole line
                b'#' => read_to_end_of_line(input)?,
                _ => break,
            }
        }

        // Now comes a number
        if !byte[0].is_ascii_digit() {
            return Err(make_io_error("Failed parsing a number"));
        }

        let mut result: u16 = 0;
        while byte[0].is_ascii_digit() {
            let value = u16::from(byte[0] - b'0');
            result = result
                .checked_mul(10)
                .map(|result| result + value)
                .ok_or_else(|| make_io_error("Overflow while parsing number"))?;

            input.read_exact(&mut byte)?;
        }

        // After the number, there should be some whitespace.
        if byte[0].is_ascii_whitespace() {
            Ok(result)
        } else {
            Err(make_io_error("Unexpected character in header"))
        }
    }

    fn parse_ppm(input: &mut impl Read) -> IOResult<Image<'static>> {
        let mut header = [0; 2];
        input.read_exact(&mut header)?;
        if header != *b"P6" {
            return Err(make_io_error("Incorrect file header"));
        }
        read_to_end_of_line(input)?;
        let width = read_decimal(input)?;
        let height = read_decimal(input)?;
        let max = read_decimal(input)?;

        if max != 255 {
            eprintln!(
                "Image declares a max pixel value of {}, but I expected 255.",
                max,
            );
            eprintln!("Something will happen...?");
        }

        let mut image = Image::allocate(
            width,
            height,
            ScanlinePad::Pad8,
            24,
            BitsPerPixel::B24,
            ImageOrder::MSBFirst,
        );
        input.read_exact(image.data_mut())?;

        Ok(image)
    }

    pub fn parse_ppm_bytes(bytes: &[u8]) -> IOResult<Image<'static>> {
        use std::io::Cursor;

        parse_ppm(&mut Cursor::new(bytes))
    }

    pub fn parse_ppm_file(file_name: &OsStr) -> IOResult<Image<'static>> {
        use std::fs::File;
        use std::io::BufReader;

        parse_ppm(&mut BufReader::new(File::open(file_name)?))
    }
}

// Simple builtin PPM that is used if none is provided on the command line
#[rustfmt::skip]
const BUILTIN_IMAGE: [u8; 35] = [
    b'P', b'6', b'\n',
    // width and height
    b'4', b' ', b'2', b'\n',
    b'2', b'5', b'5', b'\n',
    // Black pixel
    0x00, 0x00, 0x00,
    // red pixel
    0xff, 0x00, 0x00,
    // green pixel
    0x00, 0xff, 0x00,
    // blue pixel
    0x00, 0x00, 0xff,
    // white pixel
    0xff, 0xff, 0xff,
    // cyan pixel
    0x00, 0xff, 0xff,
    // magenta pixel
    0xff, 0x00, 0xff,
    // yellow pixel
    0xff, 0xff, 0x00,
];

include!("integration_test_util/util.rs");
