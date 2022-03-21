//! A pure-rust implementation of a connection to an X11 server.

use std::convert::TryInto;
use std::io::IoSlice;
use std::mem::drop;
use std::sync::{Condvar, Mutex, MutexGuard, TryLockError};

use crate::connection::{
    compute_length_field, Connection, ReplyOrError, RequestConnection, RequestKind,
};
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
pub use crate::errors::{ConnectError, ConnectionError, ParseError, ReplyError, ReplyOrIdError};
use crate::extension_manager::ExtensionManager;
use crate::protocol::bigreq::{ConnectionExt as _, EnableReply};
use crate::protocol::xproto::{Setup, SetupRequest, GET_INPUT_FOCUS_REQUEST};
use crate::utils::RawFdContainer;
use crate::x11_utils::{ExtensionInformation, Serialize, TryParse, TryParseFd};
use x11rb_protocol::connection::{Connection as ProtoConnection, PollReply, ReplyFdKind};
use x11rb_protocol::id_allocator::IdAllocator;
use x11rb_protocol::{DiscardMode, RawEventAndSeqNumber, SequenceNumber};

mod packet_reader;
mod stream;
mod write_buffer;
mod xauth;

use packet_reader::PacketReader;
pub use stream::{DefaultStream, PollMode, Stream};
use write_buffer::WriteBuffer;

type Buffer = <RustConnection as RequestConnection>::Buf;
/// A combination of a buffer and a list of file descriptors for use by [`RustConnection`].
pub type BufWithFds = crate::connection::BufWithFds<Buffer>;

#[derive(Debug)]
enum MaxRequestBytes {
    Unknown,
    Requested(Option<SequenceNumber>),
    Known(usize),
}

#[derive(Debug)]
struct ConnectionInner {
    inner: ProtoConnection,
    write_buffer: WriteBuffer,
}

type MutexGuardInner<'a> = MutexGuard<'a, ConnectionInner>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum BlockingMode {
    Blocking,
    NonBlocking,
}

/// A connection to an X11 server implemented in pure rust
///
/// This type is generic over `S`, which allows to use a generic stream to communicate with the
/// server. This stream can written to and read from, but it can also be polled, meaning that one
/// checks if new data can be read or written.
///
/// `RustConnection` always used an internal buffer for reading, so `R` does not need
/// to be buffered.
#[derive(Debug)]
pub struct RustConnection<S: Stream = DefaultStream> {
    inner: Mutex<ConnectionInner>,
    stream: S,
    // This mutex is only locked with `try_lock` (never blocks), so a simpler
    // lock based only on a atomic variable would be more efficient.
    packet_reader: Mutex<PacketReader>,
    reader_condition: Condvar,
    setup: Setup,
    extension_manager: Mutex<ExtensionManager>,
    maximum_request_bytes: Mutex<MaxRequestBytes>,
    id_allocator: Mutex<IdAllocator>,
}

// Locking rules
// =============
//
// To avoid deadlocks, it is important to have a defined ordering about mutexes:
//
// Mutexes that may be locked when no other mutex is held:
// - maximum_request_bytes
// - extension_manager
// - id_allocator
//
// Then comes `inner`. This mutex protects the information about in-flight requests and packets
// that were already read from the connection but not given out to callers. This mutex also
// contains the write buffer and has to be locked in order to write something to the X11 server.
// In this case, the mutex has to be kept locked until writing the request has finished. This is
// necessary to ensure correct sync insertion without threads interfering with each other. When
// this mutex is locked for operations other than writing, the lock should be kept only for a
// short time.
//
// The inner level is `packet_reader`. This mutex is only locked when `inner` is already held and
// only with `try_lock()`. This ensures that there is only one reader. While actually reading, the
// lock on `inner` is released so that other threads can make progress. If more threads want to
// read while `read` is already locked, they sleep on `reader_condition`. The actual reader will
// then notify this condition variable once it is done reading.
//
// n.b. notgull: write_buffer follows the same rules
//
// The condition variable is necessary since one thread may read packets that another thread waits
// for. Thus, after reading something from the connection, all threads that wait for something have
// to check if they are the intended recipient.

