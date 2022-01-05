# Comparison with other Rust X11 libraries

My main motivation for writing this library is fun and getting some experience
with Rust. As such, "there is already a library" does not count.

[![Motivation](https://imgs.xkcd.com/comics/standards.png)](https://xkcd.com/927/)

(The image is licensed under a Creative Commons Attribution-NonCommercial 2.5 License)

However, since you brought this topic up, let us look at some other libraries
that allow accessing an X11 server from Rust. If you know about more libraries
or want me to know that I got something wrong, feel free to tell me about them,
for example by opening an issue.


## xproto-rs

I only found this [on crates.io](https://crates.io/crates/xproto). The
Repository link is broken and documentation looks like someone dumped the result
of `bindgen` on the Xlib headers into a crate.


## xrb

The [Pure Rust bindings for X11](https://github.com/DaMrNelson/xrb) seem to
contain hand-written code for parsing and sending X11 messages. Also, its README
currently claims that this project is in an early state.


## x11-rs

This seems to provide FFI wrappers around Xlib and various related libraries. I
recently heard about this library because [its
unsafe](https://github.com/erlepereira/x11-rs/issues/99) code is
[unsound](https://github.com/rust-lang/rust/issues/52898) and causes undefined
behaviour. This is basically all I know and heard about this library.


## rust-xcb

This project uses xcb-proto, the XML description of the X11 protocol that comes
from the libxcb project. Based on this XML, code is generated that provides a
foreign function interface to the various libxcb libraries. Due to its FFI
nature, this project contains many instances of `unsafe`. Worse, its
`basic_window` example indicates that users of this library must also use
`unsafe` for [handling
events](https://github.com/rtbo/rust-xcb/blob/d7cb614a6fe9f4424ed26939a5720770f84acd05/examples/basic_window.rs#L66).
How can one ever be sure that there is nothing wrong with the `unsafe` one
writes?

I briefly looked at this project and found a [NULL pointer
dereference](https://github.com/rtbo/rust-xcb/issues/64) and re-discovered an
already known [leak](https://github.com/rtbo/rust-xcb/issues/57).


## xcb-dl

The [xcb-dl](https://github.com/mahkoh/xcb-dl) project seems to be similar to
rust-xcb: It provides FFI bindings to the libxcb C library. It uses the XML
description to generate this binding and the necessary types. It thus also has
many instances of `unsafe` and requires `unsafe` for using it.

The big difference to rust-xcb is that this library uses the libloading crate to
dynamically load function pointers at runtime instead of linking at compile
time.


## breadx

"An implementation of the X Window System Protocol in Rust. 100% safe and mutex-free."

[breadx](https://github.com/not-a-seagull/breadx) is another project that uses
the xcb-proto XML description of the X11 protocol to generate code. The project
claims that it is "generally faster (awaiting verification)".

One highlight is support for async. However, interactions with the X11 server
require `&mut`-access to the
[Display](https://docs.rs/breadx/0.1.3/breadx/display/struct.Display.html).
Thus, the Display needs to be wrapped in a `Mutex` or `RefCell` to share with
multiple async tasks.

Another highlight is `no_std` support.


## xcb-sys

The [xcb-sys crate](https://crates.io/crates/xcb-sys) uses bindgen to
automatically generate a Rust API from libxcb's C headers. Everything is unsafe
and as a user you are basically writing C code in Rust.

Put differently: There is lots of existing documentation (for C code) and
reference code (written in C) that you can use. It is pretty much directly
transferable to this library.


## x11rb (this project)

x11rb, the x11 rust bindings, is based on the XML description of the X11
protocol that comes from the libxcb project, similar to rust-xcb. However,
instead of providing a foreign function interface to libxcb, the generated code
reimplements the serialising and unserialising code that is provided by libxcb.
libxcb is only used for receiving and sending opaque packets.

This reimplementation tries to avoid uses of `unsafe` and thus should enjoy
Rust's usual safety guarantees. After all, the best way to trust the unsafe code
coming out of your code generator is if your code generator does not generate
any unsafe code. Unsafe code is currently necessary for FFI binding to a handful
of functions from libxcb (see `src/xcb_ffi.rs`).

This means that this project is even safer than libxcb, because libxcb forces
its users to blindly trust length fields that come from the X11 server.

The downside of this is possibly slower code. However, if your bottleneck is in
talking to the X11 server, you are seriously doing something wrong.

Examples of the generated code [can be found here](generated_code.md). Feel
free to suggest improvements to it.
