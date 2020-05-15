# Examples of the generated code

This crate uses some python code to generate rust code for the X11 protocol. You
might be curious what the generated code looks like. This document is there to
answer this question.

As you may know, the code generator uses an XML description of the X11 protocol
from `xcb-proto`. This document will show some examples of the XML description
followed by the Rust code that is generated for it.

The following code is generated at the beginning of a module:

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L1-L21

## XID types

XIDs simply are numbers. They uniquely identify, for example, a window.

```xml
<xidtype name="WINDOW" />
```

Since all XIDs are 32 bit numbers, the generated code is a type alias:

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L59

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

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L91-L127

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

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L444-L482

## Enumerations

### 'Real' enumerations

```xml
<enum name="BackingStore">
  <item name="NotUseful"> <value>0</value></item>
  <item name="WhenMapped"><value>1</value></item>
  <item name="Always">    <value>2</value></item>
</enum>
```
Depending on the largest value, appropriate `From` implementations are
generated.

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L587-L650

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

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L6794-L6871

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

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L4586-L4739

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

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L1262-L1385

## Errors

```xml
<error name="Request" number="1">
  <field type="CARD32" name="bad_value" />
  <field type="CARD16" name="minor_opcode" />
  <field type="CARD8" name="major_opcode" />
  <pad bytes="1" />
</error>
```

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L5025-L5104

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

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L17666-L17684

And this code is in the extension trait:

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L19762-L19765

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

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L10420-L10438

And this code is in the extension trait:

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L18959-L18962

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

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L6944-L7055

This code is generated for the actual request:

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L7057-L7138

And this code is in the extension trait:

https://github.com/psychon/x11rb/blob/e9dab004d422297dea2262905e28baca27730efc/src/protocol/xproto.rs#L18001-L18004
