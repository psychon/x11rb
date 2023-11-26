use std::borrow::Cow;
use std::sync::OnceLock;

use regex::Regex;

use super::Sections;

/// Shorten multi-line doc comments by replacing them with `[SNIP]`
fn shorten_docs(input: &str) -> Cow<'_, str> {
    static REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = REGEX.get_or_init(|| Regex::new("///.*(\n *///.*)+").unwrap());
    regex.replace_all(input, "/// [SNIP]")
}

/// Special helper for shortening the very long code for `CreateWindowAux`
fn shorten_create_window_aux(input: &str) -> String {
    fn regex<'a>(input: &'a str, regex: &str, replacement: &str) -> Cow<'a, str> {
        Regex::new(regex).unwrap().replace_all(input, replacement)
    }
    let result = regex(
        input,
        "pub border_pixmap:[^}]*}",
        "[SNIP - you get the idea]\n}",
    );
    let result = regex(
        &result,
        "(?s)let border_pixmap = if.*let result = CreateWindowAux",
        "[SNIP - you get the idea]\n        let result = CreateWindowAux",
    );
    let result = regex(
        &result,
        "(?s)if let Some\\(border_pixmap\\).*?\n    }\n",
        "[SNIP - you get the idea]\n    }\n",
    );
    let result = regex(
        &result,
        "(?s)if self.border_pixmap.is_some.*?expr_value\n",
        "[SNIP - you get the idea]\n        expr_value\n",
    );
    let result = regex(
        &result,
        "(?s)/// Set the `border_pixmap` field.*?\n}$",
        "[SNIP - you get the idea]\n}",
    );
    result.into_owned()
}

/// Generate the contents of the output file of this pogram
pub fn generate(proto_xproto: &Sections, x11rb_xproto: &Sections) -> String {
    let ext_trait = Sections::new_from_trait(x11rb_xproto.get_by_needle("pub trait ConnectionExt"));
    let mut result = DOCS.to_string();

    // List of replacements where a given String is inserted directly
    let other_replacements = [
        ("{protocol_header}", proto_xproto.get_range_by_index(..4)),
        (
            "{protocol_create_window_aux}",
            shorten_create_window_aux(proto_xproto.get_by_needle("pub struct CreateWindowAux")),
        ),
        (
            "{protocol_depth}",
            proto_xproto.get_by_needle("pub struct Depth").to_string(),
        ),
    ];
    for (keyword, replacement) in other_replacements {
        result = result.replace(keyword, &replacement);
    }
    for (keyword, sections, needle) in SECTION_REPLACEMENTS {
        let sections = match sections {
            SectionKind::Proto => proto_xproto,
            SectionKind::X11rb => x11rb_xproto,
            SectionKind::ExtTrait => &ext_trait,
        };
        result = result.replace(keyword, &shorten_docs(sections.get_by_needle(needle)));
    }
    result
}

enum SectionKind {
    /// The sections of x11rb-protocol's xproto.rs
    Proto,
    /// The sections of x11rb's xproto.rs
    X11rb,
    /// The sections of the ConnectionExt trait of x11rb's xproto.rs
    ExtTrait,
}

// List of replacements where something is looked up in a Sections instance
const SECTION_REPLACEMENTS: &[(&str, SectionKind, &str)] = &[
    ("{protocol_window}", SectionKind::Proto, "pub type Window"),
    ("{protocol_point}", SectionKind::Proto, "pub struct Point"),
    (
        "{protocol_backing_store}",
        SectionKind::Proto,
        "pub struct BackingStore",
    ),
    (
        "{protocol_config_window}",
        SectionKind::Proto,
        "pub struct ConfigWindow",
    ),
    (
        "{protocol_client_message_data}",
        SectionKind::Proto,
        "pub struct ClientMessageData",
    ),
    (
        "{protocol_key_press_event}",
        SectionKind::Proto,
        "pub struct KeyPressEvent",
    ),
    (
        "{protocol_request_error}",
        SectionKind::Proto,
        "pub const REQUEST_ERROR",
    ),
    (
        "{protocol_no_operation}",
        SectionKind::Proto,
        "pub struct NoOperationRequest",
    ),
    (
        "{x11rb_no_operation}",
        SectionKind::X11rb,
        "pub fn no_operation",
    ),
    (
        "{x11rb_no_operation_trait}",
        SectionKind::ExtTrait,
        "fn no_operation",
    ),
    (
        "{protocol_get_input_focus}",
        SectionKind::Proto,
        "pub struct GetInputFocusRequest",
    ),
    (
        "{protocol_get_input_focus_reply}",
        SectionKind::Proto,
        "pub struct GetInputFocusReply",
    ),
    (
        "{x11rb_get_input_focus}",
        SectionKind::X11rb,
        "pub fn get_input_focus",
    ),
    (
        "{x11rb_get_input_focus_trait}",
        SectionKind::ExtTrait,
        "fn get_input_focus",
    ),
    (
        "{protocol_create_window_request}",
        SectionKind::Proto,
        "pub struct CreateWindowRequest",
    ),
    (
        "{x11rb_create_window_request}",
        SectionKind::X11rb,
        "pub fn create_window",
    ),
    (
        "{x11rb_create_window_request_trait}",
        SectionKind::ExtTrait,
        "fn create_window",
    ),
];

