# Version 0.13.2 (2025-08-29)

New features:
* Update cursor icon file search path to follow a change in libxcb-cursor which
  in turn was changed to follow libXcursor.
* Get cursor theme name from `$XCURSOR_THEME` if set.
* Added `XCBConnection::from_existing_connection` to allow wrapping anything
  that implements `AsRawXcbConection`. Without this function, unsafe code was
  required from the user.
* Added `raw-window-handle` feature which makes `XCBConnection` implement
  `HasRawDisplayHandle` and `WindowWrapper` implement `HasRawWindowHandle`.
* Use new `hints.mostly-unused` Cargo feature. This might improve compilation
  speed with nightly compilers.

Fixes:
* Fix missing indentation in doc comments in generated code.
* Fix a minor X11 resource leak in cursor code.
* Gate tests behind the features they need so that `cargo test
  --no-default-features` works. This is now also tested in CI.
* The `present` feature depends on `dri3` to actually work, but this was not
  specified in `Cargo.toml`.

Breaking changes:
* MSRV increased to Rust 1.64.

Minor changes:
* Various minor changes to fix new compiler and clippy warnings and improve
  readability.
* Use existing xcursor-rs crate for cursor and cursor theme file parsing instead
  of own code.
* Updated dependencies to rustix 1.0, gethostname 1.0. This required a MSRV
  increase to Rust 1.64.
* Pin various dependencies in CI to keep our MSRV check working.

# Version 0.13.1 (2024-05-01)

New features:
* Add an `xkbcommon-example` to showcase use of x11rb together with the
  xkbcommon crate.
* Update from xcb-proto 1.15.2 to 1.17.0. This brings support for PRESENT 1.4
  and DRI3 1.4 and fixes some typos in the documentation.

Fixes:
* Macro hygiene: `atom_manager!` macro now uses`::std` when referring to std
  items.

Breaking changes:
* Optimise `atom_manager!` implementation. This changes the contents of the
  generated `Cookie` struct, but there should be no reasons to access these
  directly.
* Remove some unreachable public API from x11rb-async's `StreamAdaptor`.

Minor changes:
* Fix new compiler warnings and enable more lints.
* Update dependencies.

# Version 0.13.0 (2023-12-09)

New features:
* A bitmask enum in the generates code has new methods `bits()` for conversion
  to integer and `remove()` for removing certain bits.
* Update our bundled xcb-proto version. This update brings new documentation and
  support for newer versions of the DPMS and Present extensions.
* Update `$DISPLAY` parsing to match new behaviour in libxcb 1.16.
* Some variant of `x11rb_protocol::parse_display` is now also available in
  `no_std` mode.
* Better error message if `$DISPLAY` parsing fails.
* Add `Image::into_owned()` to get an Image instance with `'static`.
* Change `Image::put()` to convert the image to the X11 server's native
  representation before uploading.
* Implement x11rb's `RequestConnection` for `&C`, `&mut C`, `Box<C>`, `Arc<C>`,
  `Rc<C>` and `Cow<'_, C>` where `C: RequestConnection`.

Fixes:
* Fix broken link to x11rb in documentation of x11rb-async.
* Strip leading whitespace from documentation comments in code generator.
* Fix the `dl-libxcb` feature on OpenBSD. There is no `libxcb.so.2` on this
  system and we can simply ask for `libxcb.so` to be loaded.
* x11rb-async always needed at least tracing 0.1.33, but incorrectly declared
  compatibility with all 0.1 versions.

Breaking changes:
* Indicate not present properties in x11rb's `WmClass`, `WmSizeHints`, and
  `WmHints` helpers by introducing an `Option` in their return value.
  Previously, missing properties were reported as a parsing error.
* Avoid a `Vec` in some places in x11rb-protocol by using arrays instead. This
  affects the return value of a request's `serialize()` function, but also some
  internal code.
* Remove unused `read_exact()` method in x11rb's `Stream` trait.
* Replace use of nix with rustix and implement io-safety. This e.g. means
  that some types no longer implement `PartialEq` due to rustix's behaviour. FD
  passing now uses `std::os::unix::io::OwnedFd`.
