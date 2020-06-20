//! Utility functions for working with X11 cursors
//!
//! The code in this module is only available when the `cursor` feature of the library is enabled.

#![allow(unused_results)]

use crate::connection::Connection;
use crate::cookie::Cookie as X11Cookie;
use crate::errors::{ConnectionError, ReplyOrIdError};
use crate::protocol::render::{self, Pictformat};
use crate::protocol::xproto::{self, Font, Window};
use crate::NONE;

use std::fs::File;

mod find_cursor;
mod parse_cursor;

/// The level of cursor support of the X11 server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderSupport {
    /// Render extension not available
    None,

    /// Static cursor support (CreateCursor added in RENDER 0.5)
    StaticCursor,

    /// Animated cursor support (CreateAnimCursor added in RENDER 0.8)
    AnimatedCursor,
}

/// A cookie for creating a `Handle`
#[derive(Debug)]
pub struct Cookie<'a, C: Connection> {
    conn: &'a C,
    screen: &'a xproto::Screen,
    resource_manager: X11Cookie<'a, C, xproto::GetPropertyReply>,
    render_info: Option<(
        X11Cookie<'a, C, render::QueryVersionReply>,
        X11Cookie<'a, C, render::QueryPictFormatsReply>,
    )>,
}

impl<C: Connection> Cookie<'_, C> {
    /// Get the handle from the replies from the X11 server
    pub fn reply(self) -> Result<Handle, ReplyOrIdError> {
        let resource_manager = self.resource_manager.reply()?;
        let mut render_version = (0, 0);
        let mut picture_format = NONE;
        if let Some((version, formats)) = self.render_info {
            let version = version.reply()?;
            render_version = (version.major_version, version.minor_version);
            picture_format = find_format(&formats.reply()?);
        }
        Ok(Self::from_replies(
            self.conn,
            self.screen,
            &resource_manager,
            render_version,
            picture_format,
        )?)
    }

    /// Get the handle from the replies from the X11 server
    pub fn reply_unchecked(self) -> Result<Option<Handle>, ReplyOrIdError> {
        let resource_manager = self.resource_manager.reply_unchecked()?;
        let mut render_version = (0, 0);
        let mut picture_format = NONE;
        if let Some((version, formats)) = self.render_info {
            match (version.reply_unchecked()?, formats.reply_unchecked()?) {
                (Some(version), Some(formats)) => {
                    render_version = (version.major_version, version.minor_version);
                    picture_format = find_format(&formats);
                }
                _ => return Ok(None),
            }
        }
        let resource_manager = match resource_manager {
            None => return Ok(None),
            Some(resource_manager) => resource_manager,
        };
        Ok(Some(Self::from_replies(
            self.conn,
            self.screen,
            &resource_manager,
            render_version,
            picture_format,
        )?))
    }

    fn from_replies(
        conn: &C,
        screen: &xproto::Screen,
        resource_manager: &xproto::GetPropertyReply,
        render_version: (u32, u32),
        picture_format: Pictformat,
    ) -> Result<Handle, ReplyOrIdError> {
        let render_support = if render_version.0 >= 1 || render_version.1 >= 8 {
            RenderSupport::AnimatedCursor
        } else if render_version.0 >= 1 || render_version.1 >= 5 {
            RenderSupport::StaticCursor
        } else {
            RenderSupport::None
        };
        let (theme, cursor_size, xft_dpi) = parse_resource_manager(&resource_manager.value);
        let cursor_size = get_cursor_size(cursor_size, xft_dpi, screen);
        let cursor_font = conn.generate_id()?;
        xproto::open_font(conn, cursor_font, b"cursor")?;
        Ok(Handle {
            root: screen.root,
            cursor_font,
            picture_format,
            render_support,
            theme,
            cursor_size,
        })
    }
}

/// A handle necessary for loading cursors
#[derive(Debug)]
pub struct Handle {
    root: Window,
    cursor_font: Font,
    picture_format: Pictformat,
    render_support: RenderSupport,
    theme: Option<String>,
    cursor_size: u32,
}

