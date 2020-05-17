//! A pure-rust implementation of a connection to an X11 server.

use std::convert::{TryFrom, TryInto};
use std::io::IoSlice;
use std::sync::{Condvar, Mutex, MutexGuard, TryLockError};

use crate::connection::{
    compute_length_field, Connection, DiscardMode, ReplyOrError, RequestConnection, RequestKind,
    SequenceNumber,
};
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
pub use crate::errors::{ConnectError, ConnectionError, ParseError, ReplyError, ReplyOrIdError};
use crate::extension_manager::ExtensionManager;
use crate::protocol::bigreq::{ConnectionExt as _, EnableReply};
use crate::protocol::xproto::{Setup, SetupRequest, GET_INPUT_FOCUS_REQUEST};
use crate::utils::RawFdContainer;
use crate::x11_utils::{ExtensionInformation, Serialize};

mod fd_read_write;
mod id_allocator;
mod inner;
mod packet_reader;
mod parse_display;
mod stream;
mod xauth;

pub use fd_read_write::{BufReadFD, BufWriteFD, ReadFD, ReadFDWrapper, WriteFD, WriteFDWrapper};
use inner::PollReply;
use packet_reader::PacketReader;

type Buffer = <RustConnection as RequestConnection>::Buf;
pub type RawEventAndSeqNumber = crate::connection::RawEventAndSeqNumber<Buffer>;
pub type BufWithFds = crate::connection::BufWithFds<Buffer>;

#[derive(Debug)]
enum MaxRequestBytes {
    Unknown,
    Requested(Option<SequenceNumber>),
    Known(usize),
}

