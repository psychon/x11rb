# Examples of the generated code

This crate uses some python code to generate rust code for the X11 protocol. You
might be curious what the generated code looks like. This document is there to
answer this question.

As you may know, the code generator uses an XML description of the X11 protocol
from `xcb-proto`. This document will show some examples of the XML description
followed by the Rust code that is generated for it.

The following code is generated at the beginning of a module:
```rust
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
use crate::utils::Buffer;
#[allow(unused_imports)]
use crate::x11_utils::{GenericEvent, GenericError as X11GenericError};
use crate::x11_utils::TryParse;
#[allow(unused_imports)]
use crate::connection::SequenceNumber;
use crate::connection::{Cookie, Connection};
use crate::connection::ListFontsWithInfoCookie;
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone)]
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
        let mut visuals = Vec::with_capacity(visuals_len.try_into()?);
        for _ in 0..visuals_len {
            let (v, new_remaining) = Visualtype::try_parse(remaining)?;
            visuals.push(v);
            remaining = new_remaining;
        }
        let result = Depth { depth, visuals };
        Ok((result, remaining))
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
#[derive(Debug, Clone, Copy)]
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
impl Into<Option<u8>> for BackingStore {
    fn into(self) -> Option<u8> {
        Some(self.into())
    }
}
impl Into<u16> for BackingStore {
    fn into(self) -> u16 {
        Into::<u8>::into(self) as _
    }
}
impl Into<Option<u16>> for BackingStore {
    fn into(self) -> Option<u16> {
        Some(self.into())
    }
}
impl Into<u32> for BackingStore {
    fn into(self) -> u32 {
        Into::<u8>::into(self) as _
    }
}
impl Into<Option<u32>> for BackingStore {
    fn into(self) -> Option<u32> {
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
#[derive(Debug, Clone, Copy)]
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
impl Into<Option<u8>> for ConfigWindow {
    fn into(self) -> Option<u8> {
        Some(self.into())
    }
}
impl Into<u16> for ConfigWindow {
    fn into(self) -> u16 {
        Into::<u8>::into(self) as _
    }
}
impl Into<Option<u16>> for ConfigWindow {
    fn into(self) -> Option<u16> {
        Some(self.into())
    }
}
impl Into<u32> for ConfigWindow {
    fn into(self) -> u32 {
        Into::<u8>::into(self) as _
    }
}
impl Into<Option<u32>> for ConfigWindow {
    fn into(self) -> Option<u32> {
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
}
impl TryParse for ClientMessageData {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let inner = value[..20].iter().copied().collect();
        let result = ClientMessageData(inner);
        Ok((result, &value[20..]))
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
pub const KEY_PRESS_EVENT: u8 = 2;
#[derive(Debug, Clone, Copy)]
pub struct KeyPressEvent {
    pub detail: u8,
    pub time: u32,
    pub root: u32,
    pub event: u32,
    pub child: u32,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub state: u16,
    pub same_screen: u8,
}
impl TryParse for KeyPressEvent {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (detail, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (time, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (root, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (event, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (child, new_remaining) = u32::try_parse(remaining)?;
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
        let (same_screen, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = KeyPressEvent { detail, time, root, event, child, root_x, root_y, event_x, event_y, state, same_screen };
        Ok((result, remaining))
    }
}
impl TryFrom<Buffer> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}
impl TryFrom<GenericEvent> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: GenericEvent) -> Result<Self, Self::Error> {
        Self::try_from(Into::<Buffer>::into(value))
    }
}
impl TryFrom<&[u8]> for KeyPressEvent {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
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
pub const REQUEST_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy)]
pub struct RequestError {
    pub bad_value: u32,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}
impl TryParse for RequestError {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        remaining = &remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (bad_value, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let (minor_opcode, new_remaining) = u16::try_parse(remaining)?;
        remaining = new_remaining;
        let (major_opcode, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let result = RequestError { bad_value, minor_opcode, major_opcode };
        Ok((result, remaining))
    }
}
impl TryFrom<Buffer> for RequestError {
    type Error = ParseError;
    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        Self::try_from(&*value)
    }
}
impl TryFrom<X11GenericError> for RequestError {
    type Error = ParseError;
    fn try_from(value: X11GenericError) -> Result<Self, Self::Error> {
        Self::try_from(Into::<Buffer>::into(value))
    }
}
impl TryFrom<&[u8]> for RequestError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
```

