//! Utility functions for working with X11 properties

use std::convert::TryInto;

use crate::connection::RequestConnection;
use crate::cookie::{Cookie, VoidCookie};
use crate::errors::{ConnectionError, ParseError, ReplyError};
use crate::xproto::{Atom, AtomEnum, GetPropertyReply, Window, get_property, self};
use crate::x11_utils::{TryParse, Serialize};

/// A cookie for getting a window's `WM_CLASS` property.
#[derive(Debug)]
pub struct WmClassCookie<'a, Conn: RequestConnection + ?Sized>(Cookie<'a, Conn, GetPropertyReply>);

impl<'a, Conn> WmClassCookie<'a, Conn>
where Conn: RequestConnection + ?Sized
{
    /// Send a `GetProperty` request for the `WM_CLASS` property of the given window
    pub fn new(conn: &'a Conn, window: Window) -> Result<Self, ConnectionError> {
        Ok(Self(get_property(
            conn,
            false,
            window,
            AtomEnum::WM_CLASS.into(),
            AtomEnum::STRING.into(),
            0,
            2048,
        )?))
    }

    /// Get the reply that the server sent.
    pub fn reply(self) -> Result<WmClass, ReplyError<Conn::Buf>> {
        Ok(WmClass::from_reply(self.0.reply()?)?)
    }

    /// Get the reply that the server sent, but have errors handled as events.
    pub fn reply_unchecked(self) -> Result<Option<WmClass>, ConnectionError> {
        self.0.reply_unchecked()?
            .map(|r| WmClass::from_reply(r))
            .transpose()
            .map_err(Into::into)
    }
}

/// The value of a window's `WM_CLASS` property.
#[derive(Debug)]
pub struct WmClass(GetPropertyReply, usize);

impl WmClass {
    /// Send a `GetProperty` request for the `WM_CLASS` property of the given window
    pub fn get<C: RequestConnection>(conn: &C, window: Window) -> Result<WmClassCookie<'_, C>, ConnectionError> {
        WmClassCookie::new(conn, window)
    }

    /// Construct a new `WmClass` instance from a `GetPropertyReply`.
    ///
    /// The original `GetProperty` request must have been for a `WM_CLASS` property for this
    /// function to return sensible results.
    pub fn from_reply(reply: GetPropertyReply) -> Result<Self, ParseError> {
        if reply.type_ != AtomEnum::STRING.into() || reply.format != 8 {
            return Err(ParseError::ParseError);
        }
        // Find the first zero byte in the value
        let offset = reply.value
            .iter()
            .position(|&v| v == 0)
            .unwrap_or_else(|| reply.value.len());
        Ok(WmClass(reply, offset))
    }

    /// Get the instance contained in this `WM_CLASS` property
    pub fn instance(&self) -> &[u8] {
        &self.0.value[0..self.1]
    }

    /// Get the class contained in this `WM_CLASS` property
    pub fn class(&self) -> &[u8] {
        let start = self.1 + 1;
        if start >= self.0.value.len() {
            return &[];
        };
        let end = if self.0.value.last() == Some(&0) {
            self.0.value.len() - 1
        } else {
            self.0.value.len()
        };
        &self.0.value[start..end]
    }
}

/// Representation of whether some part of `WM_SIZE_HINTS` was user/program specified.
#[derive(Debug, Copy, Clone)]
pub enum WmSizeHintsSpecification {
    UserSpecified,
    ProgramSpecified,
}

/// A cookie for getting a window's `WM_SIZE_HINTS` property.
#[derive(Debug)]
pub struct WmSizeHintsCookie<'a, Conn: RequestConnection + ?Sized>(Cookie<'a, Conn, GetPropertyReply>);

const NUM_WM_SIZE_HINTS_ELEMENTS: u32 = 18;