impl RustConnection<DefaultStream> {
    /// Establish a new connection.
    ///
    /// If no `dpy_name` is provided, the value from `$DISPLAY` is used.
    pub fn connect(dpy_name: Option<&str>) -> Result<(Self, usize), ConnectError> {
        // Parse display information
        let parsed_display = x11rb_protocol::parse_display::parse_display(dpy_name)
            .ok_or(ConnectError::DisplayParsingError)?;

        // Establish connection
        let protocol = parsed_display.protocol.as_deref();
        let stream =
            DefaultStream::connect(&*parsed_display.host, protocol, parsed_display.display)?;
        let screen = parsed_display.screen.into();

        let (family, address) = stream.peer_addr()?;
        let (auth_name, auth_data) = xauth::get_auth(family, &address, parsed_display.display)
            // Ignore all errors while determining auth; instead we just try without auth info.
            .unwrap_or(None)
            .unwrap_or_else(|| (Vec::new(), Vec::new()));

        Ok((
            Self::connect_to_stream_with_auth_info(stream, screen, auth_name, auth_data)?,
            screen,
        ))
    }
}

impl<S: Stream> RustConnection<S> {
    /// Establish a new connection to the given streams.
    ///
    /// `read` is used for reading data from the X11 server and `write` is used for writing.
    /// `screen` is the number of the screen that should be used. This function checks that a
    /// screen with that number exists.
    pub fn connect_to_stream(stream: S, screen: usize) -> Result<Self, ConnectError> {
        Self::connect_to_stream_with_auth_info(stream, screen, Vec::new(), Vec::new())
    }

    /// Establish a new connection to the given streams.
    ///
    /// `read` is used for reading data from the X11 server and `write` is used for writing.
    /// `screen` is the number of the screen that should be used. This function checks that a
    /// screen with that number exists.
    ///
    /// The parameters `auth_name` and `auth_data` are used for the members
    /// `authorization_protocol_name` and `authorization_protocol_data` of the `SetupRequest` that
    /// is sent to the X11 server.
    pub fn connect_to_stream_with_auth_info(
        stream: S,
        screen: usize,
        auth_name: Vec<u8>,
        auth_data: Vec<u8>,
    ) -> Result<Self, ConnectError> {
        write_setup(&stream, auth_name, auth_data)?;
        let setup = read_setup(&stream)?;

        // Check that we got a valid screen number
        if screen >= setup.roots.len() {
            return Err(ConnectError::InvalidScreen);
        }

        // Success! Set up our state
        Self::for_connected_stream(stream, setup)
    }

    /// Establish a new connection for an already connected stream.
    ///
    /// The given `stream` is used for communicating with the X11 server.
    /// It is assumed that `setup` was just received from the server. Thus, the first reply to a
    /// request that is sent will have sequence number one.
    pub fn for_connected_stream(stream: S, setup: Setup) -> Result<Self, ConnectError> {
        Self::for_inner(stream, ProtoConnection::new(), setup)
    }

    fn for_inner(stream: S, inner: ProtoConnection, setup: Setup) -> Result<Self, ConnectError> {
        let id_allocator = IdAllocator::new(setup.resource_id_base, setup.resource_id_mask)?;

        Ok(RustConnection {
            inner: Mutex::new(ConnectionInner {
                inner,
                write_buffer: WriteBuffer::new(),
            }),
            stream,
            packet_reader: Mutex::new(PacketReader::new()),
            reader_condition: Condvar::new(),
            setup,
            extension_manager: Default::default(),
            maximum_request_bytes: Mutex::new(MaxRequestBytes::Unknown),
            id_allocator: Mutex::new(id_allocator),
        })
    }

    /// Internal function for actually sending a request.
    ///
    /// This function "does the actual work" for `send_request_with_reply()` and
    /// `send_request_without_reply()`.
    fn send_request(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
        kind: ReplyFdKind,
    ) -> Result<SequenceNumber, ConnectionError> {
        let mut storage = Default::default();
        let bufs = compute_length_field(self, bufs, &mut storage)?;

        // Note: `inner` must be kept blocked until the request has been completely written
        // or buffered to avoid sending the data of different requests interleaved. For this
        // reason, `read_packet_and_enqueue` must always be called with `BlockingMode::NonBlocking`
        // during a write, otherise `inner` would be temporarily released.
        let mut inner = self.inner.lock().unwrap();

        loop {
            match inner.inner.send_request(kind) {
                Some(seqno) => {
                    // Now actually send the buffers
                    let _inner = self.write_all_vectored(inner, bufs, fds)?;
                    return Ok(seqno);
                }
                None => {
                    inner = self.send_sync(inner)?;
                }
            }
        }
    }

