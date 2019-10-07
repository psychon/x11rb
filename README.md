# X11 rust bindings

TODO: Write a proper README.

## Motivation

![Motivation](https://imgs.xkcd.com/comics/standards.png)

(The image is licensed under a Creative Commons Attribution-NonCommercial 2.5 License)

- It should be possible to use an X11 library without using `unsafe`.
- Lifetimes should be tracked correctly.

## Async/await

No. If you have so many X11 connections that this would matter, you are doing
something wrong. Also, it encourages people to write high-latency code. You
should send all your requests before waiting for the replies.

## Future work

- big requests
- FD passing
- checked requests (needed?)
  - Add `connection.check_request(sequence)` and be done?
- ListFonts support
  - `MultiCookie` that is an iterator for replies?
- thread safe (connection should be Send and Sync)
- Rewrite it in Rust - a non-ffi based library

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