impl<'a, Conn> WmSizeHintsCookie<'a, Conn>
where Conn: RequestConnection + ?Sized
{
    /// Send a `GetProperty` request for the `WM_SIZE_HINTS` property of the given window.
    pub fn new(conn: &'a Conn, window: Window, property: Atom) -> Result<Self, ConnectionError> {
        Ok(Self(get_property(
            conn,
            false,
            window,
            property,
            AtomEnum::WM_SIZE_HINTS.into(),
            0,
            NUM_WM_SIZE_HINTS_ELEMENTS,
        )?))
    }

    /// Get the reply that the server sent.
    pub fn reply(self) -> Result<WmSizeHints, ReplyError<Conn::Buf>> {
        Ok(WmSizeHints::from_reply(self.0.reply()?)?)
    }

    /// Get the reply that the server sent, but have errors handled as events.
    pub fn reply_unchecked(self) -> Result<Option<WmSizeHints>, ConnectionError> {
        self.0.reply_unchecked()?
            .map(|r| WmSizeHints::from_reply(r))
            .transpose()
            .map_err(Into::into)
    }
}

// Possible flags for `WM_SIZE_HINTS`.
const U_S_POSITION: u32 = 1 << 0;
const U_S_SIZE: u32 = 1 << 1;
const P_S_POSITION: u32 = 1 << 2;
const P_S_SIZE: u32 = 1 << 3;
const P_MIN_SIZE: u32 = 1 << 4;
const P_MAX_SIZE: u32 = 1 << 5;
const P_RESIZE_INCREMENT: u32 = 1 << 6;
const P_ASPECT: u32 = 1 << 7;
const P_BASE_SIZE: u32 = 1 << 8;
const P_WIN_GRAVITY: u32 = 1 << 9;

/// An aspect ratio
#[derive(Debug, Copy, Clone)]
pub struct AspectRatio {
    pub numerator: i32,
    pub denominator: i32,
}

impl AspectRatio {
    pub fn new(numerator: i32, denominator: i32) -> Self {
        Self {
            numerator,
            denominator
        }
    }
}

/// A structure representing a `WM_SIZE_HINTS` property.
#[derive(Debug, Default, Copy, Clone)]
pub struct WmSizeHints {
    pub position: Option<(WmSizeHintsSpecification, i32, i32)>,
    pub size: Option<(WmSizeHintsSpecification, i32, i32)>,
    pub min_size: Option<(i32, i32)>,
    pub max_size: Option<(i32, i32)>,
    pub size_increment: Option<(i32, i32)>,
    /// The minimum and maximum aspect ratio
    pub aspect: Option<(AspectRatio, AspectRatio)>,
    pub base_size: Option<(i32, i32)>,
    pub win_gravity: Option<xproto::Gravity>,
}

impl WmSizeHints {
    /// Get a new, empty `WmSizeHints` structure.
    pub fn new() -> Self {
        Default::default()
    }