    /// Send a synchronisation packet to the X11 server.
    ///
    /// This function sends a `GetInputFocus` request to the X11 server and arranges for its reply
    /// to be ignored. This ensures that a reply is expected (`ConnectionInner.next_reply_expected`
    /// increases).
    fn send_sync<'a>(
        &'a self,
        mut inner: MutexGuardInner<'a>,
    ) -> Result<MutexGuardInner<'a>, std::io::Error> {
        let length = 1u16.to_ne_bytes();
        let request = [
            GET_INPUT_FOCUS_REQUEST,
            0, /* pad */
            length[0],
            length[1],
        ];

        let seqno = inner
            .inner
            .send_request(ReplyFdKind::ReplyWithoutFDs)
            .expect("Sending a HasResponse request should not be blocked by syncs");
        inner
            .inner
            .discard_reply(seqno, DiscardMode::DiscardReplyAndError);
        let inner = self.write_all_vectored(inner, &[IoSlice::new(&request)], Vec::new())?;

        Ok(inner)
    }

    /// Write a set of buffers on a `writer`. May also read packets
    /// from the server.
    fn write_all_vectored<'a>(
        &'a self,
        mut inner: MutexGuardInner<'a>,
        mut bufs: &[IoSlice<'_>],
        mut fds: Vec<RawFdContainer>,
    ) -> std::io::Result<MutexGuardInner<'a>> {
        let mut partial_buf: &[u8] = &[];
        while !partial_buf.is_empty() || !bufs.is_empty() || !fds.is_empty() {
            self.stream.poll(PollMode::ReadAndWritable)?;
            let write_result = if !partial_buf.is_empty() {
                // "inner" is held, passed into this function, so this should never be held
                inner
                    .write_buffer
                    .write(&self.stream, partial_buf, &mut fds)
            } else {
                // same as above
                inner
                    .write_buffer
                    .write_vectored(&self.stream, bufs, &mut fds)
            };
            match write_result {
                Ok(0) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::WriteZero,
                        "failed to write anything",
                    ));
                }
                Ok(mut count) => {
                    // Successful write
                    if count >= partial_buf.len() {
                        count -= partial_buf.len();
                        partial_buf = &[];
                    } else {
                        partial_buf = &partial_buf[count..];
                        count = 0;
                    }
                    while count > 0 {
                        if count >= bufs[0].len() {
                            count -= bufs[0].len();
                        } else {
                            partial_buf = &bufs[0][count..];
                            count = 0;
                        }
                        bufs = &bufs[1..];
                        // Skip empty slices
                        while bufs.first().map(|s| s.len()) == Some(0) {
                            bufs = &bufs[1..];
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Writing would block, try to read instead because the
                    // server might not accept new requests after its
                    // buffered replies have been read.
                    inner = self.read_packet_and_enqueue(inner, BlockingMode::NonBlocking)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(inner)
    }

    fn flush_impl<'a>(
        &'a self,
        mut inner: MutexGuardInner<'a>,
    ) -> std::io::Result<MutexGuardInner<'a>> {
        // n.b. notgull: inner guard is held
        while inner.write_buffer.needs_flush() {
            self.stream.poll(PollMode::ReadAndWritable)?;
            match inner.write_buffer.flush(&self.stream) {
                // Flush completed
                Ok(()) => break,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Writing would block, try to read instead because the
                    // server might not accept new requests after its
                    // buffered replies have been read.
                    inner = self.read_packet_and_enqueue(inner, BlockingMode::NonBlocking)?;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(inner)
    }

    /// Read a packet from the connection.
    ///
    /// This function waits for an X11 packet to be received. It drops the mutex protecting the
    /// inner data while waiting for a packet so that other threads can make progress. For this
    /// reason, you need to pass in a `MutexGuard` to be dropped. This function locks the mutex
    /// again and returns a new `MutexGuard`.
    ///
    /// Note: If `mode` is `BlockingMode::Blocking`, the lock on `inner` will be temporarily
    /// released. While sending a request, `inner` must be kept locked to avoid sending the data
    /// of different requests interleaved. So, when `read_packet_and_enqueue` is called as part
    /// of a write, it must always be done with `mode` set to `BlockingMode::NonBlocking`.
    fn read_packet_and_enqueue<'a>(
        &'a self,
        mut inner: MutexGuardInner<'a>,
        mode: BlockingMode,
    ) -> Result<MutexGuardInner<'a>, std::io::Error> {
        // 0.1. Try to lock the `packet_reader` mutex.
        match self.packet_reader.try_lock() {
            Err(TryLockError::WouldBlock) => {
                // In non-blocking mode, we just return immediately
                match mode {
                    BlockingMode::NonBlocking => return Ok(inner),
                    BlockingMode::Blocking => {}
                }

                // 1.1. Someone else is reading (other thread is at 2.2);
                // wait for it. `Condvar::wait` will unlock `inner`, so
                // the other thread can relock `inner` at 2.1.3 (and to allow
                // other threads to arrive 0.1).
                //
                // When `wait` finishes, other thread has enqueued a packet,
                // so the purpose of this function has been fulfilled. `wait`
                // will relock `inner` when it returns.
                Ok(self.reader_condition.wait(inner).unwrap())
            }
            Err(TryLockError::Poisoned(e)) => panic!("{}", e),
            Ok(mut packet_reader) => {
                // Make sure sleeping readers are woken up when we return
                // (Even in case of errors)
                let notify_on_drop = NotifyOnDrop(&self.reader_condition);

                // 2.1. Poll for read if mode is blocking.
                if mode == BlockingMode::Blocking {
                    // 2.1.1. Unlock `inner`, so other threads can use it while
                    // during the poll.
                    drop(inner);
                    // 2.1.2. Do the actual poll
                    self.stream.poll(PollMode::Readable)?;
                    // 2.1.3. Relock inner
                    inner = self.inner.lock().unwrap();
                }

                // 2.2. Try to read as many packets as possible without blocking.
                let mut fds = Vec::new();
                let mut packets = Vec::new();
                packet_reader.try_read_packets(&self.stream, &mut packets, &mut fds)?;

                // 2.3. Once `inner` has been relocked, drop the
                // lock on `packet_reader`. While inner is locked, other
                // threads cannot arrive at 0.1 anyways.
                //
                // `packet_reader` must be unlocked with `inner` is locked,
                // otherwise it could let another thread wait on 2.1
                // for a reply that has been read but not enqueued yet.
                drop(packet_reader);

                // 2.4. Actually enqueue the read packets.
                inner.inner.enqueue_fds(fds);
                packets
                    .into_iter()
                    .for_each(|packet| inner.inner.enqueue_packet(packet));

                // 2.5. Notify the condvar by dropping the `notify_on_drop` object.
                // The object would have been dropped when the function returns, so
                // the explicit drop is not really needed. The purpose of having a
                // explicit drop is to... make it explicit.
                drop(notify_on_drop);

                // 2.6. Return the locked `inner` back to the caller.
                Ok(inner)
            }
        }
    }

    fn prefetch_maximum_request_bytes_impl(&self, max_bytes: &mut MutexGuard<'_, MaxRequestBytes>) {
        if let MaxRequestBytes::Unknown = **max_bytes {
            let request = self
                .bigreq_enable()
                .map(|cookie| cookie.into_sequence_number())
                .ok();
            **max_bytes = MaxRequestBytes::Requested(request);
        }
    }

    /// Returns a reference to the contained stream.
    pub fn stream(&self) -> &S {
        &self.stream
    }
}