* MSRV was bumped from Rust 1.56 to 1.63 for rustix 0.38.
* Change some functions in `x11rb::rust_connection::stream::DefaultStream` to
  also return the address that a connection was established to.
* Change wrapper types like `WindowWrapper` not to require a reference to a
  connection, but to accept any connection type directly. Due to the new feature
  mentioned above, this allows to use these wrappers e.g. with `Arc<C>`.
* Types in x11rb-protocol now implement less commonly used traits like
  `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` only if the new
  `extra-traits` feature is enabled. The `Debug` impl only produces the name of
  the type and not its contents in this case. This change improves compile times
  of the crate.
* Parsing of requests with the `R::try_parse_request` function is now gated
  behind the new `request-parsing` feature. This change improves compile times.

Minor changes:
* Update dependencies.
* Enable all features when building for docs.rs.
* Fix some lints in examples.
* Change some log calls from info to debug.
* Drop gethostname dependencies on `unix` by using rustix instead.
* Various changes to please clippy.
* Improve docs front pages.

# Version 0.12.0 (2023-05-27)

New features:
* There is a new crate x11rb-async that brings x11rb to the async ecosystem.
* Bitmask enumerations now also implement BitAnd, BitAndAssign and BitOrAssign
  and offers contains() and intersects() methods.
* Add (optional) support for the `as-raw-xcb-connection` crate.
* Implement Default for `x11rb_protocol::connection::Connection`.
* Optional support for the `tracing` crate.
* New API to convert major + minor opcode of a request to human readable names.

Fixes:
* Improve error message when not all FDs could be sent.
* Use correct byte order on `Image::get()` for big-endian servers.
* Fix build for `XCBConnection` on architectures without `AtomicU64`.

Breaking changes:
* Various methods on `x11rb_protocol::protocol::xinput::EventForSend` now return
  `ParseError` instead of unwrapping errors internally.
* The generated names for some structs in `x11rb_protocol::protocol::xkb` are
  now better. For example, `SelectEventsAuxBitcase1` is now called
  `SelectEventsAuxNewKeyboardNotify`, which at least hints towards its meaning.
* `Image::get()` now also returns the visual ID from the `GetImageReply`.

Minor changes:
* Update dependencies.
* Get rid of some (infallible) `unwrap()`s in generated code by moving this to a
  hand-written helper function.
* The usual round of fixing new clippy warnings.
* Remove some unnecessary `unwrap()`s in examples.
* Use the polling crate in the xclock_utc example.

# Version 0.11.1 (2023-01-06)

Fixes:
* Fix connection breakage after sending 2^16 requests without a reply.
* Enable features on docs.rs to have better docs.
* Fix dl-libxcb feature on NetBSD not finding libxcb.so.

# Version 0.11.0 (2022-11-18)

New features:
* All extensions are available in no-std mode.
* Replies and events now implement `Serialize`.
* Support length expressions in structs in the protocol XML representation.
* Updated to xcb-proto 1.15.2 which brings support for the double buffering
  extension, dri3 1.3, xfixes 6.0, xinput 2.4, among other things.
* `x11rb::image::Image` now implements `Clone`.

Fixes:
* Fixed a broken link in the documentation.
* The cairo-example now also works under KDE.

Breaking changes:
* Consider use of enums as masks when determining enum sizes and then use our
  enum wrappers for mask fields. For example, all values of
  `xproto::ConfigWindow` fit into `u8`, so previously this was the type used for
  representing this enumeration. However, it is used as a mask for the
  `value_mask` field of `ConfigureRequestEvent`, which has 16 bits. Thus,
  `ConfigWindow` is now represented as `u16` and the `value_mask` field of
  `ConfigureRequestEvent` now has type `ConfigWindow`.
* Bump MSRV to 1.56 since `once_cell` switched to the 2021 edition.

Minor changes:
* Disable default features of nix crate.
* Deal with warnings from newest clippy.
* Improved unit test code coverage.
* Updated versions of dependencies.
* Switch to 2021 rust edition.
* Speed up `discard_reply()` implementation in Rust for cases with many pending
  replies via a binary search.

# Version 0.10.1 (2022-06-15)

