

Async/await
-----------

No. If you have so many X11 connections that this would matter, you are doing
something wrong. Also, it encourages people to write high-latency code. You
should send all your requests before waiting for the replies.

Future work
-----------

- FD passing
- checked requests (needed?)
  - Add `connection.check_request(sequence)` and be done?
- thread safe (connection should be Send and Sync)
- Rewrite it in Rust - a non-ffi based library