impl<S: Stream> RequestConnection for RustConnection<S> {
    type Buf = Vec<u8>;

    fn send_request_with_reply<Reply>(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<Cookie<'_, Self, Reply>, ConnectionError>
    where
        Reply: TryParse,
    {
        Ok(Cookie::new(
            self,
            self.send_request(bufs, fds, ReplyFdKind::ReplyWithoutFDs)?,
        ))
    }

    fn send_request_with_reply_with_fds<Reply>(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<CookieWithFds<'_, Self, Reply>, ConnectionError>
    where
        Reply: TryParseFd,
    {
        Ok(CookieWithFds::new(
            self,
            self.send_request(bufs, fds, ReplyFdKind::ReplyWithFDs)?,
        ))
    }

    fn send_request_without_reply(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<VoidCookie<'_, Self>, ConnectionError> {
        Ok(VoidCookie::new(
            self,
            self.send_request(bufs, fds, ReplyFdKind::NoReply)?,
        ))
    }

    fn discard_reply(&self, sequence: SequenceNumber, _kind: RequestKind, mode: DiscardMode) {
        self.inner
            .lock()
            .unwrap()
            .inner
            .discard_reply(sequence, mode);
    }

    fn prefetch_extension_information(
        &self,
        extension_name: &'static str,
    ) -> Result<(), ConnectionError> {
        self.extension_manager
            .lock()
            .unwrap()
            .prefetch_extension_information(self, extension_name)
    }

    fn extension_information(
        &self,
        extension_name: &'static str,
    ) -> Result<Option<ExtensionInformation>, ConnectionError> {
        self.extension_manager
            .lock()
            .unwrap()
            .extension_information(self, extension_name)
    }

    fn wait_for_reply_or_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Result<ReplyOrError<Vec<u8>>, ConnectionError> {
        match self.wait_for_reply_with_fds_raw(sequence)? {
            ReplyOrError::Reply((reply, _fds)) => Ok(ReplyOrError::Reply(reply)),
            ReplyOrError::Error(e) => Ok(ReplyOrError::Error(e)),
        }
    }

    fn wait_for_reply(&self, sequence: SequenceNumber) -> Result<Option<Vec<u8>>, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        inner = self.flush_impl(inner)?;
        loop {
            match inner.inner.poll_for_reply(sequence) {
                PollReply::TryAgain => {}
                PollReply::NoReply => return Ok(None),
                PollReply::Reply(buffer) => return Ok(Some(buffer)),
            }
            inner = self.read_packet_and_enqueue(inner, BlockingMode::Blocking)?;
        }
    }

    fn check_for_raw_error(
        &self,
        sequence: SequenceNumber,
    ) -> Result<Option<Buffer>, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        if inner.inner.prepare_check_for_reply_or_error(sequence) {
            inner = self.send_sync(inner)?;
            assert!(!inner.inner.prepare_check_for_reply_or_error(sequence));
        }
        // Ensure the request is sent
        inner = self.flush_impl(inner)?;
        loop {
            match inner.inner.poll_check_for_reply_or_error(sequence) {
                PollReply::TryAgain => {}
                PollReply::NoReply => return Ok(None),
                PollReply::Reply(buffer) => return Ok(Some(buffer)),
            }
            inner = self.read_packet_and_enqueue(inner, BlockingMode::Blocking)?;
        }
    }