Fixes:
* Fix compilation on non-Linux unixes.

# Version 0.10.0 (2022-06-12)

New features:
* x11rb was split into two crates: x11rb-protocol contains code for working with
  the X11 protocol, but does not do any X11 I/O. The x11rb crate uses
  x11rb-protocol to implement an X11 client. This change allows to use
  x11rb-protocol in other crates without depending on a whole X11 client.
* Lots of new utilities to implement a X11 client in preparation of a possible
  future x11rb-async. Most of these previously already existed, but were not
  public. Now, x11rb-protocol makes them available.
* Add traits around requests that allow to send a request in a generic way.
* Implement more traits in the generated code, where possible: `Default`,
  `PartialOrd`, `Ord`, and `Hash`.
* `x11rb-protocol` can be used in `no_std` mode.
* Optional serde support via deriving `serde::Serialize` and
  `serde::Deserialize`.
* Added support for X11 connections over abstract unix sockets.
* Increased the bus factor of the project by about 50%. Welcome @notgull!

Breaking changes:
* Bump MSRV to 1.46 due to the bitflags crate requiring this version. We depend
  on bitflags through the nix crate.
* Some minor API breakage due to the split into x11rb/x11rb-protocol. For
  example, `CreateWindowRequest::send()` was previously an associated function
  and was removed, because a similar feature is available via the new request
  traits. Also, the API for constructing a resource manager database was changed
  to remove the need for network I/O from its constructor. A convenience
  function for the old default behaviour is provided in `x11rb`.

Minor changes:
* Deal with warnings from newest clippy.
* Remove incorrect documentation for non-present fields from the generated code.
  This documentation was present because the field value is actually deduced
  from another field's value, for example the length of a vector.
* Add `#[must_use]` to setter methods on `*Aux` structs.
* Updated to latest versions of dependencies.
* Received FDs now automatically get the `CLOEXEC` flag applied.

# Version 0.9.0 (2021-08-29)

New features:
* Add `protocol::xproto::ClientMessageEvent::new()`.
* Add human readable information about the request causing the error to
  `x11_utils::X11Error`.
* Add RAII wrappers for many resources. For example,
  `protocol::xproto::PixmapWrapper` calls `free_pixmap()` in its `Drop`
  implementation.
* Add `wrapper::GrabServer` that calls `ungrab_server()` in its `Drop`
  implementation.

Breaking changes:
* Fix some `clippy::upper_case_acronyms` warnings:
  * Variants in `errors::ConnectError`:
    * `IOError` becomes `IoError`
    * `ZeroIDMask` becomes `ZeroIdMask`
    * `FDPassingFailed` becomes `FdPassingFailed`
  * Variants in `image::ImageOrder`:
    * `LSBFirst` becomes `LsbFirst`
    * `MSBFirst` becomes `MsbFirst`

Minor changes:
* `x11rb::connect()` now always returns a `RustConnection`. Previously, it
  returned `impl Connection` and used `XCBConnection` if that was enabled.
* Deal with warnings from newest clippy.
* Split up some large modules in the code generator.
* Update to latest version of crate dependencies that still support our MSRV.
* Bump our MSRV from Rust 1.40 to Rust 1.41.

# Version 0.8.1 (2021-03-10)

Minor changes:
* Update to latest version of crate dependencies.
* Fix some warnings from rustdoc.
* Deal with warnings from newest clippy.
* Try to clarify the documentation for X11 error handling.

# Version 0.8.0 (2021-01-09)

New features:
* Added a `resource_manager` library for querying the X11 resource database
  (Xrm).
* Added a `from_configure_request(&ConfigureRequestEvent)` function to
  `xproto::ConfigureWindowAux`.

Fixes:
* Rework `<enum>` representation so that we can represent invalid values from
  the X11 server. This mainly fixes problems with XInput.
* Add an `InvalidValue` variant to the representation of `<switch>`es. This
  variant is used when the server sends an invalid value for the `<switch>`
  discriminant. This fixes problems with XInput.
* The `cursor` code now depends on the Xrm code for proper database support.

