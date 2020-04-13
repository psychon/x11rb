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
    pub fn new(conn: &'a Conn, window: Window, property: impl Into<Atom>) -> Result<Self, ConnectionError> {
        Ok(Self(get_property(
            conn,
            false,
            window,
            property.into(),
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

impl TryParse for AspectRatio {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let ((numerator, denominator), remaining) = TryParse::try_parse(value)?;
        let result = AspectRatio::new(numerator, denominator);
        Ok((result, remaining))
    }
}

impl Serialize for AspectRatio {
    type Bytes = [u8; 8];
    fn serialize(&self) -> Self::Bytes {
        let [a, b, c, d] = self.numerator.serialize();
        let [e, f, g, h] = self.denominator.serialize();
        [a, b, c, d, e, f, g, h]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        (self.numerator, self.denominator).serialize_into(bytes);
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

    /// Send a `GetProperty` request for the given property of the given window
    pub fn get<C: RequestConnection>(conn: &C, window: Window, property: impl Into<Atom>) -> Result<WmSizeHintsCookie<'_, C>, ConnectionError> {
        WmSizeHintsCookie::new(conn, window, property)
    }

    /// Send a `GetProperty` request for the `WM_NORMAL_HINTS` property of the given window
    pub fn get_normal_hints<C: RequestConnection>(conn: &C, window: Window) -> Result<WmSizeHintsCookie<'_, C>, ConnectionError> {
        Self::get(conn, window, AtomEnum::WM_NORMAL_HINTS)
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
        let (min_size, remaining) = parse_with_flag::<(i32, i32)>(remaining, flags, P_MIN_SIZE)?;
        let (max_size, remaining) = parse_with_flag::<(i32, i32)>(remaining, flags, P_MAX_SIZE)?;
        let (size_increment, remaining) = parse_with_flag::<(i32, i32)>(remaining, flags, P_RESIZE_INCREMENT)?;
        let (aspect, remaining) = parse_with_flag::<(AspectRatio, AspectRatio)>(remaining, flags, P_ASPECT)?;
        // Apparently, some older version of ICCCM didn't have these...?
        let (base_size, wire_win_gravity) = if remaining.is_empty() {
            (min_size, Some(1))
        } else {
            let (base_size, remaining) = parse_with_flag::<(i32, i32)>(remaining, flags, P_BASE_SIZE)?;
            let (wire_win_gravity, _remaining) = parse_with_flag::<u32>(remaining, flags, P_WIN_GRAVITY)?;
            (base_size, wire_win_gravity)
        };

        // FIXME: Move this into the code generator
        let win_gravity = match wire_win_gravity {
            None => None,
            Some(1) => Some(xproto::Gravity::NorthWest),
            Some(2) => Some(xproto::Gravity::North),
            Some(3) => Some(xproto::Gravity::NorthEast),
            Some(4) => Some(xproto::Gravity::West),
            Some(5) => Some(xproto::Gravity::Center),
            Some(6) => Some(xproto::Gravity::East),
            Some(7) => Some(xproto::Gravity::SouthWest),
            Some(8) => Some(xproto::Gravity::South),
            Some(9) => Some(xproto::Gravity::SouthEast),
            Some(10) => Some(xproto::Gravity::Static),
            // BitForget and WinUnmap are not allowed here
            _ => return Err(ParseError::ParseError),
        };
        assert_eq!(wire_win_gravity, win_gravity.map(Into::into));

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

    /// Set these `WM_SIZE_HINTS` on some window as the `WM_NORMAL_HINTS` property.
    pub fn set_normal_hints<'a, C: RequestConnection + ?Sized>(&self, conn: &'a C, window: Window) -> Result<VoidCookie<'a, C>, ConnectionError> {
        self.set(conn, window, AtomEnum::WM_NORMAL_HINTS)
    }

    /// Set these `WM_SIZE_HINTS` on some window as the given property.
    pub fn set<'a, C: RequestConnection + ?Sized>(&self, conn: &'a C, window: Window, property: impl Into<Atom>) -> Result<VoidCookie<'a, C>, ConnectionError> {
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
        flags |= self.min_size.map_or(0, |_| P_MIN_SIZE);
        flags |= self.max_size.map_or(0, |_| P_MAX_SIZE);
        flags |= self.size_increment.map_or(0, |_| P_RESIZE_INCREMENT);
        flags |= self.aspect.map_or(0, |_| P_ASPECT);
        flags |= self.base_size.map_or(0, |_| P_BASE_SIZE);
        flags |= self.win_gravity.map_or(0, |_| P_WIN_GRAVITY);
        flags.serialize_into(&mut data);

        match self.position {
            Some((_, x, y)) => (x, y),
            None => (0, 0),
        }.serialize_into(&mut data);

         match self.size {
            Some((_, width, height)) => (width, height),
            None => (0, 0),
        }.serialize_into(&mut data);

        self.min_size.unwrap_or((0, 0)).serialize_into(&mut data);
        self.max_size.unwrap_or((0, 0)).serialize_into(&mut data);
        self.size_increment.unwrap_or((0, 0)).serialize_into(&mut data);
        self.aspect.unwrap_or((AspectRatio::new(0, 0), AspectRatio::new(0, 0))).serialize_into(&mut data);
        self.base_size.unwrap_or((0, 0)).serialize_into(&mut data);
        self.win_gravity.map_or(0, u32::from).serialize_into(&mut data);

        xproto::change_property(
            conn,
            xproto::PropMode::Replace,
            window,
            property.into(),
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
        let (input, remaining) = parse_with_flag::<u32>(remaining, flags, HINT_INPUT)?;
        let (initial_state, remaining) = parse_with_flag::<u32>(remaining, flags, HINT_STATE)?;
        let (icon_pixmap, remaining) = parse_with_flag::<u32>(remaining, flags, HINT_ICON_PIXMAP)?;
        let (icon_window, remaining) = parse_with_flag::<u32>(remaining, flags, HINT_ICON_WINDOW)?;
        let (icon_position, remaining) = parse_with_flag::<(i32, i32)>(remaining, flags, HINT_ICON_POSITION)?;
        let (icon_mask, remaining) = parse_with_flag::<u32>(remaining, flags, HINT_ICON_MASK)?;
        // Apparently, some older version of ICCCM didn't have this...?
        let window_group = if remaining.is_empty() {
            None
        } else {
            let (window_group, _remaining) = parse_with_flag::<u32>(remaining, flags, HINT_WINDOW_GROUP)?;
            window_group
        };

        let input = input.map(|input| input != 0);

        let initial_state = match initial_state {
            None => None,
            Some(1) => Some(WmStateState::Normal),
            Some(3) => Some(WmStateState::Iconic),
            _ => return Err(ParseError::ParseError),
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
        flags |= self.input.map_or(0, |_| HINT_INPUT);
        flags |= self.initial_state.map_or(0, |_| HINT_STATE);
        flags |= self.icon_pixmap.map_or(0, |_| HINT_ICON_PIXMAP);
        flags |= self.icon_window.map_or(0, |_| HINT_ICON_WINDOW);
        flags |= self.icon_position.map_or(0, |_| HINT_ICON_POSITION);
        flags |= self.icon_mask.map_or(0, |_| HINT_ICON_MASK);
        flags |= self.window_group.map_or(0, |_| HINT_WINDOW_GROUP);
        if self.urgent {
            flags |= HINT_URGENCY;
        }

        flags.serialize_into(&mut data);
        self.input.unwrap_or(false).serialize_into(&mut data);
        match self.initial_state {
            Some(WmStateState::Normal) => 1,
            Some(WmStateState::Iconic) => 3,
            None => 0,
        }.serialize_into(&mut data);
        self.icon_pixmap.unwrap_or(0).serialize_into(&mut data);
        self.icon_window.unwrap_or(0).serialize_into(&mut data);
        self.icon_position.unwrap_or((0, 0)).serialize_into(&mut data);
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

/// Parse an element of type `T` and turn it into an `Option` by checking if the given `bit` is set
/// in `flags`.
fn parse_with_flag<T: TryParse>(remaining: &[u8], flags: u32, bit: u32) -> Result<(Option<T>, &[u8]), ParseError> {
    let (value, remaining) = T::try_parse(remaining)?;
    if flags & bit != 0 {
        Ok((Some(value), remaining))
    } else {
        Ok((None, remaining))
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