    /// Send a `GetProperty` request for the `WM_SIZE_HINTS` property of the given window
    pub fn get<C: RequestConnection>(conn: &C, window: Window, property: Atom) -> Result<WmSizeHintsCookie<'_, C>, ConnectionError> {
        WmSizeHintsCookie::new(conn, window, property)
    }

    /// Construct a new `WmSizeHints` instance from a `GetPropertyReply`.
    ///
    /// The original `WmSizeHints` request must have been for a `WM_SIZE_HINTS` property for this
    /// function to return sensible results.
    pub fn from_reply(reply: GetPropertyReply) -> Result<Self, ParseError> {
        // Implemented based on what xcb_icccm does. At least a bit. This stuff makes no sense...

        if reply.type_ != AtomEnum::WM_SIZE_HINTS.into() || reply.format != 32 {
            return Err(ParseError::ParseError);
        }

        let remaining = &reply.value;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (x, remaining) = i32::try_parse(remaining)?;
        let (y, remaining) = i32::try_parse(remaining)?;
        let (width, remaining) = i32::try_parse(remaining)?;
        let (height, remaining) = i32::try_parse(remaining)?;
        let (min_width, remaining) = i32::try_parse(remaining)?;
        let (min_height, remaining) = i32::try_parse(remaining)?;
        let (max_width, remaining) = i32::try_parse(remaining)?;
        let (max_height, remaining) = i32::try_parse(remaining)?;
        let (width_increment, remaining) = i32::try_parse(remaining)?;
        let (height_increment, remaining) = i32::try_parse(remaining)?;
        let (min_aspect_num, remaining) = i32::try_parse(remaining)?;
        let (min_aspect_den, remaining) = i32::try_parse(remaining)?;
        let (max_aspect_num, remaining) = i32::try_parse(remaining)?;
        let (max_aspect_den, remaining) = i32::try_parse(remaining)?;
        // Apparently, some older version of ICCCM didn't have these...?
        let (base_width, base_height, wire_win_gravity) = if remaining.is_empty() {
            (min_width, min_height, 1)
        } else {
            let (base_width, remaining) = i32::try_parse(remaining)?;
            let (base_height, remaining) = i32::try_parse(remaining)?;
            let (wire_win_gravity, _remaining) = u32::try_parse(remaining)?;
            (base_width, base_height, wire_win_gravity)
        };

        // FIXME: Move this into the code generator
        let win_gravity = match wire_win_gravity {
            1 => xproto::Gravity::NorthWest,
            2 => xproto::Gravity::North,
            3 => xproto::Gravity::NorthEast,
            4 => xproto::Gravity::West,
            5 => xproto::Gravity::Center,
            6 => xproto::Gravity::East,
            7 => xproto::Gravity::SouthWest,
            8 => xproto::Gravity::South,
            9 => xproto::Gravity::SouthEast,
            10 => xproto::Gravity::Static,
            // BitForget and WinUnmap are not allowed here
            _ => return Err(ParseError::ParseError),
        };
        assert_eq!(wire_win_gravity, win_gravity.into());

        let position = if flags & U_S_POSITION != 0 {
            Some((WmSizeHintsSpecification::UserSpecified, x, y))
        } else if flags & P_S_POSITION != 0 {
            Some((WmSizeHintsSpecification::ProgramSpecified, x, y))
        } else {
            None
        };
        let size = if flags & U_S_SIZE != 0 {
            Some((WmSizeHintsSpecification::UserSpecified, width, height))
        } else if flags & P_S_SIZE != 0 {
            Some((WmSizeHintsSpecification::ProgramSpecified, width, height))
        } else {
            None
        };
        let min_size = if flags & P_MIN_SIZE != 0 {
            Some((min_width, min_height))
        } else {
            None
        };
        let max_size = if flags & P_MAX_SIZE != 0 {
            Some((max_width, max_height))
        } else {
            None
        };
        let size_increment = if flags & P_RESIZE_INCREMENT != 0 {
            Some((width_increment, height_increment))
        } else {
            None
        };
        let aspect = if flags & P_ASPECT != 0 {
            let min_aspect = AspectRatio::new(min_aspect_num, min_aspect_den);
            let max_aspect = AspectRatio::new(max_aspect_num, max_aspect_den);
            Some((min_aspect, max_aspect))
        } else {
            None
        };
        let base_size = if flags & P_BASE_SIZE != 0 {
            Some((base_width, base_height))
        } else {
            None
        };
        let win_gravity = if flags & P_WIN_GRAVITY != 0 {
            Some(win_gravity)
        } else {
            None
        };

        Ok(WmSizeHints {
            position,
            size,
            min_size,
            max_size,
            size_increment,
            aspect,
            base_size,
            win_gravity,
        })
    }

    /// Set these `WM_SIZE_HINTS` on some window.
    pub fn set<'a, C: RequestConnection + ?Sized>(&self, conn: &'a C, window: Window, property: Atom) -> Result<VoidCookie<'a, C>, ConnectionError> {
        // 18*4 surely fits into an usize, so this unwrap() cannot trigger
        let mut data = Vec::with_capacity((NUM_WM_SIZE_HINTS_ELEMENTS * 4).try_into().unwrap());

        let mut flags = 0;
        match self.position {
            Some((WmSizeHintsSpecification::UserSpecified, _, _)) => flags |= U_S_POSITION,
            Some((WmSizeHintsSpecification::ProgramSpecified, _, _)) => flags |= P_S_POSITION,
            None => {},
        }
        match self.size {
            Some((WmSizeHintsSpecification::UserSpecified, _, _)) => flags |= U_S_SIZE,
            Some((WmSizeHintsSpecification::ProgramSpecified, _, _)) => flags |= P_S_SIZE,
            None => {},
        }
        if self.min_size.is_some() {
            flags |= P_MIN_SIZE;
        }
        if self.max_size.is_some() {
            flags |= P_MAX_SIZE;
        }
        if self.size_increment.is_some() {
            flags |= P_RESIZE_INCREMENT;
        }
        if self.aspect.is_some() {
            flags |= P_ASPECT;
        }
        if self.base_size.is_some() {
            flags |= P_BASE_SIZE;
        }
        if self.win_gravity.is_some() {
            flags |= P_WIN_GRAVITY;
        }

        flags.serialize_into(&mut data);

        let (x, y) = match self.position {
            Some((_, x, y)) => (x, y),
            None => (0, 0),
        };
        x.serialize_into(&mut data);
        y.serialize_into(&mut data);

        let (width, height) = match self.size {
            Some((_, width, height)) => (width, height),
            None => (0, 0),
        };
        width.serialize_into(&mut data);
        height.serialize_into(&mut data);

        let (min_width, min_height) = self.min_size.unwrap_or((0, 0));
        min_width.serialize_into(&mut data);
        min_height.serialize_into(&mut data);

        let (max_width, max_height) = self.max_size.unwrap_or((0, 0));
        max_width.serialize_into(&mut data);
        max_height.serialize_into(&mut data);

        let (width_inc, height_inc) = self.size_increment.unwrap_or((0, 0));
        width_inc.serialize_into(&mut data);
        height_inc.serialize_into(&mut data);

        let (min_aspect, max_aspect) = self.aspect.unwrap_or((AspectRatio::new(0, 0), AspectRatio::new(0, 0)));
        min_aspect.numerator.serialize_into(&mut data);
        min_aspect.denominator.serialize_into(&mut data);
        max_aspect.numerator.serialize_into(&mut data);
        max_aspect.denominator.serialize_into(&mut data);

        let (base_width, base_height) = self.base_size.unwrap_or((0, 0));
        base_width.serialize_into(&mut data);
        base_height.serialize_into(&mut data);

        let gravity = self.win_gravity.map_or(0, u32::from);
        gravity.serialize_into(&mut data);

        xproto::change_property(
            conn,
            xproto::PropMode::Replace,
            window,
            property,
            AtomEnum::WM_SIZE_HINTS,
            32,
            NUM_WM_SIZE_HINTS_ELEMENTS,
            &data,
        )
    }
}