Breaking changes:
* Reword `<enum>` representation: Instead of using a Rust `enum`, we now use a
  newtype around an integer. The named variants are provided as associated
  constants instead of enum variants. Due to Rust's naming rules, this means
  that e.g. `xproto::EventMask::ButtonRelease` is now called `BUTTON_RELEASE`.
* `ParseError` no longer implements `From<Infallible>`.
* Enumerations no longer implement conversion to `bool`.
* Removed unused `TryFrom<uXX>` implementations for enumerations.
* Removed `TryFrom<&[u8]>` implementations from the generated code.

Minor changes:
* Removed some unnecessary `unwrap()`s in the generated code.
* Minor improvements to the code generator.
* Fixed some new clippy warnings.
* Binary operations on enumeration values now preserve the type instead of
  mapping to integers (e.g. `ConfigWindow::WIDTH | 0` now has type
  `ConfigWindow` instead of `u8`).
* Add a `Debug` impl to enums that selects between `UPPER_CASE` and `CamelCase`
  based on the `alternate()` flag.
* Documentation improvements
* Added an internal helper trait `TryIntoUSize` to shorten the conversion from
  other numbers to `usize`.
* Remove some no longer necessary `#![allow]`s from the generated code.

# Version 0.7.0 (2020-10-11)

New features:
* An image utility was added under `x11rb::image`. This allows to do similar
  things as xcb's image library, e.g. endian conversion. A new `display_ppm`
  example showcases this new library a bit.
* Add a `dl-libxcb` feature. With this feature, we do not link against
  `libxcb.so` for `XCBConnection`, but instead load the library at runtime via
  `libloading`.
* Request structs now have a `send()` method. This allows for more readable
  request sending, without having to wonder "what was the fifth argument to
  `create_window` again?":
```
conn.create_window(
    screen.root_depth,
    win_id,
    screen.root,
    0,
    0,
    width,
    height,
    0,
    WindowClass::InputOutput,
    0,
    &win_aux,
)?;
CreateWindowRequest {
    depth: screen.root_depth,
    wid: win_id,
    parent: screen.root,
    x: 0,
    y: 0,
    width,
    height,
    border_width: 0,
    class: WindowClass::InputOutput,
    visual: 0,
    value_list: std::borrow::Cow::Borrowed(&win_aux),
}.send(conn)?;
```

Fixes:
* X11 errors are now represented by a single struct
  `x11rb::x11_utils::X11Error`, because all errors are structurally equivalent.
  Only the error code differs.
* More features are enabled for `docs.rs`, hopefully making the docs complete.
* Improve support for the record extension and add an example that shows how to
  use it.

Breaking changes:
* The minimum supported Rust version is now Rust 1.40 due to
  `#[non_exhaustive]`.
* The enumeration `x11rb::errors::ParseError` was split up into multiple values
  that now provide more information about which kind of error occurred.
* Hide `x11rb::cursor` behind a feature gate.
* `x11rb::protocol::Error` and all individual error structs were removed and
  replaced with `x11rb::x11_utils::X11Error`. This removes about 2.5k LOC of
  generated code.
* Some enums are now marked as `#[non_exhaustive]`. This includes all enums in
  the generated protocol code.

Minor changes:
* `x11rb::utils::RawFdContainer` now always implements `Drop`, even on
  non-`cfg(unix)` systems.
* More items received doc comments. Only the generated code is not fully
  documented.
* Added a new `xtrace-example` that runs a program and prints all of its X11
  traffic to the console.
* The `simple_window_manager` example was improved. It now has better support
  for titlebars: Only clicks inside the close button close the window and the
  remaining area allows to move a window. Also, the example now uses the X11
  save set.
* Update dependencies to `nix` 0.18 and `winapi-wsapoll` 0.1.1.
* Minor internal changes to please newer clippy versions.

# Version 0.6.0 (2020-06-19)

New features:
* The examples in this repository where extended:
  * New `xclock_utc` example shows a clock. This is useful to see how to get a
    wakeup once per second.
  * `cairo-example` uses a transparent background if a EWMH compliant composite
    manager is running.
* The value of length fields are not part of the generated code. This release
  adds accessors that allow recomputing their value. The motivation for this
  change was `xproto::GetModiferMappingReply`'s `keycodes_per_modifier` field.