    fn wait_for_reply_with_fds_raw(
        &self,
        sequence: SequenceNumber,
    ) -> Result<ReplyOrError<BufWithFds, Buffer>, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        // Ensure the request is sent
        inner = self.flush_impl(inner)?;
        loop {
            if let Some(reply) = inner.inner.poll_for_reply_or_error(sequence) {
                if reply.0[0] == 0 {
                    return Ok(ReplyOrError::Error(reply.0));
                } else {
                    return Ok(ReplyOrError::Reply(reply));
                }
            }
            inner = self.read_packet_and_enqueue(inner, BlockingMode::Blocking)?;
        }
    }

    fn maximum_request_bytes(&self) -> usize {
        let mut max_bytes = self.maximum_request_bytes.lock().unwrap();
        self.prefetch_maximum_request_bytes_impl(&mut max_bytes);
        use MaxRequestBytes::*;
        match *max_bytes {
            Unknown => unreachable!("We just prefetched this"),
            Requested(seqno) => {
                let length = seqno
                    // If prefetching the request succeeded, get a cookie
                    .and_then(|seqno| {
                        Cookie::<_, EnableReply>::new(self, seqno)
                            // and then get the reply to the request
                            .reply()
                            .map(|reply| reply.maximum_request_length)
                            .ok()
                    })
                    // If anything failed (sending the request, getting the reply), use Setup
                    .unwrap_or_else(|| self.setup.maximum_request_length.into())
                    // Turn the u32 into usize, using the max value in case of overflow
                    .try_into()
                    .unwrap_or(usize::max_value());
                let length = length * 4;
                *max_bytes = Known(length);
                length
            }
            Known(length) => length,
        }
    }

    fn prefetch_maximum_request_bytes(&self) {
        let mut max_bytes = self.maximum_request_bytes.lock().unwrap();
        self.prefetch_maximum_request_bytes_impl(&mut max_bytes);
    }

    fn parse_error(&self, error: &[u8]) -> Result<crate::x11_utils::X11Error, ParseError> {
        let ext_mgr = self.extension_manager.lock().unwrap();
        crate::x11_utils::X11Error::try_parse(error, &*ext_mgr)
    }

    fn parse_event(&self, event: &[u8]) -> Result<crate::protocol::Event, ParseError> {
        let ext_mgr = self.extension_manager.lock().unwrap();
        crate::protocol::Event::parse(event, &*ext_mgr)
    }
}

