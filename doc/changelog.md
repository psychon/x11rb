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
