# Version 0.9.0 (2021-XX-XX)

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