impl<S: Stream> Connection for RustConnection<S> {
    fn wait_for_raw_event_with_sequence(
        &self,
    ) -> Result<RawEventAndSeqNumber<Vec<u8>>, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        loop {
            if let Some(event) = inner.inner.poll_for_event_with_sequence() {
                return Ok(event);
            }
            inner = self.read_packet_and_enqueue(inner, BlockingMode::Blocking)?;
        }
    }

    fn poll_for_raw_event_with_sequence(
        &self,
    ) -> Result<Option<RawEventAndSeqNumber<Vec<u8>>>, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        if let Some(event) = inner.inner.poll_for_event_with_sequence() {
            Ok(Some(event))
        } else {
            inner = self.read_packet_and_enqueue(inner, BlockingMode::NonBlocking)?;
            Ok(inner.inner.poll_for_event_with_sequence())
        }
    }

    fn flush(&self) -> Result<(), ConnectionError> {
        let inner = self.inner.lock().unwrap();
        let _inner = self.flush_impl(inner)?;
        Ok(())
    }

    fn setup(&self) -> &Setup {
        &self.setup
    }

    fn generate_id(&self) -> Result<u32, ReplyOrIdError> {
        let mut id_allocator = self.id_allocator.lock().unwrap();
        if let Some(id) = id_allocator.generate_id() {
            Ok(id)
        } else {
            use crate::protocol::xc_misc::{self, ConnectionExt as _};

            if self
                .extension_information(xc_misc::X11_EXTENSION_NAME)?
                .is_none()
            {
                // IDs are exhausted and XC-MISC is not available
                Err(ReplyOrIdError::IdsExhausted)
            } else {
                id_allocator.update_xid_range(&self.xc_misc_get_xid_range()?.reply()?)?;
                id_allocator
                    .generate_id()
                    .ok_or(ReplyOrIdError::IdsExhausted)
            }
        }
    }
}

#[cfg(target_endian = "little")]
fn byte_order() -> u8 {
    0x6c
}

#[cfg(target_endian = "big")]
fn byte_order() -> u8 {
    0x42
}

/// Send a `SetupRequest` to the X11 server.
fn write_setup(
    write: &impl Stream,
    auth_name: Vec<u8>,
    auth_data: Vec<u8>,
) -> Result<(), std::io::Error> {
    let request = SetupRequest {
        byte_order: byte_order(),
        protocol_major_version: 11,
        protocol_minor_version: 0,
        authorization_protocol_name: auth_name,
        authorization_protocol_data: auth_data,
    };
    // Write the data
    let data = request.serialize();
    let mut nwritten = 0;
    while nwritten != data.len() {
        write.poll(PollMode::Writable)?;
        // poll returned successfully, so the stream is writable.
        match write.write(&data[nwritten..], &mut Vec::new()) {
            Ok(0) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ))
            }
            Ok(n) => nwritten += n,
            // Spurious wakeup from poll, try again
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Read a `Setup` from the X11 server.
///
/// If the server sends a `SetupFailed` or `SetupAuthenticate` packet, these will be returned
/// as errors.
fn read_setup(stream: &impl Stream) -> Result<Setup, ConnectError> {
    let mut fds = Vec::new();
    let mut setup = vec![0; 8];
    stream.read_exact(&mut setup, &mut fds)?;
    let extra_length = usize::from(u16::from_ne_bytes([setup[6], setup[7]])) * 4;
    // Use `Vec::reserve_exact` because this will be the final
    // length of the vector.
    setup.reserve_exact(extra_length);
    setup.resize(8 + extra_length, 0);
    stream.read_exact(&mut setup[8..], &mut fds)?;
    if !fds.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "unexpectedly received FDs in connection setup",
        )
        .into());
    }
    match setup[0] {
        // 0 is SetupFailed
        0 => Err(ConnectError::SetupFailed(
            TryParse::try_parse(&setup[..])?.0,
        )),
        // Success
        1 => Ok(Setup::try_parse(&setup[..])?.0),
        // 2 is SetupAuthenticate
        2 => Err(ConnectError::SetupAuthenticate(
            TryParse::try_parse(&setup[..])?.0,
        )),
        // Uhm... no other cases are defined
        _ => Err(ParseError::InvalidValue.into()),
    }
}