// WM_HINTS

/// A cookie for getting a window's `WM_HINTS` property.
#[derive(Debug)]
pub struct WmHintsCookie<'a, Conn: RequestConnection + ?Sized>(Cookie<'a, Conn, GetPropertyReply>);

const NUM_WM_HINTS_ELEMENTS: u32 = 9;

impl<'a, Conn> WmHintsCookie<'a, Conn>
where Conn: RequestConnection + ?Sized
{
    /// Send a `GetProperty` request for the `WM_CLASS` property of the given window
    pub fn new(conn: &'a Conn, window: Window) -> Result<Self, ConnectionError> {
        Ok(Self(get_property(
            conn,
            false,
            window,
            AtomEnum::WM_HINTS.into(),
            AtomEnum::WM_HINTS.into(),
            0,
            NUM_WM_HINTS_ELEMENTS,
        )?))
    }

    /// Get the reply that the server sent.
    pub fn reply(self) -> Result<WmHints, ReplyError<Conn::Buf>> {
        Ok(WmHints::from_reply(self.0.reply()?)?)
    }

    /// Get the reply that the server sent, but have errors handled as events.
    pub fn reply_unchecked(self) -> Result<Option<WmHints>, ConnectionError> {
        self.0.reply_unchecked()?
            .map(|r| WmHints::from_reply(r))
            .transpose()
            .map_err(Into::into)
    }
}