* Added `struct`s for X11 requests and added infrastructure for parsing
  requests. Interested users should start with the `protocol::Request` enum.
* Added a `protocol::Reply` enum over all possible replies.
* Added a `TryParseFd` trait for types that can be parsed from a combination of
  raw bytes and a list of file descriptors.

Fixes:
* Use nonblocking reads and writes for all interactions with the X11 server.
  This fixes a theoretical deadlock with the X11 server that was never seen in
  practice. Bindings for `poll()` are now required.
* `RustConnection::poll_for_event()` previously only checked for pending queued
  event, but did not actually try to read new events from the X11 server.
* There was a possible deadlock on low-level socket errors. A thread could end
  up waiting on a condition variable without ever getting woken up.
* The support for big requests was broken. Big requests are requests with more
  than 2^18 bytes. An incorrect length field was sent, cutting of the last four
  bytes of the request and causing them to be interpreted as a new request.
* The parsing codes for errors, events, and replies now return a correct
  `remaining` slice.

Breaking changes:
* The whole code around `rust_connection`'s low level stream abstraction was
  redesigned. This now uses an abstraction over the `poll()` function from libc
  (and requires similar support for all alternative transport implementations).
* The `allow-unsafe-code` feature is no longer enabled by default.
* `xkb_select_event`'s `affect_which` field is now deduced based on other
  arguments and no longer has to be provided explicitly.
* Changed return type of `VoidCookie::check()` from `Result<Option<Error>,
  ConnectionError>` to `Result<(), ReplyError>`. This still has the same
  possible errors as before, but they are represented differently.
* Remove the `response_type` fields from errors and replies, because they always
  have a fixed value (`0` for errors and `1` for replies).

Minor changes:
* `x11rb::rust_connection::Stream` now implements the traits `AsRawFd`,
  `IntoRawFd`, `AsRawSocket`, and `IntoRawSocket`.
* `x11rb::rust_connection::RustConnection` provides immutable access to its low
  level stream via the `stream()` method.
* Reworked CI integration. Tests are now also run on a big endian platform.
* Brought back the [explanation of the generated code](generated_code.md).
* Added a module-level doc comment to the generated code.
* Satisfy the newest version of our clippy overload for she has many complaints.

# Version 0.5.0 (2020-05-10)

New features:
* The new `Connection::prefetch_maximum_request_bytes()` API allows avoiding a
  round-trip when sending large requests.
* Use enums to represent values in structs where it makes sense. Previously,
  numeric types like `u8` were used. This simplifies usage of much of the API.
* `x11rb::extension_manager::ExtensionInformation` can now provide information
  about all the extensions that are known to be present.
* New enums `x11rb::protocol::Error` and `Event` allow representing any possible
  error or event in a parsed form. This allows e.g. `match`ing on events for
  event handling. Also, these enums implement `Debug` and thus allow printing
  human-readable output on unexpected errors or events. This type also abstracts
  some of the complications of parsing events away from the library user. The
  `Connection` trait was extended to be able to parse events/errors into these
  new types.
* Some arguments to request functions became generic where it makes sense. For
  example, `xproto::change_property` now accepts `Into<ATOM>` instead of just
  `ATOM` as the name of the property to be changed. Thus, the predefined atoms
  like `AtomEnum::WM_NAME` can be used directly without having to call `.into()`
  yourself.
* Type names now follow Rust convention. For example, `xproto::WINDOW` is now
  called `xproto::Window`.
* Added utilities for working `WM_CLASS`, `WM_SIZE_HINTS`, and `WM_SIZE`
  properties.
* Implement `Serialize` and `TryParse` for tuples.
* Increased the bus factor of the project by about factor 2. Welcome @eduardosm!
* Replace the Python code generator with a generator written in Rust.
* Added an API for reading a cursor from the active cursor theme. The
  `simple_window` example was extended to use the new API.
* `RustConnection` now supports FD passing.

Fixes:
* A possible hang when a `QueryExtension` request failed was fixed.
* Improve FFI definitions used by `XCBConnection`.
* Fix some shadowing issues in the generated code. For example, GLX has its own
  `Pixmap` type, but also wants to refer to xproto's `Pixmap` in some places.