type MutexGuardInner<'a> = MutexGuard<'a, inner::ConnectionInner>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ReplyFDKind {
    NoReply,
    ReplyWithoutFDs,
    ReplyWithFDs,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum BlockingMode {
    Blocking,
    NonBlocking,
}

impl BlockingMode {
    fn set_on_reader(self, read: &mut impl ReadFD) -> std::io::Result<()> {
        match self {
            BlockingMode::Blocking => read.set_nonblocking(false),
            BlockingMode::NonBlocking => read.set_nonblocking(true),
        }
    }
}

/// A connection to an X11 server implemented in pure rust
#[derive(Debug)]
pub struct RustConnection<
    R: ReadFD = BufReadFD<stream::Stream>,
    W: WriteFD = BufWriteFD<stream::Stream>,
> {
    inner: Mutex<inner::ConnectionInner>,
    read: Mutex<PacketReader<R>>,
    write: Mutex<W>,
    reader_condition: Condvar,
    id_allocator: Mutex<id_allocator::IDAllocator>,
    setup: Setup,
    extension_manager: Mutex<ExtensionManager>,
    maximum_request_bytes: Mutex<MaxRequestBytes>,
}

impl RustConnection<BufReadFD<stream::Stream>, BufWriteFD<stream::Stream>> {
    /// Establish a new connection.
    ///
    /// If no `dpy_name` is provided, the value from `$DISPLAY` is used.
    pub fn connect(dpy_name: Option<&str>) -> Result<(Self, usize), ConnectError> {
        // Parse display information
        let parsed_display =
            parse_display::parse_display(dpy_name).ok_or(ConnectError::DisplayParsingError)?;

        // Establish connection
        let protocol = parsed_display.protocol.as_ref().map(|s| &**s);
        let stream =
            stream::Stream::connect(&*parsed_display.host, protocol, parsed_display.display)?;
        let screen = parsed_display.screen.into();

        let (family, address) = stream.peer_addr()?;
        let (auth_name, auth_data) = xauth::get_auth(family, &address, parsed_display.display)
            // Ignore all errors while determining auth; instead we just try without auth info.
            .unwrap_or(None)
            .unwrap_or_else(|| (Vec::new(), Vec::new()));

        let write = BufWriteFD::new(stream.try_clone()?);
        let read = BufReadFD::new(stream);
        Ok((
            Self::connect_to_stream_with_auth_info(read, write, screen, auth_name, auth_data)?,
            screen,
        ))
    }
}

impl<R: ReadFD, W: WriteFD> RustConnection<R, W> {
    /// Establish a new connection to the given streams.
    ///
    /// `read` is used for reading data from the X11 server and `write` is used for writing.
    /// `screen` is the number of the screen that should be used. This function checks that a
    /// screen with that number exists.
    pub fn connect_to_stream(read: R, write: W, screen: usize) -> Result<Self, ConnectError> {
        Self::connect_to_stream_with_auth_info(read, write, screen, Vec::new(), Vec::new())
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
        mut read: R,
        mut write: W,
        screen: usize,
        auth_name: Vec<u8>,
        auth_data: Vec<u8>,
    ) -> Result<Self, ConnectError> {
        write_setup(&mut write, auth_name, auth_data)?;
        let setup = read_setup(&mut read)?;

        // Check that we got a valid screen number
        if screen >= setup.roots.len() {
            return Err(ConnectError::InvalidScreen);
        }

        // Success! Set up our state
        Self::for_connected_stream(read, write, setup)
    }

    /// Establish a new connection for an already connected stream.
    ///
    /// `read` is used for reading data from the X11 server and `write` is used for writing.
    /// It is assumed that `setup` was just received from the server. Thus, the first reply to a
    /// request that is sent will have sequence number one.
    pub fn for_connected_stream(read: R, write: W, setup: Setup) -> Result<Self, ConnectError> {
        Self::for_inner(read, write, inner::ConnectionInner::new(), setup)
    }

    fn for_inner(
        read: R,
        write: W,
        inner: inner::ConnectionInner,
        setup: Setup,
    ) -> Result<Self, ConnectError> {
        let allocator =
            id_allocator::IDAllocator::new(setup.resource_id_base, setup.resource_id_mask)?;
        Ok(RustConnection {
            inner: Mutex::new(inner),
            read: Mutex::new(PacketReader::new(read)),
            write: Mutex::new(write),
            reader_condition: Condvar::new(),
            id_allocator: Mutex::new(allocator),
            setup,
            extension_manager: Default::default(),
            maximum_request_bytes: Mutex::new(MaxRequestBytes::Unknown),
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
        kind: ReplyFDKind,
    ) -> Result<SequenceNumber, ConnectionError> {
        let mut storage = Default::default();
        let bufs = compute_length_field(self, bufs, &mut storage)?;

        let mut write = self.write.lock().unwrap();
        let mut inner = self.inner.lock().unwrap();
        loop {
            match inner.send_request(kind) {
                Some(seqno) => {
                    // Now actually send the buffers
                    // FIXME: We must always be able to read when we write
                    write_all_vectored(&mut *write, bufs, fds)?;
                    return Ok(seqno);
                }
                None => self.send_sync(&mut *inner, &mut *write)?,
            }
        }
    }

    /// Send a synchronisation packet to the X11 server.
    ///
    /// This function sends a `GetInputFocus` request to the X11 server and arranges for its reply
    /// to be ignored. This ensures that a reply is expected (`ConnectionInner.next_reply_expected`
    /// increases).
    fn send_sync(
        &self,
        inner: &mut inner::ConnectionInner,
        write: &mut W,
    ) -> Result<(), std::io::Error> {
        let length = 1u16.to_ne_bytes();
        let request = [
            GET_INPUT_FOCUS_REQUEST,
            0, /* pad */
            length[0],
            length[1],
        ];

        let seqno = inner
            .send_request(ReplyFDKind::ReplyWithoutFDs)
            .expect("Sending a HasResponse request should not be blocked by syncs");
        inner.discard_reply(seqno, DiscardMode::DiscardReplyAndError);
        write_all_vectored(write, &[IoSlice::new(&request)], Vec::new())?;

        Ok(())
    }

    /// Read a packet from the connection.
    ///
    /// This function waits for an X11 packet to be received. It drops the mutex protecting the
    /// inner data while waiting for a packet so that other threads can make progress. For this
    /// reason, you need to pass in a `MutexGuard` to be dropped. This function locks the mutex
    /// again and returns a new `MutexGuard`.
    fn read_packet_and_enqueue<'a>(
        &'a self,
        mut inner: MutexGuardInner<'a>,
        mode: BlockingMode,
    ) -> Result<MutexGuardInner<'a>, std::io::Error> {
        // 0.1. Try to lock the `read` mutex.
        match self.read.try_lock() {
            Err(TryLockError::WouldBlock) => {
                // In non-blocking mode, we just return immediately
                match mode {
                    BlockingMode::NonBlocking => return Ok(inner),
                    BlockingMode::Blocking => {}
                }

                // 1.1. Someone else is reading (other thread is at 2.2);
                // wait for it. `Condvar::wait` will unlock `inner`, so
                // the other thread can relock `inner` at 2.3 (and to allow
                // other threads to arrive 0.1).
                //
                // When `wait` finishes, other thread has enqueued a packet,
                // so the purpose of this function has been fulfilled. `wait`
                // will relock `inner` when it returns.
                Ok(self.reader_condition.wait(inner).unwrap())
            }
            Err(TryLockError::Poisoned(e)) => panic!("{}", e),
            Ok(mut lock) => {
                // 2.1. Drop inner so other threads can use it while
                // `read_packet` is blocking.
                drop(inner);

                // Make sure sleeping readers are woken up when we return
                // (Even in case of errors)
                let _notify = NotifyOnDrop(&self.reader_condition);

                // 2.2. Block the thread until a packet is received.
                mode.set_on_reader(lock.get_mut())?;
                let mut fds = Vec::new();
                use std::io::ErrorKind::WouldBlock;
                let packet = lock.read_packet(&mut fds);

                // 2.3. Relock `inner` to enqueue the packet.
                inner = self.inner.lock().unwrap();

                let packet = match packet {
                    Err(e) if e.kind() == WouldBlock => return Ok(inner),
                    Err(e) => return Err(e),
                    Ok(packet) => packet,
                };

                // 2.4. Once `inner` has been relocked, drop the
                // lock on `read`. While inner is locked, other
                // threads cannot arrive 0.1 anyways.
                //
                // `read` cannot unlocked before `inner` is relocked
                // because it could let another thread wait on 2.2
                // for a reply that has been read but not enqueued yet.
                drop(lock);

                // 2.5. Actually enqueue the read packet.
                inner.enqueue_fds(fds);
                inner.enqueue_packet(packet);

                // 2.6. Return the locked `inner` to the caller.
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
}

impl<R: ReadFD, W: WriteFD> RequestConnection for RustConnection<R, W> {
    type Buf = Vec<u8>;

    fn send_request_with_reply<Reply>(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<Cookie<'_, Self, Reply>, ConnectionError>
    where
        Reply: for<'a> TryFrom<&'a [u8], Error = ParseError>,
    {
        Ok(Cookie::new(
            self,
            self.send_request(bufs, fds, ReplyFDKind::ReplyWithoutFDs)?,
        ))
    }

    fn send_request_with_reply_with_fds<Reply>(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<CookieWithFds<'_, Self, Reply>, ConnectionError>
    where
        Reply: for<'a> TryFrom<(&'a [u8], Vec<RawFdContainer>), Error = ParseError>,
    {
        Ok(CookieWithFds::new(
            self,
            self.send_request(bufs, fds, ReplyFDKind::ReplyWithFDs)?,
        ))
    }

    fn send_request_without_reply(
        &self,
        bufs: &[IoSlice<'_>],
        fds: Vec<RawFdContainer>,
    ) -> Result<VoidCookie<'_, Self>, ConnectionError> {
        Ok(VoidCookie::new(
            self,
            self.send_request(bufs, fds, ReplyFDKind::NoReply)?,
        ))
    }

    fn discard_reply(&self, sequence: SequenceNumber, _kind: RequestKind, mode: DiscardMode) {
        self.inner.lock().unwrap().discard_reply(sequence, mode);
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
        let mut write = self.write.lock().unwrap();
        let mut inner = self.inner.lock().unwrap();
        write.flush()?; // Ensure the request is sent
        drop(write);
        loop {
            match inner.poll_for_reply(sequence) {
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
        let mut write = self.write.lock().unwrap();
        let mut inner = self.inner.lock().unwrap();
        if inner.prepare_check_for_reply_or_error(sequence) {
            self.send_sync(&mut *inner, &mut *write)?;
            assert!(!inner.prepare_check_for_reply_or_error(sequence));
        }
        write.flush()?; // Ensure the request is sent
        drop(write);
        loop {
            match inner.poll_check_for_reply_or_error(sequence) {
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
        let mut write = self.write.lock().unwrap();
        let mut inner = self.inner.lock().unwrap();
        write.flush()?; // Ensure the request is sent
        drop(write);
        loop {
            if let Some(reply) = inner.poll_for_reply_or_error(sequence) {
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

    fn parse_error(&self, error: &[u8]) -> Result<crate::protocol::Error, ParseError> {
        let ext_mgr = self.extension_manager.lock().unwrap();
        crate::protocol::Error::parse(error, &*ext_mgr)
    }

    fn parse_event(&self, event: &[u8]) -> Result<crate::protocol::Event, ParseError> {
        let ext_mgr = self.extension_manager.lock().unwrap();
        crate::protocol::Event::parse(event, &*ext_mgr)
    }
}

impl<R: ReadFD, W: WriteFD> Connection for RustConnection<R, W> {
    fn wait_for_raw_event_with_sequence(&self) -> Result<RawEventAndSeqNumber, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        loop {
            if let Some(event) = inner.poll_for_event_with_sequence() {
                return Ok(event);
            }
            inner = self.read_packet_and_enqueue(inner, BlockingMode::Blocking)?;
        }
    }

    fn poll_for_raw_event_with_sequence(
        &self,
    ) -> Result<Option<RawEventAndSeqNumber>, ConnectionError> {
        let mut inner = self.inner.lock().unwrap();
        if let Some(event) = inner.poll_for_event_with_sequence() {
            Ok(Some(event))
        } else {
            inner = self.read_packet_and_enqueue(inner, BlockingMode::NonBlocking)?;
            Ok(inner.poll_for_event_with_sequence())
        }
    }

    fn flush(&self) -> Result<(), ConnectionError> {
        self.write.lock().unwrap().flush()?;
        Ok(())
    }

    fn setup(&self) -> &Setup {
        &self.setup
    }

    fn generate_id(&self) -> Result<u32, ReplyOrIdError> {
        self.id_allocator.lock().unwrap().generate_id(self)
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
    write: &mut impl WriteFD,
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
    write.write_all(&request.serialize(), Vec::new())?;
    write.flush()?;
    Ok(())
}

/// Read a `Setup` from the X11 server.
///
/// If the server sends a `SetupFailed` or `SetupAuthenticate` packet, these will be returned
/// as errors.
fn read_setup(read: &mut impl ReadFD) -> Result<Setup, ConnectError> {
    let mut fds = Vec::new();
    let mut setup = vec![0; 8];
    read.read_exact(&mut setup, &mut fds)?;
    let extra_length = usize::from(u16::from_ne_bytes([setup[6], setup[7]])) * 4;
    // Use `Vec::reserve_exact` because this will be the final
    // length of the vector.
    setup.reserve_exact(extra_length);
    setup.resize(8 + extra_length, 0);
    read.read_exact(&mut setup[8..], &mut fds)?;
    if !fds.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "unexpectedly received FDs in connection setup",
        )
        .into());
    }
    match setup[0] {
        // 0 is SetupFailed
        0 => Err(ConnectError::SetupFailed((&setup[..]).try_into()?)),
        // Success
        1 => Ok((&setup[..]).try_into()?),
        // 2 is SetupAuthenticate
        2 => Err(ConnectError::SetupAuthenticate((&setup[..]).try_into()?)),
        // Uhm... no other cases are defined
        _ => Err(ParseError::ParseError.into()),
    }
}

/// Write a set of buffers on a Writer.
///
/// This is basically `Write::write_all_vectored`, but on stable and for `WriteFD`.
fn write_all_vectored(
    write: &mut impl WriteFD,
    bufs: &[IoSlice<'_>],
    mut fds: Vec<RawFdContainer>,
) -> Result<(), std::io::Error> {
    let mut bufs = bufs;
    while !bufs.is_empty() {
        let mut count = write.write_vectored(bufs, &mut fds)?;
        if count == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                "failed to write anything",
            ));
        }
        while count > 0 {
            if count >= bufs[0].len() {
                count -= bufs[0].len();
            } else {
                let remaining = &bufs[0][count..];
                write.write_all(remaining, fds)?;
                fds = Vec::new();
                count = 0;
            }
            bufs = &bufs[1..];

            // Skip empty slices
            while bufs.first().map(|s| s.len()) == Some(0) {
                bufs = &bufs[1..];
            }
        }
    }
    if !fds.is_empty() {
        write.write_all(&[], fds)?;
    }
    Ok(())
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
    use std::io::{IoSlice, Read, Result, Write};

    use super::{read_setup, write_all_vectored, ReadFD, WriteFD};
    use crate::errors::ConnectError;
    use crate::protocol::xproto::{ImageOrder, Setup, SetupAuthenticate, SetupFailed};
    use crate::utils::RawFdContainer;
    use crate::x11_utils::Serialize;

    struct WriteFDSlice<'a>(&'a mut [u8]);

    impl WriteFD for WriteFDSlice<'_> {
        fn write(&mut self, buf: &[u8], fds: &mut Vec<RawFdContainer>) -> Result<usize> {
            assert!(fds.is_empty());
            self.0.write(buf)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct ReadFDSlice<'a>(&'a [u8]);

    impl ReadFD for ReadFDSlice<'_> {
        fn read(&mut self, buf: &mut [u8], _fd_storage: &mut Vec<RawFdContainer>) -> Result<usize> {
            self.0.read(buf)
        }

        fn set_nonblocking(&mut self, _nonblocking: bool) -> Result<()> {
            unimplemented!()
        }
    }

    fn partial_write_test(request: &[u8], expected_err: &str) {
        let mut written = [0x21; 2];
        let mut output = &mut written[..];
        let request = [IoSlice::new(&request), IoSlice::new(&request)];
        let mut writer = WriteFDSlice(&mut output);
        let error = write_all_vectored(&mut writer, &request, Vec::new()).unwrap_err();
        assert_eq!(expected_err, error.to_string());
    }

    #[test]
    fn partial_write_larger_slice() {
        // This tries to write 4+4 bytes into a buffer of size 2
        partial_write_test(&[0; 4], "failed to write whole buffer");
    }

    #[test]
    fn partial_write_slice_border() {
        // This tries to write 2+2 bytes into a buffer of size 2
        partial_write_test(&[0; 2], "failed to write anything");
    }

    #[test]
    fn full_write_trailing_empty() {
        let mut written = [0; 4];
        let mut output = &mut written[..];
        let (request1, request2) = ([0; 4], [0; 0]);
        let request = [IoSlice::new(&request1), IoSlice::new(&request2)];
        let mut writer = WriteFDSlice(&mut output);
        write_all_vectored(&mut writer, &request, Vec::new()).unwrap();
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
            image_byte_order: ImageOrder::LSBFirst,
            bitmap_format_bit_order: ImageOrder::LSBFirst,
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

        let mut reader = ReadFDSlice(&setup_bytes[..]);
        let read = read_setup(&mut reader);
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

        let mut reader = ReadFDSlice(&setup_bytes[..]);
        match read_setup(&mut reader) {
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

        let mut reader = ReadFDSlice(&setup_bytes[..]);
        match read_setup(&mut reader) {
            Err(ConnectError::SetupAuthenticate(read)) => assert_eq!(setup, read),
            value => panic!("Unexpected value {:?}", value),
        }
    }
}
