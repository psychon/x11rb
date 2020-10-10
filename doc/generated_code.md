# Examples of the generated code

This crate uses a code generator to generate rust code for the X11 protocol. You
might be curious what the generated code looks like. This document is there to
answer this question.

As you may know, the code generator uses an XML description of the X11 protocol
from `xcb-proto`. This document will show some examples of the XML description
followed by the Rust code that is generated for it.

The following code is generated at the beginning of a module:
```rust
// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the core X11 protocol.
//!
//! For more documentation on the X11 protocol, see the
//! [protocol reference manual](https://www.x.org/releases/X11R7.6/doc/xproto/x11protocol.html).
//! This is especially recommended for looking up the exact semantics of
//! specific errors, events, or requests.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]

#[allow(unused_imports)]
use std::borrow::Cow;
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use crate::connection::{BufWithFds, PiecewiseBuf, RequestConnection};
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::cookie::ListFontsWithInfoCookie;
use crate::errors::{ConnectionError, ParseError};
```

## XID types

XIDs simply are numbers. They uniquely identify, for example, a window.

```xml
<xidtype name="WINDOW" />
```

Since all XIDs are 32 bit numbers, the generated code is a type alias:

```rust
pub type Window = u32;
```

## Structs

### Fixed length structs

Structs are used as building blocks for other things. For example, a request
could send a list of structs to the X11 server.
```xml
<struct name="POINT">
  <field type="INT16" name="x" />
  <field type="INT16" name="y" />
</struct>
```
The server can send structs to us. For this reason, there is a `TryParse` trait
that is implemented on structs. This trait is used by the generated code, for
example to parse a list of `Point`s. For easier use, additionally `TryFrom` is
implemented for some inputs.