/// The possible values for a `WM_STATE`'s state field.
#[derive(Debug, Copy, Clone)]
pub enum WmStateState {
    Normal,
    Iconic,
}

// Possible flags for `WM_HINTS`.
const HINT_INPUT: u32 = 1 << 0;
const HINT_STATE: u32 = 1 << 1;
const HINT_ICON_PIXMAP: u32 = 1 << 2;
const HINT_ICON_WINDOW: u32 = 1 << 3;
const HINT_ICON_POSITION: u32 = 1 << 4;
const HINT_ICON_MASK: u32 = 1 << 5;
const HINT_WINDOW_GROUP: u32 = 1 << 6;
// This bit is obsolete, according to ICCCM
//const HINT_MESSAGE: u32 = 1 << 7;
const HINT_URGENCY: u32 = 1 << 8;

/// A structure representing a `WM_HINTS` property.
#[derive(Debug, Default, Copy, Clone)]
pub struct WmHints {
    pub input: Option<bool>,
    pub initial_state: Option<WmStateState>,
    pub icon_pixmap: Option<xproto::Pixmap>,
    pub icon_window: Option<Window>,
    pub icon_position: Option<(i32, i32)>,
    pub icon_mask: Option<xproto::Pixmap>,
    pub window_group: Option<Window>,
    pub urgent: bool,
}

impl WmHints {
    /// Get a new, empty `WmSizeHints` structure.
    pub fn new() -> Self {
        Default::default()
    }

