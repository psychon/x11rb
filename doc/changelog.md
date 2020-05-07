# Version 0.5.0 (2020-XX-XX)

New features:
* The new `Connection::prefetch_maximum_request_bytes()` API allows avoiding a
  round-trip when sending large requests.
* Use enums to represent values in structs where it makes sense. Previously,
  numeric types like `u8` were used.  This simplifies usage of much of the API.
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
