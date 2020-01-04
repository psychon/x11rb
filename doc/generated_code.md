# Examples of the generated code

This crate uses some python code to generate rust code for the X11 protocol. You
might be curious what the generated code looks like. This document is there to
answer this question.

As you may know, the code generator uses an XML description of the X11 protocol
from `xcb-proto`. This document will show some examples of the XML description
followed by the Rust code that is generated for it.

The following code is generated at the beginning of a module:
```rust
#![allow(clippy::unreadable_literal)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use std::option::Option as RustOption;
#[allow(unused_imports)]
use crate::utils::{Buffer, RawFdContainer};
#[allow(unused_imports)]
use crate::x11_utils::{GenericEvent as X11GenericEvent, GenericError as X11GenericError, Event as _};
use crate::x11_utils::TryParse;
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::cookie::ListFontsWithInfoCookie;
use crate::errors::{ParseError, ConnectionError};
```

## XID types

XIDs simply are numbers. They uniquely identify, for example, a window.

```xml
<xidtype name="WINDOW" />
```

Since all XIDs are 32 bit numbers, the generated code is a type alias:

```rust
pub type WINDOW = u32;
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

We must also be able to send structs to the server. This is handled by the
`to_ne_bytes()` method. 'ne' stands for 'native endian'.
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
impl TryParse for Point {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (x, new_remaining) = i16::try_parse(remaining)?;
        remaining = new_remaining;
        let (y, new_remaining) = i16::try_parse(remaining)?;
        remaining = new_remaining;
        let result = Point { x, y };
        Ok((result, remaining))
    }
}
impl TryFrom<&Buffer> for Point {
    type Error = ParseError;
    fn try_from(value: &Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}
impl TryFrom<Buffer> for Point {
    type Error = ParseError;
    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}
impl TryFrom<&[u8]> for Point {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Point {
    pub fn to_ne_bytes(&self) -> [u8; 4] {
        let x_bytes = self.x.to_ne_bytes();
        let y_bytes = self.y.to_ne_bytes();
        [
            x_bytes[0],
            x_bytes[1],
            y_bytes[0],
            y_bytes[1],
        ]
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
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Depth {
    pub depth: u8,
    pub visuals: Vec<Visualtype>,
}
impl TryParse for Depth {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (depth, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (visuals_len, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(4..).ok_or(ParseError::ParseError)?;
        let list_length = visuals_len as usize;
        let mut visuals = Vec::with_capacity(list_length);
        for _ in 0..list_length {
            let (v, new_remaining) = Visualtype::try_parse(remaining)?;
            visuals.push(v);
            remaining = new_remaining;
        }
        let result = Depth { depth, visuals };
        Ok((result, remaining))
    }
}
impl TryFrom<&Buffer> for Depth {
    type Error = ParseError;
    fn try_from(value: &Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}
impl TryFrom<Buffer> for Depth {
    type Error = ParseError;
    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}
impl TryFrom<&[u8]> for Depth {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Depth {
    pub fn to_ne_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let depth_bytes = self.depth.to_ne_bytes();
        let visuals_len = self.visuals.len() as u16;
        let visuals_len_bytes = visuals_len.to_ne_bytes();
        result.extend([
            depth_bytes[0],
            0,
            visuals_len_bytes[0],
            visuals_len_bytes[1],
            0,
            0,
            0,
            0,
        ].iter());
        for obj in self.visuals.iter() {
            result.extend(obj.to_ne_bytes().iter());
        }
        result
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
Depending on the largest value, appropriate `Into` implementations are
generated.
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackingStore {
    NotUseful,
    WhenMapped,
    Always,
}
impl Into<u8> for BackingStore {
    fn into(self) -> u8 {
        match self {
            BackingStore::NotUseful => 0,
            BackingStore::WhenMapped => 1,
            BackingStore::Always => 2,
        }
    }
}
impl Into<RustOption<u8>> for BackingStore {
    fn into(self) -> RustOption<u8> {
        Some(self.into())
    }
}
impl Into<u16> for BackingStore {
    fn into(self) -> u16 {
        Into::<u8>::into(self).into()
    }
}
impl Into<RustOption<u16>> for BackingStore {
    fn into(self) -> RustOption<u16> {
        Some(self.into())
    }
}
impl Into<u32> for BackingStore {
    fn into(self) -> u32 {
        Into::<u8>::into(self).into()
    }
}
impl Into<RustOption<u32>> for BackingStore {
    fn into(self) -> RustOption<u32> {
        Some(self.into())
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
pub enum ConfigWindow {
    X,
    Y,
    Width,
    Height,
    BorderWidth,
    Sibling,
    StackMode,
}
impl Into<u8> for ConfigWindow {
    fn into(self) -> u8 {
        match self {
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
impl Into<RustOption<u8>> for ConfigWindow {
    fn into(self) -> RustOption<u8> {
        Some(self.into())
    }
}
impl Into<u16> for ConfigWindow {
    fn into(self) -> u16 {
        Into::<u8>::into(self).into()
    }
}
impl Into<RustOption<u16>> for ConfigWindow {
    fn into(self) -> RustOption<u16> {
        Some(self.into())
    }
}
impl Into<u32> for ConfigWindow {
    fn into(self) -> u32 {
        Into::<u8>::into(self).into()
    }
}
impl Into<RustOption<u32>> for ConfigWindow {
    fn into(self) -> RustOption<u32> {
        Some(self.into())
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
```rust
#[derive(Debug, Clone)]
pub struct ClientMessageData(Vec<u8>);
impl ClientMessageData {
    pub fn as_data8(&self) -> [u8; 20] {
        fn do_the_parse(value: &[u8]) -> Result<[u8; 20], ParseError> {
            let mut remaining = value;
            let (data8_0, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_1, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_2, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_3, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_4, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_5, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_6, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_7, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_8, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_9, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_10, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_11, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_12, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_13, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_14, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_15, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_16, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_17, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_18, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let (data8_19, new_remaining) = u8::try_parse(remaining)?;
            remaining = new_remaining;
            let data8 = [
                data8_0,
                data8_1,
                data8_2,
                data8_3,
                data8_4,
                data8_5,
                data8_6,
                data8_7,
                data8_8,
                data8_9,
                data8_10,
                data8_11,
                data8_12,
                data8_13,
                data8_14,
                data8_15,
                data8_16,
                data8_17,
                data8_18,
                data8_19,
            ];
            let _ = remaining;
            Ok(data8)
        }
        do_the_parse(&self.0[..]).unwrap()
    }
    pub fn as_data16(&self) -> [u16; 10] {
        fn do_the_parse(value: &[u8]) -> Result<[u16; 10], ParseError> {
            let mut remaining = value;
            let (data16_0, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_1, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_2, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_3, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_4, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_5, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_6, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_7, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_8, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
            let (data16_9, new_remaining) = u16::try_parse(remaining)?;
            remaining = new_remaining;
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
        do_the_parse(&self.0[..]).unwrap()
    }
    pub fn as_data32(&self) -> [u32; 5] {
        fn do_the_parse(value: &[u8]) -> Result<[u32; 5], ParseError> {
            let mut remaining = value;
            let (data32_0, new_remaining) = u32::try_parse(remaining)?;
            remaining = new_remaining;
            let (data32_1, new_remaining) = u32::try_parse(remaining)?;
            remaining = new_remaining;
            let (data32_2, new_remaining) = u32::try_parse(remaining)?;
            remaining = new_remaining;
            let (data32_3, new_remaining) = u32::try_parse(remaining)?;
            remaining = new_remaining;
            let (data32_4, new_remaining) = u32::try_parse(remaining)?;
            remaining = new_remaining;
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
        do_the_parse(&self.0[..]).unwrap()
    }
    fn to_ne_bytes(&self) -> &[u8] {
        &self.0
    }
}
impl TryParse for ClientMessageData {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner = value.get(..20)
            .ok_or(ParseError::ParseError)?
            .iter().copied().collect();
        let result = ClientMessageData(inner);
        Ok((result, &value[20..]))
    }
}
impl From<[u8; 20]> for ClientMessageData {
    fn from(value: [u8; 20]) -> Self {
        Self(value.to_vec())
    }
}
impl From<[u16; 10]> for ClientMessageData {
    fn from(value: [u16; 10]) -> Self {
        let value0 = value[0].to_ne_bytes();
        let value1 = value[1].to_ne_bytes();
        let value2 = value[2].to_ne_bytes();
        let value3 = value[3].to_ne_bytes();
        let value4 = value[4].to_ne_bytes();
        let value5 = value[5].to_ne_bytes();
        let value6 = value[6].to_ne_bytes();
        let value7 = value[7].to_ne_bytes();
        let value8 = value[8].to_ne_bytes();
        let value9 = value[9].to_ne_bytes();
        let value = [
            value0[0],
            value0[1],
            value1[0],
            value1[1],
            value2[0],
            value2[1],
            value3[0],
            value3[1],
            value4[0],
            value4[1],
            value5[0],
            value5[1],
            value6[0],
            value6[1],
            value7[0],
            value7[1],
            value8[0],
            value8[1],
            value9[0],
            value9[1],
        ];
        Self(value.to_vec())
    }
}
impl From<[u32; 5]> for ClientMessageData {
    fn from(value: [u32; 5]) -> Self {
        let value0 = value[0].to_ne_bytes();
        let value1 = value[1].to_ne_bytes();
        let value2 = value[2].to_ne_bytes();
        let value3 = value[3].to_ne_bytes();
        let value4 = value[4].to_ne_bytes();
        let value = [
            value0[0],
            value0[1],
            value0[2],
            value0[3],
            value1[0],
            value1[1],
            value1[2],
            value1[3],
            value2[0],
            value2[1],
            value2[2],
            value2[3],
            value3[0],
            value3[1],
            value3[2],
            value3[3],
            value4[0],
            value4[1],
            value4[2],
            value4[3],
        ];
        Self(value.to_vec())
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
    pub detail: KEYCODE,
    pub sequence: u16,
    pub time: TIMESTAMP,
    pub root: WINDOW,
    pub event: WINDOW,
    pub child: WINDOW,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: bool,
}
impl KeyPressEvent {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (detail, new_remaining) = KEYCODE::try_parse(remaining)?;
        remaining = new_remaining;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (time, new_remaining) = TIMESTAMP::try_parse(remaining)?;
        remaining = new_remaining;
        let (root, new_remaining) = WINDOW::try_parse(remaining)?;
        remaining = new_remaining;
        let (event, new_remaining) = WINDOW::try_parse(remaining)?;
        remaining = new_remaining;
        let (child, new_remaining) = WINDOW::try_parse(remaining)?;
        remaining = new_remaining;
        let (root_x, new_remaining) = i16::try_parse(remaining)?;
        remaining = new_remaining;
        let (root_y, new_remaining) = i16::try_parse(remaining)?;
        remaining = new_remaining;
        let (event_x, new_remaining) = i16::try_parse(remaining)?;
        remaining = new_remaining;
        let (event_y, new_remaining) = i16::try_parse(remaining)?;
        remaining = new_remaining;
        let (state, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (same_screen, new_remaining) = bool::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = KeyPressEvent { response_type, detail, sequence, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen };
        Ok((result, remaining))
    }
}
impl TryFrom<&Buffer> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}
impl TryFrom<Buffer> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}
impl TryFrom<&[u8]> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<X11GenericEvent> for KeyPressEvent {
    fn from(value: X11GenericEvent) -> Self {
        Self::try_from(Into::<Buffer>::into(value)).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&X11GenericEvent> for KeyPressEvent {
    fn from(value: &X11GenericEvent) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl Into<[u8; 32]> for &KeyPressEvent {
    fn into(self) -> [u8; 32] {
        let sequence = self.sequence.to_ne_bytes();
        let time = self.time.to_ne_bytes();
        let root = self.root.to_ne_bytes();
        let event = self.event.to_ne_bytes();
        let child = self.child.to_ne_bytes();
        let root_x = self.root_x.to_ne_bytes();
        let root_y = self.root_y.to_ne_bytes();
        let event_x = self.event_x.to_ne_bytes();
        let event_y = self.event_y.to_ne_bytes();
        let state = self.state.to_ne_bytes();
        [
            self.response_type, self.detail, sequence[0], sequence[1], time[0], time[1], time[2], time[3],
            root[0], root[1], root[2], root[3], event[0], event[1], event[2], event[3],
            child[0], child[1], child[2], child[3], root_x[0], root_x[1], root_y[0], root_y[1],
            event_x[0], event_x[1], event_y[0], event_y[1], state[0], state[1], (self.same_screen as u8), 0
        ]
    }
}
impl Into<[u8; 32]> for KeyPressEvent {
    fn into(self) -> [u8; 32] {
        (&self).into()
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
```rust
/// Opcode for the Request error
pub const REQUEST_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl RequestError {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (error_code, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (bad_value, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (minor_opcode, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (major_opcode, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = RequestError { response_type, error_code, sequence, bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<&Buffer> for RequestError {
    type Error = ParseError;
    fn try_from(value: &Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}
impl TryFrom<Buffer> for RequestError {
    type Error = ParseError;
    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}
impl TryFrom<&[u8]> for RequestError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl From<X11GenericError> for RequestError {
    fn from(value: X11GenericError) -> Self {
        Self::try_from(Into::<Buffer>::into(value)).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&X11GenericError> for RequestError {
    fn from(value: &X11GenericError) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl Into<[u8; 32]> for &RequestError {
    fn into(self) -> [u8; 32] {
        let sequence = self.sequence.to_ne_bytes();
        let bad_value = self.bad_value.to_ne_bytes();
        let minor_opcode = self.minor_opcode.to_ne_bytes();
        [
            self.response_type, self.error_code, sequence[0], sequence[1], bad_value[0], bad_value[1], bad_value[2], bad_value[3],
            minor_opcode[0], minor_opcode[1], self.major_opcode, 0, /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl Into<[u8; 32]> for RequestError {
    fn into(self) -> [u8; 32] {
        (&self).into()
    }
}
```

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
```
This code is generated in the module:
```rust
/// Opcode for the NoOperation request
pub const NO_OPERATION_REQUEST: u8 = 127;
pub fn no_operation<Conn>(conn: &Conn) -> Result<VoidCookie<Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).to_ne_bytes();
    let request0 = [
        NO_OPERATION_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
```
And this code is in the extension trait:
```rust
fn no_operation(&self) -> Result<VoidCookie<Self>, ConnectionError>
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
This code is generated in the module:
```rust
/// Opcode for the GetInputFocus request
pub const GET_INPUT_FOCUS_REQUEST: u8 = 43;
pub fn get_input_focus<Conn>(conn: &Conn) -> Result<Cookie<Conn, GetInputFocusReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).to_ne_bytes();
    let request0 = [
        GET_INPUT_FOCUS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetInputFocusReply {
    pub response_type: u8,
    pub revert_to: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: WINDOW,
}
impl GetInputFocusReply {
    pub(crate) fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        let (response_type, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (revert_to, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        let (sequence, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (length, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (focus, new_remaining) = WINDOW::try_parse(remaining)?;
        remaining = new_remaining;
        let result = GetInputFocusReply { response_type, revert_to, sequence, length, focus };
        Ok((result, remaining))
    }
}
impl TryFrom<&Buffer> for GetInputFocusReply {
    type Error = ParseError;
    fn try_from(value: &Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}
impl TryFrom<Buffer> for GetInputFocusReply {
    type Error = ParseError;
    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}
impl TryFrom<&[u8]> for GetInputFocusReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
```
And this code is in the extension trait:
```rust
fn get_input_focus(&self) -> Result<Cookie<Self, GetInputFocusReply>, ConnectionError>
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
This code is generated in the module:
```rust
/// Opcode for the ConfigureWindow request
pub const CONFIGURE_WINDOW_REQUEST: u8 = 12;
/// Auxiliary and optional information for the configure_window function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ConfigureWindowAux {
    x: RustOption<i32>,
    y: RustOption<i32>,
    width: RustOption<u32>,
    height: RustOption<u32>,
    border_width: RustOption<u32>,
    sibling: RustOption<u32>,
    stack_mode: RustOption<u32>,
}
impl ConfigureWindowAux {
    /// Create a new instance with all fields unset / not present.
    pub fn new() -> Self {
        Default::default()
    }
    fn wire_length(&self) -> usize {
        let mut result = 0;
        if self.x.is_some() {
            result += 4;
        }
        if self.y.is_some() {
            result += 4;
        }
        if self.width.is_some() {
            result += 4;
        }
        if self.height.is_some() {
            result += 4;
        }
        if self.border_width.is_some() {
            result += 4;
        }
        if self.sibling.is_some() {
            result += 4;
        }
        if self.stack_mode.is_some() {
            result += 4;
        }
        result
    }
    fn to_ne_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        if let Some(value) = self.x {
            result.extend(value.to_ne_bytes().iter());
        }
        if let Some(value) = self.y {
            result.extend(value.to_ne_bytes().iter());
        }
        if let Some(value) = self.width {
            result.extend(value.to_ne_bytes().iter());
        }
        if let Some(value) = self.height {
            result.extend(value.to_ne_bytes().iter());
        }
        if let Some(value) = self.border_width {
            result.extend(value.to_ne_bytes().iter());
        }
        if let Some(value) = self.sibling {
            result.extend(value.to_ne_bytes().iter());
        }
        if let Some(value) = self.stack_mode {
            result.extend(value.to_ne_bytes().iter());
        }
        result
    }
    fn value_mask(&self) -> u16 {
        let mut mask = 0;
        if self.x.is_some() {
            mask |= Into::<u16>::into(ConfigWindow::X);
        }
        if self.y.is_some() {
            mask |= Into::<u16>::into(ConfigWindow::Y);
        }
        if self.width.is_some() {
            mask |= Into::<u16>::into(ConfigWindow::Width);
        }
        if self.height.is_some() {
            mask |= Into::<u16>::into(ConfigWindow::Height);
        }
        if self.border_width.is_some() {
            mask |= Into::<u16>::into(ConfigWindow::BorderWidth);
        }
        if self.sibling.is_some() {
            mask |= Into::<u16>::into(ConfigWindow::Sibling);
        }
        if self.stack_mode.is_some() {
            mask |= Into::<u16>::into(ConfigWindow::StackMode);
        }
        mask
    }
    /// Set the x field of this structure.
    pub fn x<I>(mut self, value: I) -> Self where I: Into<RustOption<i32>> {
        self.x = value.into();
        self
    }
    /// Set the y field of this structure.
    pub fn y<I>(mut self, value: I) -> Self where I: Into<RustOption<i32>> {
        self.y = value.into();
        self
    }
    /// Set the width field of this structure.
    pub fn width<I>(mut self, value: I) -> Self where I: Into<RustOption<u32>> {
        self.width = value.into();
        self
    }
    /// Set the height field of this structure.
    pub fn height<I>(mut self, value: I) -> Self where I: Into<RustOption<u32>> {
        self.height = value.into();
        self
    }
    /// Set the border_width field of this structure.
    pub fn border_width<I>(mut self, value: I) -> Self where I: Into<RustOption<u32>> {
        self.border_width = value.into();
        self
    }
    /// Set the sibling field of this structure.
    pub fn sibling<I>(mut self, value: I) -> Self where I: Into<RustOption<u32>> {
        self.sibling = value.into();
        self
    }
    /// Set the stack_mode field of this structure.
    pub fn stack_mode<I>(mut self, value: I) -> Self where I: Into<RustOption<u32>> {
        self.stack_mode = value.into();
        self
    }
}
/// [SNIP]
pub fn configure_window<'c, Conn>(conn: &'c Conn, window: WINDOW, value_list: &ConfigureWindowAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let value_mask = value_list.value_mask();
    let length: usize = (12 + value_list.wire_length() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).to_ne_bytes();
    let window_bytes = window.to_ne_bytes();
    let value_mask_bytes = value_mask.to_ne_bytes();
    let value_list_bytes = value_list.to_ne_bytes();
    let request0 = [
        CONFIGURE_WINDOW_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
        window_bytes[0],
        window_bytes[1],
        window_bytes[2],
        window_bytes[3],
        value_mask_bytes[0],
        value_mask_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&value_list_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes), IoSlice::new(&padding1)], Vec::new())?)
}
```
And this code is in the extension trait:
```rust
/// [SNIP]
fn configure_window<'c>(&'c self, window: WINDOW, value_list: &ConfigureWindowAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
{
    configure_window(self, window, value_list)
}
```