impl Handle {
    /// Create a new cursor handle for creating cursors on the given screen.
    ///
    /// This function returns a cookie that can be used to later get the actual handle.
    ///
    /// If you want this function not to block, you should prefetch the RENDER extension's data on
    /// the connection.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<C: Connection>(conn: &C, screen: usize) -> Result<Cookie<'_, C>, ConnectionError> {
        let screen = &conn.setup().roots[screen];
        let root = screen.root;
        let resource_manager = xproto::get_property(
            conn,
            false,
            root,
            xproto::AtomEnum::RESOURCE_MANAGER,
            xproto::AtomEnum::STRING,
            0,
            1 << 20,
        )?;
        let mut render_info = None;
        if conn
            .extension_information(render::X11_EXTENSION_NAME)?
            .is_some()
        {
            let render_version = render::query_version(conn, 0, 8)?;
            let render_pict_format = render::query_pict_formats(conn)?;
            render_info = Some((render_version, render_pict_format));
        }
        Ok(Cookie {
            conn,
            screen,
            resource_manager,
            render_info,
        })
    }

    /// Loads the specified cursor, either from the cursor theme or by falling back to the X11
    /// "cursor" font.
    pub fn load_cursor<C>(&self, conn: &C, name: &str) -> Result<xproto::Cursor, ReplyOrIdError>
    where
        C: Connection,
    {
        load_cursor(conn, self, name)
    }
}

fn open_cursor(theme: &Option<String>, name: &str) -> Option<find_cursor::Cursor<File>> {
    if let Some(theme) = theme {
        if let Ok(cursor) = find_cursor::find_cursor(theme, name) {
            return Some(cursor);
        }
    }
    if let Ok(cursor) = find_cursor::find_cursor("default", name) {
        Some(cursor)
    } else {
        None
    }
}

fn create_core_cursor<C: Connection>(
    conn: &C,
    cursor_font: Font,
    cursor: u16,
) -> Result<xproto::Cursor, ReplyOrIdError> {
    let result = conn.generate_id()?;
    xproto::create_glyph_cursor(
        conn,
        result,
        cursor_font,
        cursor_font,
        cursor,
        cursor + 1,
        // foreground color
        0,
        0,
        0,
        // background color
        u16::max_value(),
        u16::max_value(),
        u16::max_value(),
    )?;
    Ok(result)
}

fn create_render_cursor<C: Connection>(
    conn: &C,
    handle: &Handle,
    image: &parse_cursor::Image,
    storage: &mut Option<(xproto::Pixmap, xproto::Gcontext, u16, u16)>,
) -> Result<render::Animcursorelt, ReplyOrIdError> {
    let (cursor, picture) = (conn.generate_id()?, conn.generate_id()?);

    // Get a pixmap of the right size and a gc for it
    let (pixmap, gc) = if storage.map(|(_, _, w, h)| (w, h)) == Some((image.width, image.height)) {
        storage.map(|(pixmap, gc, _, _)| (pixmap, gc)).unwrap()
    } else {
        let (pixmap, gc) = if let Some((pixmap, gc, _, _)) = storage {
            xproto::free_gc(conn, *gc)?;
            xproto::free_pixmap(conn, *pixmap)?;
            (*pixmap, *gc)
        } else {
            (conn.generate_id()?, conn.generate_id()?)
        };
        xproto::create_pixmap(conn, 32, pixmap, handle.root, image.width, image.height)?;
        xproto::create_gc(conn, gc, pixmap, &Default::default())?;

        *storage = Some((pixmap, gc, image.width, image.height));
        (pixmap, gc)
    };

    // Sigh. We need the pixel data as a bunch of bytes.
    let pixels = crate::x11_utils::Serialize::serialize(&image.pixels[..]);
    xproto::put_image(
        conn,
        xproto::ImageFormat::ZPixmap,
        pixmap,
        gc,
        image.width,
        image.height,
        0,
        0,
        0,
        32,
        &pixels,
    )?;

    render::create_picture(
        conn,
        picture,
        pixmap,
        handle.picture_format,
        &Default::default(),
    )?;
    render::create_cursor(conn, cursor, picture, image.x_hot, image.y_hot)?;
    render::free_picture(conn, picture)?;

    Ok(render::Animcursorelt {
        cursor,
        delay: image.delay,
    })
}