static DOCS: &str = r#"# Examples of the generated code

This crate uses a code generator to generate Rust code for the X11 protocol. You
might be curious what the generated code looks like. This document is there to
answer this question.

There are two crates involved:

- x11rb-protocol will contain most of the resulting code and all examples below
  are for this crate, unless explicitly mentioned
- x11rb contains some helper functions to simplify request sending

The code in x11rb-async is similar to what is generated for x11rb. Hence, it is
not shown here.

As you may know, the code generator uses an XML description of the X11 protocol
from `xcb-proto`. This document will show some examples of the XML description
followed by the Rust code that is generated for it.

The following code is generated at the beginning of a module (example for
`xproto` in x11rb-protocol; other modules and x11rb have some slight differences):
```rust
{protocol_header}
```

## XID types

XIDs simply are numbers. They uniquely identify, for example, a window.

```xml
<xidtype name="WINDOW" />
```

Since all XIDs are 32 bit numbers, the generated code is a type alias:

```rust
{protocol_window}
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
example to parse a list of `Point`s.

We must also be able to send structs to the server. This is handled through the
`Serialize` trait that produces data in the native endian.

```rust
{protocol_point}
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
{protocol_depth}
```

## Enumerations

Enumerations have a set of defined values, similar to rust `enum`s. However,
they are not represented as `enum`s since there are cases where the X11 server
violates the X11 protocol. These cases previously caused `ParseError`. Thus,
enumerations are now represented as a newtype around numbers.

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
{protocol_backing_store}
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
{protocol_config_window}
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
{protocol_client_message_data}
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
{protocol_key_press_event}
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
{protocol_request_error}
```
The actual representation of an X11 error can be found in
[`x11rb::x11_utils::X11Error`](../x11rb-protocol/src/x11_utils.rs).

## Requests

For requests, we generate an extension trait in x11rb. Individual requests are
available on this trait and as global functions. The generic structure looks
like this:
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
The request is represented by a structure that contains all of the request's
fields. This `struct` can be constructed explicitly and then
`conn.send_trait_request_without_reply()` to an X11 server.

This code is generated in the module in `x11rb-protocol`:
```rust
{protocol_no_operation}
```
In the `x11rb`-crate, a function for sending the request via a function call is
generated.
```rust
{x11rb_no_operation}
```
The request sending function is also available on the extension trait:
```rust
{x11rb_no_operation_trait}
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
There is again a structure generated in `x11rb-protocol` that represents the request:
```rust
{protocol_get_input_focus}
```
The reply is handled similar to a `struct`:
```rust
{protocol_get_input_focus_reply}
```
In `x11rb`, there is a function to send the request:
```rust
{x11rb_get_input_focus}
```
There is also a function for sending the request in the extension trait:
```rust
{x11rb_get_input_focus_trait}
```

### Requests with a switch

Some requests have optional fields. A bit in the request then indicates the
presence of this extra field. For this kind of requests, a helper structure is
generated.
```xml
<request name="CreateWindow" opcode="1">
  <field type="CARD8" name="depth" />
  <field type="WINDOW" name="wid" />
  <field type="WINDOW" name="parent" />
  <field type="INT16" name="x" />
  <field type="INT16" name="y" />
  <field type="CARD16" name="width" />
  <field type="CARD16" name="height" />
  <field type="CARD16" name="border_width" />
  <field type="CARD16" name="class" enum="WindowClass" />
  <field type="VISUALID" name="visual" />
  <field type="CARD32" name="value_mask" mask="CW" />
  <switch name="value_list">
      <fieldref>value_mask</fieldref>
      <bitcase>
        <enumref ref="CW">BackPixmap</enumref>
        <field type="PIXMAP" name="background_pixmap" altenum="BackPixmap"/>
      </bitcase>
      <bitcase>
        <enumref ref="CW">BackPixel</enumref>
        <field type="CARD32" name="background_pixel" />
      </bitcase>
      [SNIP - you get the idea]
  </switch>
  <doc>[SNIP]</doc>
</request>
```
The switch is represented via a helper struct:
```rust
{protocol_create_window_aux}
```
This code is generated for the actual request:
```rust
{protocol_create_window_request}
```
The code in `x11rb` looks like this:
```rust
{x11rb_create_window_request}
```
And this code is in the extension trait:
```rust
{x11rb_create_window_request_trait}
```

## Common code

The above showed examples for the code that is generated in a single module.
There is also some common code in
[`x11rb_protocol::protocol`](../x11rb-protocol/src/protocol/mod.rs).  This
contains `enum`s over all possible requests, replies, errors, and events.  Via
these, you can e.g. get the `sequence_number` contained in an event without
having to write a big `match` over all possible events.
"#;