    /// Send a `GetProperty` request for the `WM_HINTS` property of the given window
    pub fn get<C: RequestConnection>(conn: &C, window: Window) -> Result<WmHintsCookie<'_, C>, ConnectionError> {
        WmHintsCookie::new(conn, window)
    }

    /// Construct a new `WmHints` instance from a `GetPropertyReply`.
    ///
    /// The original `WmHints` request must have been for a `WM_HINTS` property for this
    /// function to return sensible results.
    pub fn from_reply(reply: GetPropertyReply) -> Result<Self, ParseError> {
        if reply.type_ != AtomEnum::WM_HINTS.into() || reply.format != 32 {
            return Err(ParseError::ParseError);
        }

        let remaining = &reply.value;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (input, remaining) = u32::try_parse(remaining)?;
        let (initial_state, remaining) = u32::try_parse(remaining)?;
        let (icon_pixmap, remaining) = u32::try_parse(remaining)?;
        let (icon_window, remaining) = u32::try_parse(remaining)?;
        let (icon_x, remaining) = i32::try_parse(remaining)?;
        let (icon_y, remaining) = i32::try_parse(remaining)?;
        let (icon_mask, remaining) = u32::try_parse(remaining)?;
        // Apparently, some older version of ICCCM didn't have tis...?
        let window_group = if remaining.is_empty() {
            0
        } else {
            let (window_group, _remaining) = u32::try_parse(remaining)?;
            window_group
        };

        let input = if flags & HINT_INPUT != 0 {
            Some(input != 0)
        } else {
            None
        };
        let initial_state = if flags & HINT_STATE != 0 {
            Some(match initial_state {
                1 => WmStateState::Normal,
                3 => WmStateState::Iconic,
                _ => return Err(ParseError::ParseError),
            })
        } else {
            None
        };
        let icon_pixmap = if flags & HINT_ICON_PIXMAP != 0 {
            Some(icon_pixmap)
        } else {
            None
        };
        let icon_window = if flags & HINT_ICON_WINDOW != 0 {
            Some(icon_window)
        } else {
            None
        };
        let icon_position = if flags & HINT_ICON_POSITION != 0 {
            Some((icon_x, icon_y))
        } else {
            None
        };
        let icon_mask = if flags & HINT_ICON_MASK != 0 {
            Some(icon_mask)
        } else {
            None
        };
        let window_group = if flags & HINT_WINDOW_GROUP != 0 {
            Some(window_group)
        } else {
            None
        };
        let urgent = flags & HINT_URGENCY != 0;

        Ok(WmHints {
            input,
            initial_state,
            icon_pixmap,
            icon_window,
            icon_position,
            icon_mask,
            window_group,
            urgent,
        })
    }

    /// Set these `WM_HINTS` on some window.
    pub fn set<'a, C: RequestConnection + ?Sized>(&self, conn: &'a C, window: Window) -> Result<VoidCookie<'a, C>, ConnectionError> {
        // 9*4 surely fits into an usize, so this unwrap() cannot trigger
        let mut data = Vec::with_capacity((NUM_WM_HINTS_ELEMENTS * 4).try_into().unwrap());

        let mut flags = 0;
        if self.input.is_some() {
            flags |= HINT_INPUT;
        }
        if self.initial_state.is_some() {
            flags |= HINT_STATE;
        }
        if self.icon_pixmap.is_some() {
            flags |= HINT_ICON_PIXMAP;
        }
        if self.icon_window.is_some() {
            flags |= HINT_ICON_WINDOW;
        }
        if self.icon_position.is_some() {
            flags |= HINT_ICON_POSITION;
        }
        if self.icon_mask.is_some() {
            flags |= HINT_ICON_MASK;
        }
        if self.window_group.is_some() {
            flags |= HINT_WINDOW_GROUP;
        }
        if self.urgent {
            flags |= HINT_URGENCY;
        }

        flags.serialize_into(&mut data);
        self.input.unwrap_or(false).serialize_into(&mut data);
        let initial_state = match self.initial_state {
            Some(WmStateState::Normal) => 1,
            Some(WmStateState::Iconic) => 3,
            None => 0,
        };
        initial_state.serialize_into(&mut data);
        self.icon_pixmap.unwrap_or(0).serialize_into(&mut data);
        self.icon_window.unwrap_or(0).serialize_into(&mut data);
        let (icon_x, icon_y) = self.icon_position.unwrap_or((0, 0));
        icon_x.serialize_into(&mut data);
        icon_y.serialize_into(&mut data);
        self.icon_mask.unwrap_or(0).serialize_into(&mut data);
        self.window_group.unwrap_or(0).serialize_into(&mut data);

        xproto::change_property(
            conn,
            xproto::PropMode::Replace,
            window,
            xproto::AtomEnum::WM_HINTS,
            xproto::AtomEnum::WM_HINTS,
            32,
            NUM_WM_HINTS_ELEMENTS,
            &data,
        )
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryInto;

    use crate::xproto::{AtomEnum, GetPropertyReply};
    use super::WmClass;

    fn get_property_reply(value: &[u8]) -> GetPropertyReply {
        GetPropertyReply {
            response_type: 1,
            format: 8,
            sequence: 0,
            length: 0,
            type_: AtomEnum::STRING.into(),
            bytes_after: 0,
            value_len: value.len().try_into().unwrap(),
            value: value.to_vec(),
        }
    }

    #[test]
    fn test_wm_class() {
        for (input, instance, class) in &[
            (&b""[..], &b""[..], &b""[..]),
            (b"\0", b"", b""),
            (b"\0\0", b"", b""),
            (b"\0\0\0", b"", b"\0"),
            (b"Hello World", b"Hello World", b""),
            (b"Hello World\0", b"Hello World", b""),
            (b"Hello\0World\0", b"Hello", b"World"),
            (b"Hello\0World", b"Hello", b"World"),
            (b"Hello\0World\0Good\0Day", b"Hello", b"World\0Good\0Day"),
        ] {
            let wm_class = WmClass::from_reply(get_property_reply(input)).unwrap();
            assert_eq!((wm_class.instance(), wm_class.class()), (*instance, *class));
        }
    }
}