fn load_cursor<C: Connection>(
    conn: &C,
    handle: &Handle,
    name: &str,
) -> Result<xproto::Cursor, ReplyOrIdError> {
    // Find the right cursor, load it directly if it is a core cursor
    let cursor_file = match open_cursor(&handle.theme, name) {
        None => return Ok(NONE),
        Some(find_cursor::Cursor::CoreChar(c)) => {
            return create_core_cursor(conn, handle.cursor_font, c)
        }
        Some(find_cursor::Cursor::File(f)) => f,
    };

    // We have to load a file and use RENDER to create a cursor
    if handle.render_support == RenderSupport::None {
        return Ok(NONE);
    }

    // Load the cursor from the file
    use std::io::BufReader;
    let images =
        parse_cursor::parse_cursor(&mut BufReader::new(cursor_file), handle.cursor_size)
            .or(Err(crate::errors::ParseError::ParseError))?;
    let mut images = &images[..];

    // No animated cursor support? Only use the first image
    if handle.render_support == RenderSupport::StaticCursor {
        images = &images[0..1];
    }

    // Now transfer the cursors to the X11 server
    let mut storage = None;
    let cursors = images
        .iter()
        .map(|image| create_render_cursor(conn, handle, image, &mut storage))
        .collect::<Result<Vec<_>, _>>()?;
    if let Some((pixmap, gc, _, _)) = storage {
        xproto::free_gc(conn, gc)?;
        xproto::free_pixmap(conn, pixmap)?;
    }

    if cursors.len() == 1 {
        Ok(cursors[0].cursor)
    } else {
        let result = conn.generate_id()?;
        render::create_anim_cursor(conn, result, &cursors)?;
        for elem in cursors {
            xproto::free_cursor(conn, elem.cursor)?;
        }
        Ok(result)
    }
}

fn find_format(reply: &render::QueryPictFormatsReply) -> Pictformat {
    reply
        .formats
        .iter()
        .filter(|format| {
            format.type_ == render::PictType::Direct
                && format.depth == 32
                && format.direct.red_shift == 16
                && format.direct.red_mask == 0xff
                && format.direct.green_shift == 8
                && format.direct.green_mask == 0xff
                && format.direct.blue_shift == 0
                && format.direct.blue_mask == 0xff
                && format.direct.alpha_shift == 24
                && format.direct.alpha_mask == 0xff
        })
        .map(|format| format.id)
        .next()
        .expect("The X11 server is missing the RENDER ARGB_32 standard format!")
}

fn parse_resource_manager(rm: &[u8]) -> (Option<String>, u32, u32) {
    let (mut theme, mut cursor_size, mut xft_dpi) = (None, 0, 0);

    for line in rm.split(|&c| c == b'\n') {
        // Split line at first ':'
        let pos = match line.iter().enumerate().find(|&(_, &c)| c == b':') {
            None => continue,
            Some((pos, _)) => pos,
        };
        let (key, mut value) = line.split_at(pos);

        // Skip leading whitespace
        while value.get(0).map(|&c| char::from(c).is_whitespace()) == Some(true) {
            value = &value[1..];
        }
        // Skip trailing whitespace
        while value.last().map(|&c| char::from(c).is_whitespace()) == Some(true) {
            value = &value[..value.len() - 1];
        }

        if let Ok(value) = std::str::from_utf8(value) {
            match key {
                b"Xcursor.theme" => theme = Some(value.to_string()),
                b"Xcursor.size" => {
                    if let Ok(num) = value.parse() {
                        cursor_size = num;
                    }
                }
                b"Xft.dpi" => {
                    if let Ok(num) = value.parse() {
                        xft_dpi = num;
                    }
                }
                _ => {}
            }
        }
    }

    (theme, cursor_size, xft_dpi)
}

fn get_cursor_size(rm_cursor_size: u32, rm_xft_dpi: u32, screen: &xproto::Screen) -> u32 {
    if let Some(size) = std::env::var("XCURSOR_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
    {
        return size;
    }
    if rm_cursor_size > 0 {
        return rm_cursor_size;
    }
    if rm_xft_dpi > 0 {
        return rm_xft_dpi * 16 / 72;
    }
    u32::from(screen.height_in_pixels.min(screen.width_in_pixels) / 48)
}