* Alignment pads were incorrectly ignored in `<switch>` cases with only one
  visible field. This affected only the xinput and xkb extensions.
* Properly reconstruct sequence numbers in `XCBConnection` after 2^32 requests.
  The old code only ever provided 32 bits of the sequence number.
* Fix compiler error on empty `atom_manager!` invocations.
* Request serialisation panics if some overflow occurs instead of producing
  incorrect data.

Breaking changes:
* The `vendor-xcb-proto` feature flag is no longer available. The included
  xcb-proto is now always used.
* The `xproto` feature flag was removed. It did not do anything.
* The module `x11rb::generated` was renamed to `x11rb::protocol`.
* Better snake names are generated, e.g. `XIQueryVersion` becomes
  `xi_query_version` instead of `xiquery_version`.
* Opcodes for events using the generic event extension are now `u16`.
* `x11rb::connection::Connection::compute_length_field()` was moved out of the
  `Connection` trait and to `x11rb::connection::compute_length_field()`.
* Use enums to represent values in structs where it makes sense.
* Enums that collide with types now have the suffix `Enum` attached to their
  name. For example, `xproto::Atom` is now called `xproto::AtomEnum`.
* Return a simplified struct from `RequestConnection::extension_information`
  instead of the full `QueryExtensionReply`.
* Rename the `ExtensionInformation` struct to `ExtensionManager`.
* `GenericEvent`, `GenericError`, and the related `Event` trait were removed.
* Changes to `RequestConnection` and `Connection` that affect implementations of
  these traits. For example, methods `parse_event` and `parse_error` were added.
* The `Connection` trait now returns parsed event and errors of type
  `x11rb::protocol::Event` and `Error`, respectively. The old API providing
  bytes is retained with alternative names. For example, there is
  `wait_for_event()` and `wait_for_raw_event()`.
* Some functions in the `Connection` trait now return X11 errors in their `Ok`
  variant via a new `ReplyOrError` enum. This should only affect implementations
  of the `Connection` trait.
* Added some wrapper structs in some parts of `xinput`. This is related to
  `<switch>` handling.
* Swap the order of elements in `EventAndSeqNumber`.
* Remove some length fields from the public API that can be deduced from the
  length of other values.
* Some `Serialize` implementations were removed from types that depend on value
  from the surrounding context.
* `RustConnection` now supports FD passing. This removes the API for
  constructing a `RustConnection` from a pair of `Read` and `Write`, but
  alternatives for the new API exists.
* `RawFdContainer` provides no methods when it is not available. This turns some
  run-time errors into compile-time errors.

Minor changes:
* No code is generated anymore at build times. Instead, the generated code is
  shipped with the crate.
* The code generator sorts the list of XML files before generating output,
  ensuring a stable output independent of file system order.
* Some parsing code was simplified, saving 5903 (according to git's `--stat`
  output).
* The vendored copy of xcb-proto was updated.
* Some fixes to the internal XCB FFI mock that is used for testing.
* The code generator and `xcb-proto` are excluded from releases, shrinking the
  size of the crate a bit.
* `XCBConnection` now uses `xcb_poll_for_reply64()` instead of
  `xcb_poll_for_reply()`. This means that libxcb 1.12 or newer is required.
* The `simple_window` example sets more properties.
* Some readability and general improvements to the generated code.
* Implement conversion to `bool` for enums with just two values.
* Added a special fast path for parsing lists of `u8`.
* Some refactoring to the I/O in `RustConnection`.
* Implement `From<ParseError>` for `ReplyOrIdError`.
* Added an example showing how to use x11rb with cairo.
* `RawFdContainer` no longer depends on the `allow-unsafe-code` feature.
* `RawFdContainer` no longer panics when the `close` on drop fails.
* New API `close()` and `try_clone()` is available on `RawFdContainer`.
* Use type aliases for `<eventcopy>` and `<errorcopy>` definitions. This means
  that the `Debug` implementation may now print the 'wrong' type name, but
  shortens the generated code by more than 4000 lines.

# Version 0.4.1 (2020-03-12)