## Requests

### Request without a reply

```xml
<request name="NoOperation" opcode="127" />
```
```rust
pub const NO_OPERATION_REQUEST: u8 = 127;
pub fn no_operation<A: Connection>(c: &A) -> Result<SequenceNumber, ConnectionError>
{
    let length: usize = (4 + 3) / 4;
    let length_bytes = length.to_ne_bytes();
    let request0 = [
        NO_OPERATION_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    assert_eq!((*&request0).len(), (4 + 3) / 4 * 4);
    Ok(c.send_request_without_reply(&[IoSlice::new(&request0)]))
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
```rust
pub const GET_INPUT_FOCUS_REQUEST: u8 = 43;
pub fn get_input_focus<A: Connection>(c: &A) -> Result<Cookie<A, GetInputFocusReply>, ConnectionError>
{
    let length: usize = (4 + 3) / 4;
    let length_bytes = length.to_ne_bytes();
    let request0 = [
        GET_INPUT_FOCUS_REQUEST,
        0,
        length_bytes[0],
        length_bytes[1],
    ];
    assert_eq!((*&request0).len(), (4 + 3) / 4 * 4);
    Ok(c.send_request_with_reply(&[IoSlice::new(&request0)]))
}
#[derive(Debug, Clone, Copy)]
pub struct GetInputFocusReply {
    pub revert_to: u8,
    pub focus: u32,
}
impl TryParse for GetInputFocusReply {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let mut remaining = value;
        remaining = &remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (revert_to, new_remaining) = u8::try_parse(remaining)?;
        remaining = new_remaining;
        remaining = &remaining.get(2..).ok_or(ParseError::ParseError)?;
        remaining = &remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (focus, new_remaining) = u32::try_parse(remaining)?;
        remaining = new_remaining;
        let result = GetInputFocusReply { revert_to, focus };
        Ok((result, remaining))
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
```rust
pub const CONFIGURE_WINDOW_REQUEST: u8 = 12;
#[derive(Debug, Clone, Copy, Default)]
pub struct ConfigureWindowAux {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub border_width: Option<u32>,
    pub sibling: Option<u32>,
    pub stack_mode: Option<u32>,
}
impl ConfigureWindowAux {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn to_ne_bytes(&self) -> Vec<u8> {
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
    pub fn value_mask(&self) -> u16 {
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
    pub fn x<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.x = value.into();
        self
    }
    pub fn y<I>(mut self, value: I) -> Self where I: Into<Option<i32>> {
        self.y = value.into();
        self
    }
    pub fn width<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.width = value.into();
        self
    }
    pub fn height<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.height = value.into();
        self
    }
    pub fn border_width<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.border_width = value.into();
        self
    }
    pub fn sibling<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.sibling = value.into();
        self
    }
    pub fn stack_mode<I>(mut self, value: I) -> Self where I: Into<Option<u32>> {
        self.stack_mode = value.into();
        self
    }
}
pub fn configure_window<A: Connection>(c: &A, window: u32, value_list: &ConfigureWindowAux) -> Result<SequenceNumber, ConnectionError>
{
    let value_mask = value_list.value_mask();
    let length: usize = (12 + (4 * value_mask.count_ones()) as usize + 3) / 4;
    let length_bytes = length.to_ne_bytes();
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
    assert_eq!((*&request0).len() + (*&value_list_bytes).len(), (12 + (4 * value_mask.count_ones()) as usize + 3) / 4 * 4);
    Ok(c.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&value_list_bytes)]))
}
```