We must also be able to send structs to the server. This is handled through the
`Serialize` trait that produces data in the native endian.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
impl TryParse for Point {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (x, remaining) = i16::try_parse(remaining)?;
        let (y, remaining) = i16::try_parse(remaining)?;
        let result = Point { x, y };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Point {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Point {
    type Bytes = [u8; 4];
    fn serialize(&self) -> [u8; 4] {
        let x_bytes = self.x.serialize();
        let y_bytes = self.y.serialize();
        [
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(4);
        self.x.serialize_into(bytes);
        self.y.serialize_into(bytes);
    }
}
```

### Variable length structs

Structs can be a bit more complicated than the above example, because they can
contain lists of other structs. This example demonstrates this.
```xml
<struct name="DEPTH">
  <field type="CARD8" name="depth" />
  <pad bytes="1" />
  <field type="CARD16" name="visuals_len" />
  <pad bytes="4" />
  <list type="VISUALTYPE" name="visuals">
    <fieldref>visuals_len</fieldref>
  </list>
</struct>
```
The field `visuals_len` is not part of the generated struct since it is
represented implicitly as the length of the `visuals` `Vec`. To make this less
confusing, a function `visuals_len` is generated.
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Depth {
    pub depth: u8,
    pub visuals: Vec<Visualtype>,
}
impl TryParse for Depth {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (depth, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let (visuals_len, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::InsufficientData)?;
        let (visuals, remaining) = crate::x11_utils::parse_list::<Visualtype>(remaining, visuals_len.try_into().or(Err(ParseError::ConversionFailed))?)?;
        let result = Depth { depth, visuals };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for Depth {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for Depth {
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(8);
        self.depth.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 1]);
        let visuals_len = u16::try_from(self.visuals.len()).expect("`visuals` has too many elements");
        visuals_len.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.visuals.serialize_into(bytes);
    }
}
impl Depth {
    /// Get the value of the `visuals_len` field.
    ///
    /// The `visuals_len` field is used as the length field of the `visuals` field.
    /// This function computes the field's value again based on the length of the list.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented in the target type. This
    /// cannot happen with values of the struct received from the X11 server.
    pub fn visuals_len(&self) -> u16 {
        self.visuals.len()
            .try_into().unwrap()
    }
}
```

## Enumerations

### 'Real' enumerations

```xml
<enum name="BackingStore">
  <item name="NotUseful"> <value>0</value></item>
  <item name="WhenMapped"><value>1</value></item>
  <item name="Always">    <value>2</value></item>
</enum>
```
Depending on the largest value, appropriate `From` and `TryFrom` implementations
are generated.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum BackingStore {
    NotUseful = 0,
    WhenMapped = 1,
    Always = 2,
}
impl From<BackingStore> for u8 {
    fn from(input: BackingStore) -> Self {
        match input {
            BackingStore::NotUseful => 0,
            BackingStore::WhenMapped => 1,
            BackingStore::Always => 2,
        }
    }
}
impl From<BackingStore> for Option<u8> {
    fn from(input: BackingStore) -> Self {
        Some(u8::from(input))
    }
}
impl From<BackingStore> for u16 {
    fn from(input: BackingStore) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BackingStore> for Option<u16> {
    fn from(input: BackingStore) -> Self {
        Some(u16::from(input))
    }
}
impl From<BackingStore> for u32 {
    fn from(input: BackingStore) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<BackingStore> for Option<u32> {
    fn from(input: BackingStore) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for BackingStore {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BackingStore::NotUseful),
            1 => Ok(BackingStore::WhenMapped),
            2 => Ok(BackingStore::Always),
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for BackingStore {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for BackingStore {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
```

### Bitmask enumerations

Bitmasks also get an invocation of the `bitmask_binop!` macro. This creates
implementations of `BitOr` and `BitOrAssign`.
```xml
<enum name="ConfigWindow">
  <item name="X">          <bit>0</bit></item>
  <item name="Y">          <bit>1</bit></item>
  <item name="Width">      <bit>2</bit></item>
  <item name="Height">     <bit>3</bit></item>
  <item name="BorderWidth"><bit>4</bit></item>
  <item name="Sibling">    <bit>5</bit></item>
  <item name="StackMode">  <bit>6</bit></item>
</enum>
```

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[repr(u8)]
#[non_exhaustive]
pub enum ConfigWindow {
    X = 1 << 0,
    Y = 1 << 1,
    Width = 1 << 2,
    Height = 1 << 3,
    BorderWidth = 1 << 4,
    Sibling = 1 << 5,
    StackMode = 1 << 6,
}
impl From<ConfigWindow> for u8 {
    fn from(input: ConfigWindow) -> Self {
        match input {
            ConfigWindow::X => 1 << 0,
            ConfigWindow::Y => 1 << 1,
            ConfigWindow::Width => 1 << 2,
            ConfigWindow::Height => 1 << 3,
            ConfigWindow::BorderWidth => 1 << 4,
            ConfigWindow::Sibling => 1 << 5,
            ConfigWindow::StackMode => 1 << 6,
        }
    }
}
impl From<ConfigWindow> for Option<u8> {
    fn from(input: ConfigWindow) -> Self {
        Some(u8::from(input))
    }
}
impl From<ConfigWindow> for u16 {
    fn from(input: ConfigWindow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ConfigWindow> for Option<u16> {
    fn from(input: ConfigWindow) -> Self {
        Some(u16::from(input))
    }
}
impl From<ConfigWindow> for u32 {
    fn from(input: ConfigWindow) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ConfigWindow> for Option<u32> {
    fn from(input: ConfigWindow) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ConfigWindow {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ConfigWindow::X),
            2 => Ok(ConfigWindow::Y),
            4 => Ok(ConfigWindow::Width),
            8 => Ok(ConfigWindow::Height),
            16 => Ok(ConfigWindow::BorderWidth),
            32 => Ok(ConfigWindow::Sibling),
            64 => Ok(ConfigWindow::StackMode),
            _ => Err(ParseError::InvalidValue),
        }
    }
}
impl TryFrom<u16> for ConfigWindow {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
impl TryFrom<u32> for ConfigWindow {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::InvalidValue))?)
    }
}
bitmask_binop!(ConfigWindow, u8);
```

## Unions

Sadly, there are unions in X11. In core X11, there is only one union:
ClientMessageData.

```xml
<union name="ClientMessageData">
  <!-- The format member of the ClientMessage event determines which array
       to use. -->
  <list type="CARD8"  name="data8" ><value>20</value></list> <!--  8 -->
  <list type="CARD16" name="data16"><value>10</value></list> <!-- 16 -->
  <list type="CARD32" name="data32"><value>5</value></list>  <!-- 32 -->
</union>
```

The union contains an array of unparsed data. The data is then parsed as the
requested type in the accessor functions.

```rust
#[derive(Debug, Copy, Clone)]
pub struct ClientMessageData([u8; 20]);
impl ClientMessageData {
    pub fn as_data8(&self) -> [u8; 20] {
        fn do_the_parse(remaining: &[u8]) -> Result<[u8; 20], ParseError> {
            let (data8, remaining) = crate::x11_utils::parse_u8_list(remaining, 20)?;
            let data8 = <[u8; 20]>::try_from(data8).unwrap();
            let _ = remaining;
            Ok(data8)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_data16(&self) -> [u16; 10] {
        fn do_the_parse(remaining: &[u8]) -> Result<[u16; 10], ParseError> {
            let (data16_0, remaining) = u16::try_parse(remaining)?;
            let (data16_1, remaining) = u16::try_parse(remaining)?;
            let (data16_2, remaining) = u16::try_parse(remaining)?;
            let (data16_3, remaining) = u16::try_parse(remaining)?;
            let (data16_4, remaining) = u16::try_parse(remaining)?;
            let (data16_5, remaining) = u16::try_parse(remaining)?;
            let (data16_6, remaining) = u16::try_parse(remaining)?;
            let (data16_7, remaining) = u16::try_parse(remaining)?;
            let (data16_8, remaining) = u16::try_parse(remaining)?;
            let (data16_9, remaining) = u16::try_parse(remaining)?;
            let data16 = [
                data16_0,
                data16_1,
                data16_2,
                data16_3,
                data16_4,
                data16_5,
                data16_6,
                data16_7,
                data16_8,
                data16_9,
            ];
            let _ = remaining;
            Ok(data16)
        }
        do_the_parse(&self.0).unwrap()
    }
    pub fn as_data32(&self) -> [u32; 5] {
        fn do_the_parse(remaining: &[u8]) -> Result<[u32; 5], ParseError> {
            let (data32_0, remaining) = u32::try_parse(remaining)?;
            let (data32_1, remaining) = u32::try_parse(remaining)?;
            let (data32_2, remaining) = u32::try_parse(remaining)?;
            let (data32_3, remaining) = u32::try_parse(remaining)?;
            let (data32_4, remaining) = u32::try_parse(remaining)?;
            let data32 = [
                data32_0,
                data32_1,
                data32_2,
                data32_3,
                data32_4,
            ];
            let _ = remaining;
            Ok(data32)
        }
        do_the_parse(&self.0).unwrap()
    }
}
impl Serialize for ClientMessageData {
    type Bytes = [u8; 20];
    fn serialize(&self) -> [u8; 20] {
        self.0
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0);
    }
}
impl TryParse for ClientMessageData {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner: [u8; 20] = value.get(..20)
            .ok_or(ParseError::InsufficientData)?
            .try_into()
            .unwrap();
        let result = ClientMessageData(inner);
        Ok((result, &value[20..]))
    }
}
impl From<[u8; 20]> for ClientMessageData {
    fn from(data8: [u8; 20]) -> Self {
        Self(data8)
    }
}
impl From<[u16; 10]> for ClientMessageData {
    fn from(data16: [u16; 10]) -> Self {
        let data16_0_bytes = data16[0].serialize();
        let data16_1_bytes = data16[1].serialize();
        let data16_2_bytes = data16[2].serialize();
        let data16_3_bytes = data16[3].serialize();
        let data16_4_bytes = data16[4].serialize();
        let data16_5_bytes = data16[5].serialize();
        let data16_6_bytes = data16[6].serialize();
        let data16_7_bytes = data16[7].serialize();
        let data16_8_bytes = data16[8].serialize();
        let data16_9_bytes = data16[9].serialize();
        let value = [
            data16_0_bytes[0],
            data16_0_bytes[1],
            data16_1_bytes[0],
            data16_1_bytes[1],
            data16_2_bytes[0],
            data16_2_bytes[1],
            data16_3_bytes[0],
            data16_3_bytes[1],
            data16_4_bytes[0],
            data16_4_bytes[1],
            data16_5_bytes[0],
            data16_5_bytes[1],
            data16_6_bytes[0],
            data16_6_bytes[1],
            data16_7_bytes[0],
            data16_7_bytes[1],
            data16_8_bytes[0],
            data16_8_bytes[1],
            data16_9_bytes[0],
            data16_9_bytes[1],
        ];
        Self(value)
    }
}
impl From<[u32; 5]> for ClientMessageData {
    fn from(data32: [u32; 5]) -> Self {
        let data32_0_bytes = data32[0].serialize();
        let data32_1_bytes = data32[1].serialize();
        let data32_2_bytes = data32[2].serialize();
        let data32_3_bytes = data32[3].serialize();
        let data32_4_bytes = data32[4].serialize();
        let value = [
            data32_0_bytes[0],
            data32_0_bytes[1],
            data32_0_bytes[2],
            data32_0_bytes[3],
            data32_1_bytes[0],
            data32_1_bytes[1],
            data32_1_bytes[2],
            data32_1_bytes[3],
            data32_2_bytes[0],
            data32_2_bytes[1],
            data32_2_bytes[2],
            data32_2_bytes[3],
            data32_3_bytes[0],
            data32_3_bytes[1],
            data32_3_bytes[2],
            data32_3_bytes[3],
            data32_4_bytes[0],
            data32_4_bytes[1],
            data32_4_bytes[2],
            data32_4_bytes[3],
        ];
        Self(value)
    }
}
```

## Events

```xml
<event name="KeyPress" number="2">
  <field type="KEYCODE" name="detail" />
  <field type="TIMESTAMP" name="time" />
  <field type="WINDOW" name="root" />
  <field type="WINDOW" name="event" />
  <field type="WINDOW" name="child" altenum="Window" />
  <field type="INT16" name="root_x" />
  <field type="INT16" name="root_y" />
  <field type="INT16" name="event_x" />
  <field type="INT16" name="event_y" />
  <field type="CARD16" name="state" mask="KeyButMask" />
  <field type="BOOL" name="same_screen" />
  <pad bytes="1" />
  <doc>[SNIP]</doc>
</event>
```

```rust
/// Opcode for the KeyPress event
pub const KEY_PRESS_EVENT: u8 = 2;
/// [SNIP]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyPressEvent {
    pub response_type: u8,
    pub detail: Keycode,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
}
impl TryParse for KeyPressEvent {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (detail, remaining) = Keycode::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (time, remaining) = Timestamp::try_parse(remaining)?;
        let (root, remaining) = Window::try_parse(remaining)?;
        let (event, remaining) = Window::try_parse(remaining)?;
        let (child, remaining) = Window::try_parse(remaining)?;
        let (root_x, remaining) = i16::try_parse(remaining)?;
        let (root_y, remaining) = i16::try_parse(remaining)?;
        let (event_x, remaining) = i16::try_parse(remaining)?;
        let (event_y, remaining) = i16::try_parse(remaining)?;
        let (state, remaining) = u16::try_parse(remaining)?;
        let (same_screen, remaining) = bool::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let result = KeyPressEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen };
        let _ = remaining;
        let remaining = initial_value.get(32..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<&KeyPressEvent> for [u8; 32] {
    fn from(input: &KeyPressEvent) -> Self {
        let response_type_bytes = input.response_type.serialize();
        let detail_bytes = input.detail.serialize();
        let sequence_bytes = input.sequence.serialize();
        let time_bytes = input.time.serialize();
        let root_bytes = input.root.serialize();
        let event_bytes = input.event.serialize();
        let child_bytes = input.child.serialize();
        let root_x_bytes = input.root_x.serialize();
        let root_y_bytes = input.root_y.serialize();
        let event_x_bytes = input.event_x.serialize();
        let event_y_bytes = input.event_y.serialize();
        let state_bytes = input.state.serialize();
        let same_screen_bytes = input.same_screen.serialize();
        [
            response_type_bytes[0],
            detail_bytes[0],
            sequence_bytes[0],
            sequence_bytes[1],
            time_bytes[0],
            time_bytes[1],
            time_bytes[2],
            time_bytes[3],
            root_bytes[0],
            root_bytes[1],
            root_bytes[2],
            root_bytes[3],
            event_bytes[0],
            event_bytes[1],
            event_bytes[2],
            event_bytes[3],
            child_bytes[0],
            child_bytes[1],
            child_bytes[2],
            child_bytes[3],
            root_x_bytes[0],
            root_x_bytes[1],
            root_y_bytes[0],
            root_y_bytes[1],
            event_x_bytes[0],
            event_x_bytes[1],
            event_y_bytes[0],
            event_y_bytes[1],
            state_bytes[0],
            state_bytes[1],
            same_screen_bytes[0],
            0,
        ]
    }
}
impl From<KeyPressEvent> for [u8; 32] {
    fn from(input: KeyPressEvent) -> Self {
        Self::from(&input)
    }
}
```

## Errors

```xml
<error name="Request" number="1">
  <field type="CARD32" name="bad_value" />
  <field type="CARD16" name="minor_opcode" />
  <field type="CARD8" name="major_opcode" />
  <pad bytes="1" />
</error>
```

All X11 errors contain the same fields. Thus, we only need to remember which
number represents this error.
```rust
/// Opcode for the Request error
pub const REQUEST_ERROR: u8 = 1;
```
The actual representation of an X11 error can be found in
[`x11rb::x11_utils::X11Error`](../src/x11_utils.rs).

## Requests

For requests, we generate an extension trait. Individual requests are available
on this trait and as global functions. The generic structure looks like this:
```rust
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    // Following code examples are in here
}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}
```

### Request without a reply

```xml
<request name="NoOperation" opcode="127" />
The request is represented by a structure that contains all of the request's
fields. This `struct` can be constructed explicitly and then `.send()` to an X11
server.
```
This code is generated in the module:
```rust
/// Opcode for the NoOperation request
pub const NO_OPERATION_REQUEST: u8 = 127;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoOperationRequest;
impl NoOperationRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let _ = conn;
        let length_so_far = 0;
        let mut request0 = vec![
            NO_OPERATION_REQUEST,
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.major_opcode != NO_OPERATION_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let remaining = &[header.minor_opcode];
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        let _ = value;
        Ok(NoOperationRequest
        )
    }
}
```
A trait is used to map between requests and their corresponding reply. For
requests without a reply, this maps to the unit type:
```rust
impl Request for NoOperationRequest {
    type Reply = ();
}
```
There is also a helper function for sending the request with a function call.
```rust
pub fn no_operation<Conn>(conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = NoOperationRequest;
    request0.send(conn)
}
```
The request sending function is also available on the extension trait:
```rust
    fn no_operation(&self) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        no_operation(self)
    }
```

### Request with a reply

```xml
<request name="GetInputFocus" opcode="43">
  <reply>
    <field type="CARD8" name="revert_to" enum="InputFocus" />
    <field type="WINDOW" name="focus" altenum="InputFocus" />
  </reply>
</request>
```
There is again a structure generated in the code that represents the request:
```rust
/// Opcode for the GetInputFocus request
pub const GET_INPUT_FOCUS_REQUEST: u8 = 43;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetInputFocusRequest;
impl GetInputFocusRequest {
    /// Serialize this request into bytes for the provided connection
    fn serialize<'input, Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let _ = conn;
        let length_so_far = 0;
        let mut request0 = vec![
            GET_INPUT_FOCUS_REQUEST,
            0,
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<Cookie<'_, Conn, GetInputFocusReply>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_with_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &[u8]) -> Result<Self, ParseError> {
        if header.major_opcode != GET_INPUT_FOCUS_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let remaining = &[header.minor_opcode];
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        let _ = value;
        Ok(GetInputFocusRequest
        )
    }
}
```
Since this request has a reply, the implementation of the `Request` trait maps
to that reply:
```rust
impl Request for GetInputFocusRequest {
    type Reply = GetInputFocusReply;
}
```
Of course, there is a function to send the request:
```rust
pub fn get_input_focus<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetInputFocusReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetInputFocusRequest;
    request0.send(conn)
}
```
The reply is handled similar to a `struct`:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetInputFocusReply {
    pub revert_to: InputFocus,
    pub sequence: u16,
    pub length: u32,
    pub focus: Window,
}
impl TryParse for GetInputFocusReply {
    fn try_parse(initial_value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let remaining = initial_value;
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (revert_to, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (focus, remaining) = Window::try_parse(remaining)?;
        if response_type != 1 {
            return Err(ParseError::InvalidValue);
        }
        let revert_to = revert_to.try_into()?;
        let result = GetInputFocusReply { revert_to, sequence, length, focus };
        let _ = remaining;
        let remaining = initial_value.get(32 + length as usize * 4..)
            .ok_or(ParseError::InsufficientData)?;
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetInputFocusReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
```
There is also a function for sending the request in the extension trait:
```rust
    fn get_input_focus(&self) -> Result<Cookie<'_, Self, GetInputFocusReply>, ConnectionError>
    {
        get_input_focus(self)
    }
```

### Requests with a switch

Some requests have optional fields. A bit in the request then indicates the
presence of this extra field. For this kind of requests, a helper structure is
generated.
```xml
<request name="ConfigureWindow" opcode="12">
  <pad bytes="1" />
  <field type="WINDOW" name="window" />
  <field type="CARD16" name="value_mask" mask="ConfigWindow" />
  <pad bytes="2" />
  <switch name="value_list">
      <fieldref>value_mask</fieldref>
      <bitcase>
        <enumref ref="ConfigWindow">X</enumref>
        <field type="INT32" name="x" />
      </bitcase>
      <bitcase>
        <enumref ref="ConfigWindow">Y</enumref>
        <field type="INT32" name="y" />
      </bitcase>
      <bitcase>
        <enumref ref="ConfigWindow">Width</enumref>
        <field type="CARD32" name="width" />
      </bitcase>
      <bitcase>
        <enumref ref="ConfigWindow">Height</enumref>
        <field type="CARD32" name="height" />
      </bitcase>
      <bitcase>
        <enumref ref="ConfigWindow">BorderWidth</enumref>
        <field type="CARD32" name="border_width" />
      </bitcase>
      <bitcase>
        <enumref ref="ConfigWindow">Sibling</enumref>
        <field type="WINDOW" name="sibling" altenum="Window"/>
      </bitcase>
      <bitcase>
        <enumref ref="ConfigWindow">StackMode</enumref>
        <field type="CARD32" name="stack_mode" enum="StackMode"/>
      </bitcase>
  </switch>
  <doc>[SNIP]</doc>
</request>
```
The switch is represented via a helper struct:
```rust
/// Auxiliary and optional information for the `configure_window` function
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConfigureWindowAux {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub border_width: Option<u32>,
    pub sibling: Option<Window>,
    pub stack_mode: Option<StackMode>,
}
impl ConfigureWindowAux {
    fn try_parse(value: &[u8], value_mask: u16) -> Result<(Self, &[u8]), ParseError> {
        let switch_expr = u32::from(value_mask);
        let mut outer_remaining = value;
        let x = if switch_expr & u32::from(ConfigWindow::X) != 0 {
            let remaining = outer_remaining;
            let (x, remaining) = i32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(x)
        } else {
            None
        };
        let y = if switch_expr & u32::from(ConfigWindow::Y) != 0 {
            let remaining = outer_remaining;
            let (y, remaining) = i32::try_parse(remaining)?;
            outer_remaining = remaining;
            Some(y)
        } else {
            None
        };
        [SNIP - you get the idea]
        let result = ConfigureWindowAux { x, y, width, height, border_width, sibling, stack_mode };
        Ok((result, outer_remaining))
    }
}
impl ConfigureWindowAux {
    #[allow(dead_code)]
    fn serialize(&self, value_mask: u16) -> Vec<u8> {
        let mut result = Vec::new();
        self.serialize_into(&mut result, value_mask);
        result
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>, value_mask: u16) {
        assert_eq!(self.switch_expr(), u32::from(value_mask), "switch `value_list` has an inconsistent discriminant");
        if let Some(x) = self.x {
            x.serialize_into(bytes);
        }
        if let Some(y) = self.y {
            y.serialize_into(bytes);
        }
        [SNIP - you get the idea]
    }
}
impl ConfigureWindowAux {
    fn switch_expr(&self) -> u32 {
        let mut expr_value = 0;
        if self.x.is_some() {
            expr_value |= u32::from(ConfigWindow::X);
        }
        if self.y.is_some() {
            expr_value |= u32::from(ConfigWindow::Y);
        }
        [SNIP - you get the idea]
        expr_value
    }
}
impl ConfigureWindowAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    /// Set the `x` field of this structure.
    pub fn x<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.x = value.into();
        self
    }
    /// Set the `y` field of this structure.
    pub fn y<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.y = value.into();
        self
    }
    [SNIP - you get the idea]
}
```
This code is generated for the actual request:
```rust
/// Opcode for the ConfigureWindow request
pub const CONFIGURE_WINDOW_REQUEST: u8 = 12;
/// [SNIP]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigureWindowRequest<'input> {
    pub window: Window,
    pub value_list: Cow<'input, ConfigureWindowAux>,
}
impl<'input> ConfigureWindowRequest<'input> {
    /// Serialize this request into bytes for the provided connection
    fn serialize<Conn>(self, conn: &Conn) -> Result<BufWithFds<PiecewiseBuf<'input>>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let _ = conn;
        let length_so_far = 0;
        let window_bytes = self.window.serialize();
        let value_mask = u16::try_from(self.value_list.switch_expr()).unwrap();
        let value_mask_bytes = value_mask.serialize();
        let mut request0 = vec![
            CONFIGURE_WINDOW_REQUEST,
            0,
            0,
            0,
            window_bytes[0],
            window_bytes[1],
            window_bytes[2],
            window_bytes[3],
            value_mask_bytes[0],
            value_mask_bytes[1],
            0,
            0,
        ];
        let length_so_far = length_so_far + request0.len();
        let value_list_bytes = self.value_list.serialize(value_mask);
        let length_so_far = length_so_far + value_list_bytes.len();
        let padding0 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
        let length_so_far = length_so_far + padding0.len();
        assert_eq!(length_so_far % 4, 0);
        let length = u16::try_from(length_so_far / 4).unwrap_or(0);
        request0[2..4].copy_from_slice(&length.to_ne_bytes());
        Ok((vec![request0.into(), value_list_bytes.into(), padding0.into()], vec![]))
    }
    pub fn send<Conn>(self, conn: &Conn) -> Result<VoidCookie<'_, Conn>, ConnectionError>
    where
        Conn: RequestConnection + ?Sized,
    {
        let (bytes, fds) = self.serialize(conn)?;
        let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
        Ok(conn.send_request_without_reply(&slices, fds)?)
    }
    /// Parse this request given its header, its body, and any fds that go along with it
    pub fn try_parse_request(header: RequestHeader, value: &'input [u8]) -> Result<Self, ParseError> {
        if header.major_opcode != CONFIGURE_WINDOW_REQUEST {
            return Err(ParseError::InvalidValue);
        }
        let remaining = &[header.minor_opcode];
        let remaining = remaining.get(1..).ok_or(ParseError::InsufficientData)?;
        let _ = remaining;
        let (window, remaining) = Window::try_parse(value)?;
        let (value_mask, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::InsufficientData)?;
        let (value_list, remaining) = ConfigureWindowAux::try_parse(remaining, value_mask)?;
        let _ = remaining;
        Ok(ConfigureWindowRequest {
            window,
            value_list: Cow::Owned(value_list),
        })
    }
    /// Clone all borrowed data in this ConfigureWindowRequest.
    pub fn into_owned(self) -> ConfigureWindowRequest<'static> {
        ConfigureWindowRequest {
            window: self.window,
            value_list: Cow::Owned(self.value_list.into_owned()),
        }
    }
}
impl<'input> Request for ConfigureWindowRequest<'input> {
    type Reply = ();
}
/// [SNIP]
pub fn configure_window<'c, 'input, Conn>(conn: &'c Conn, window: Window, value_list: &'input ConfigureWindowAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ConfigureWindowRequest {
        window,
        value_list: Cow::Borrowed(value_list),
    };
    request0.send(conn)
}
```
And this code is in the extension trait:
```rust
    /// [SNIP]
    fn configure_window<'c, 'input>(&'c self, window: Window, value_list: &'input ConfigureWindowAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        configure_window(self, window, value_list)
    }
```

## Common code

The above showed examples for the code that is generated in a single module.
There is also some common code in [`x11rb::protocol`](../src/protocol/mod.rs).
This contains `enum`s over all possible requests, replies, errors, and events.
Via these, you can e.g. get the `sequence_number` contained in an event without
having to write a big `match` over all possible events.