/// Call `notify_all` on a condition variable when dropped.
#[derive(Debug)]
struct NotifyOnDrop<'a>(&'a Condvar);

impl Drop for NotifyOnDrop<'_> {
    fn drop(&mut self) {
        self.0.notify_all();
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::io::{Read, Result, Write};

    use super::{read_setup, PollMode, Stream};
    use crate::errors::ConnectError;
    use crate::protocol::xproto::{ImageOrder, Setup, SetupAuthenticate, SetupFailed};
    use crate::utils::RawFdContainer;
    use crate::x11_utils::Serialize;

    struct SliceStream<'a, 'b> {
        read_slice: RefCell<&'a [u8]>,
        write_slice: RefCell<&'b mut [u8]>,
    }

    impl<'a, 'b> Stream for SliceStream<'a, 'b> {
        fn poll(&self, _mode: PollMode) -> Result<()> {
            Ok(())
        }

        fn read(&self, buf: &mut [u8], _fd_storage: &mut Vec<RawFdContainer>) -> Result<usize> {
            self.read_slice.borrow_mut().read(buf)
        }

        fn write(&self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
            assert!(fds.is_empty());
            self.write_slice.borrow_mut().write(buf)
        }
    }

    #[test]
    fn read_setup_success() {
        let mut setup = Setup {
            status: 1,
            protocol_major_version: 11,
            protocol_minor_version: 0,
            length: 0,
            release_number: 0,
            resource_id_base: 0,
            resource_id_mask: 0,
            motion_buffer_size: 0,
            maximum_request_length: 0,
            image_byte_order: ImageOrder::LSB_FIRST,
            bitmap_format_bit_order: ImageOrder::LSB_FIRST,
            bitmap_format_scanline_unit: 0,
            bitmap_format_scanline_pad: 0,
            min_keycode: 0,
            max_keycode: 0,
            vendor: vec![],
            pixmap_formats: vec![],
            roots: vec![],
        };
        setup.length = ((setup.serialize().len() - 8) / 4) as _;
        let setup_bytes = setup.serialize();

        let stream = SliceStream {
            read_slice: RefCell::new(&setup_bytes),
            write_slice: RefCell::new(&mut []),
        };
        let read = read_setup(&stream);
        assert_eq!(setup, read.unwrap());
    }

    #[test]
    fn read_setup_failed() {
        let mut setup = SetupFailed {
            status: 0,
            protocol_major_version: 11,
            protocol_minor_version: 0,
            length: 0,
            reason: b"whatever".to_vec(),
        };
        setup.length = ((setup.serialize().len() - 8) / 4) as _;
        let setup_bytes = setup.serialize();

        let stream = SliceStream {
            read_slice: RefCell::new(&setup_bytes),
            write_slice: RefCell::new(&mut []),
        };
        match read_setup(&stream) {
            Err(ConnectError::SetupFailed(read)) => assert_eq!(setup, read),
            value => panic!("Unexpected value {:?}", value),
        }
    }

    #[test]
    fn read_setup_authenticate() {
        let setup = SetupAuthenticate {
            status: 2,
            reason: b"12345678".to_vec(),
        };
        let setup_bytes = setup.serialize();

        let stream = SliceStream {
            read_slice: RefCell::new(&setup_bytes),
            write_slice: RefCell::new(&mut []),
        };
        match read_setup(&stream) {
            Err(ConnectError::SetupAuthenticate(read)) => assert_eq!(setup, read),
            value => panic!("Unexpected value {:?}", value),
        }
    }
}