Fixes for XKB:
* Fix encoding of `xkb::SelectEvents`. More details can be found in commit
  83ff34452bf9.
* Make fields of `switch` structs public so that e.g. `GetMapMap` becomes
  usable.

Minor changes:
* Only depend on libc if the `allow-unsafe-code` feature is enabled.
* Reexport error/event type in `rust_connection`/`xcb_ffi` modules.
* Move `multithread_test` from `examples/` to `tests/`.

# Version 0.4.0 (2020-03-08)

New features:
* Add support for the XKB and XInput extensions to the code generator.
  * x11rb now supports the same X11 extension that libxcb supports!
  * The `GetKbdByName`, `GetGeometry`, `SetGeometry`, `ListComponents` are not
    correctly described by xcb-proto and thus still unsupported.
* Add an `allow-unsafe-code` and `forbid(unsafe-code)` without this feature.
* Add `x11rb::connect()` for establishing a connection to an X11 server.
  Depending on the `allow-unsafe-code` feature, this either returns a
  `RustConnection` or an `XCBConnection`.
* Make `RustConnection` fully functional and thread-safe.
  * The only known missing features are FD-passing and XDM-AUTHORIZATION-1
    support.
  * This required a new dependency on the `gethostname` crate.
* Add an `atom_manager!` macro.
  * This macro allows to generate a struct that queries many atoms with a single
    round-trip.
* Add a `Serialize` trait for producing wire bytes from an in-memory
  representation.
  * This trait can be implemented for all types and slices, simplifying the
    generated Rust code.
  * Serialization is possible into a returned `Vec<u8>` or by appending into a
    provided `&mut Vec<u8>`.
* Add helpers to `GetPropertyReply` that simplify its interpretation.
  * This adds `value8(&self)`, `value16(&self)` and `value32(&self)` methods
    that check for the correct format and return an iterator over the value.
* Add a function to create an `XCBConnection` from a raw pointer.
* Add `Connection::wait_for_event_with_sequence()` and
  `Connection::poll_for_event_with_sequence()` that allow to get an event
  together with its full sequence number.
* Add API for prefetching extension information.
* General improvements to the documentation.
* Emit correct `cargo:rerun-if-changed=` lines from `build.rs`.
* Make `x11rb::utils::CSlice` safer.
* Use `rustfmt`.
* Add AppVeyor for Windows CI.

Breaking changes:
* Add feature gates for individual X11 extensions. The `all-extensions` feature
  enables all X11 extensions.
* Introduce `Result`s in some places that previously ignored errors or paniced.
* Split up `ConnectionError` into two enums, one for errors that can occur while
  establishing a connection and another for errors on an already-established
  connection.
* Prefix request function names in the `ConnectionExt` traits with the extension
  name. This is necessary since several extensions contain different versions of
  the same request.
  * As an example, `x11rb::generated::shm::ConnectionExt::create_pixmap` is
    now called `shm_create_pixmap` to avoid a collision with
    `x11rb::generated::xproto::ConnectionExt::create_pixmap`. The plain function
    `shm::create_pixmap` kept its name.
* Rename `ConnectionErrorOrX11Error` to `ReplyError`.
* Remove `x11rb::utils::Buffer` type. Instead, there is an associated type
  `Connection::Buf`. This is `Vec<u8>` for `RustConnection` and `CSlice` for
  `XCBConnection`.
* Remove `LazyAtom`.

Changes to the generated code and the code generator.
* Implement `From` instead of `Into` in the generated code.
* Use better type names in the generated code, e.g. `WINDOW` instead of `u32`.
* Simplify and fix some warnings in the generated code without changing visible
  behaviour.
* Change internal storage of unions in the generated code from `Vec` to fixed
  length arrays.
* Fix xproto's `SetModifierMapping` request. The code generator was ignoring an
  expression that required a multiplication with a constant value to get the
  length of a list.
* Fix xproto's `SetupAuthenticate` and res's `ClientIdValue` serialization. An
  expression in the XMl was ignored.
* Specify enum discriminators where possible.
* Implement `TryFrom<uX>` for enums.

Many thanks to @dalcde and @eduardosm for their valuable contributions.

# Version 0.3.0 (2020-01-04)

* Split out some types from `x11rb::connection` into their own modules.
* Split `xcb_ffi` and `rust_connection` into multiple modules.
* Use type aliases like `WINDOW` in the generated code instead of `u32`.
* Fix a type confusion in the SYNC extension's generated code where `i64` was
  used instead of the correct `Int64`.
* Update the vendored copy of xcb-proto so that `bool` types can be properly
  handled in the generated code.
* Add constants to extensions containing the extension version number.
* Add a `LazyAtom` type that sends an `InternAtom` request and asynchronously
  waits for the answer.
* Implement `Clone`, `PartialEq`, and `Eq` for various types.
* Add various "zero constants" that XCB defines in `xcb.h`.
* Allow `Connection` implementations to be unsized (`?Sized`).
* Split the `Connection` trait into `RequestConnection` and `Connection`.
* Run Clippy on the code and deal with its findings.
* Return a copy instead of a reference from `extension_information()`.
* Allow the creation of union instances via `From`.
* Add module-level functions for each request instead of having them only
  available in traits. This simplifies handling of some name collisions, e.g.
  around `query_version` being defined by basically all extensions.
* Add support for XGE events to the code generator and `XCBConnection`.
* Mock libxcb for unit tests.
* Implement parsing from borrowed data where possible, i.e. `TryFrom<&Buffer>`,
  `From<&GenericEvent>`, and `From<&GenericError>`.
* Implement `From<GenericError>` for `GenericEvent`.
* Clean up some of the internals of `XCBConnection` and `RustConnection`.
* Add API for checked/unchecked requests and improve documentation on X11 error
  handling.
* Simplify and document the code generator.
* Use examples as integration tests on Travis.
* Fix padding with non-unitary alignment.
* Implement implicit padding at end of requests.
* Fix padding length calculation in conjunction with `<pad align="4"/>`.
* Add `sequence_number()` and `raw_reply()` functions to all cookies.
* Add a `Makefile` that allows to evaluate changes to the code generator. Usage
  is `make ref`, then changing the code generator, then `make cmp` to get a diff
  with the changes.
* Add support for some of the XML used for the XInput extension.
* Implement `TryParse` only for structs.
* Deny more Rust lints and fix resulting errors.
* Add support for sending and receiving file descriptors (FD passing).
* Add more examples.
* Add a sync() function behaving like `xcb_aux_sync()` / `XSync()`.
* Test rustc 1.37.0 on Travis as minimum supported Rust version.


# Version 0.2.0 (2019-11-02)

* The `only_if_exists` parameter of `InternAtom` has type `bool`.
* Make `SendEvent` usable. Events now implement `Into<[u8; 32]>`.
* Add fields like `sequence` to the generates structs for replies, errors, and
  events.
* Implement `From<GenericEvent>` for events and `From<GenericError>` for errors.
* Add support for big requests.
* Derive `Eq` for the error enums.
* Return `Result` from `send_request_with/without_reply()`.
* Make `XCBConnection` Send and Sync.
* Add a getter for `xcb_connection_t*` to `XCBConnection`.
* Implement `AsRawFd` for `XCBConnection`.
* Move functions for sending requests into an extension trait.
* Generate documentation from the `<doc>` tags in the XML.
* Add a copy of xcb-proto 1.13 to this crate. This copy is used if the
  `vendor-xcb-proto` feature is enabled, which it is by default. Thus, this
  crate becomes easier to compile.
* Change calls to `xcb_send_request64()` so that the indices -1 and -2 are
  valid.
* Add `poll_for_event()` to `Connection`.
* Improve the code generator so that all extensions except xinput and xkb are
  supported.
* Improve the examples.
  * New examples: `xeyes`, `hypnomoire`, `tutorial`
* Add hand-written wrappers to simplify use of `ChangeProperty`.
* Improve documentation
* Fix length calculations for lists with expressions (`ChangeProperty`,
  `ChangeKeyboardMapping`, `GetPropertyReply`, ...)
* Fix padding calculation for lists with elements with sizes different from
  zero.
* Added a tutorial as `examples/tutorial.rs`.


# Version 0.1.0 (2019-10-11)

* First release.
