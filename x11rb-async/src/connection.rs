use crate::asyncchannel::{oneshot, unbounded, UnboundedReceiver};
use crate::asyncmutex::AsyncMutex;
use crate::asynctraits::{ReadStream, StreamFactory, WriteStream};
use crate::connection_state::ConnectionState;
use crate::cookie::Cookie;
use crate::extension_manager::{ExtState, ExtensionManager};
use std::convert::TryInto;
use std::io::Result as IoResult;
use std::sync::Arc;
use x11rb::connection::SequenceNumber;
use x11rb::errors::ConnectionError;
use x11rb::protocol::xproto::{
    GetInputFocusRequest, QueryExtensionReply, QueryExtensionRequest, Setup, SetupRequest,
    GE_GENERIC_EVENT,
};
use x11rb::x11_utils::{
    ExtensionInformation, ReplyRequest, Request, Serialize, TryParse, VoidRequest,
};

pub struct Connection<Stream: StreamFactory<(String, u16)>> {
    writer: AsyncMutex<Stream::Write>,
    reader: AsyncMutex<UnboundedReceiver<Vec<u8>>>,

    inner: Arc<AsyncMutex<ConnectionState>>,
    extensions: AsyncMutex<ExtensionManager>,

    setup: Setup,
}

impl<Stream: StreamFactory<(String, u16)>> Connection<Stream> {
    pub async fn connect(
        display_number: u16,
    ) -> Result<(Self, impl std::future::Future<Output = ()> + Send), ConnectionError> {
        let (mut read, mut write) =
            Stream::connect(("127.0.0.1".to_string(), 6000 + display_number)).await?;

        send_setup(&mut write).await?;
        let setup = read_setup(&mut read).await?;

        let (write_events, read_events) = unbounded();

        let inner = Arc::new(AsyncMutex::new(ConnectionState::new(write_events)));
        let inner2 = Arc::clone(&inner);
        let reader = read_packets(read, inner2);

        let connection = Self {
            writer: AsyncMutex::new(write),
            reader: AsyncMutex::new(read_events),
            inner,
            extensions: Default::default(),
            setup,
        };

        Ok((connection, reader))
    }

    pub fn setup(&self) -> &Setup {
        &self.setup
    }

    async fn request_ext_state(
        &self,
        extension_name: &str,
        writer: &mut Stream::Write,
    ) -> Result<ExtState, ConnectionError> {
        let (buf, fds) = QueryExtensionRequest {
            name: extension_name.as_bytes().into(),
        }
        .serialize(0);
        debug_assert!(fds.is_empty(), "QueryExtension does not have any FDs");

        let (sender, receiver) = oneshot();
        {
            // Lock the inner state, but unlock it before trying to send anything
            self.inner
                .lock()
                .await
                .expect("TODO pass error to caller")
                .sending_reply_request(sender);
        }

        writer.write_all(&buf).await?;
        writer.flush().await?;

        let reply = QueryExtensionReply::try_parse(&receiver.await)?.0;
        let state = if reply.present {
            ExtState::Present(ExtensionInformation {
                major_opcode: reply.major_opcode,
                first_event: reply.first_event,
                first_error: reply.first_error,
            })
        } else {
            ExtState::Missing
        };
        Ok(state)
    }

    async fn send_request_impl<Req>(
        &self,
        request: Req,
        writer: &mut Stream::Write,
    ) -> Result<SequenceNumber, ConnectionError>
    where
        Req: Request,
    {
        let opcode = match Req::EXTENSION_NAME {
            None => 0,
            Some(ext) => {
                let mut extensions = self
                    .extensions
                    .lock()
                    .await
                    .expect("TODO pass error to caller");
                let state = match extensions.extension_information(ext) {
                    Some(info) => info,
                    None => {
                        // Request the info
                        let state = self.request_ext_state(ext, writer).await?;
                        extensions.insert(ext, state);
                        state
                    }
                };
                match state {
                    ExtState::Missing => return Err(ConnectionError::UnsupportedExtension.into()),
                    ExtState::Present(info) => info.major_opcode,
                }
            }
        };
        let (buf, fds) = request.serialize(opcode);
        assert!(fds.is_empty(), "TODO: Implement FD passing");

        let seqno = {
            // Lock the inner state, but unlock it before trying to send anything
            let mut inner = self.inner.lock().await.expect("TODO pass error to caller");
            inner.last_sequence_send += 1;
            inner.last_sequence_send
        };
        writer.write_all(&buf).await?;

        Ok(seqno)
    }

    pub async fn send_request_without_reply<Req>(&self, request: Req) -> Result<(), ConnectionError>
    where
        Req: VoidRequest,
    {
        let mut writer = self.writer.lock().await.expect("TODO pass error to caller");

        if self
            .inner
            .lock()
            .await
            .expect("TODO pass error to caller")
            .need_sync()
        {
            self.send_request_with_reply_impl(GetInputFocusRequest, &mut *writer)
                .await?;
        }

        self.inner
            .lock()
            .await
            .expect("TODO pass error to caller")
            .last_sequence_send += 1;
        self.send_request_impl(request, &mut writer).await?;

        Ok(())
    }

    pub async fn send_request_with_reply<Req>(
        &self,
        request: Req,
    ) -> Result<Cookie<Req::Reply>, ConnectionError>
    where
        Req: ReplyRequest,
        Req::Reply: TryParse,
    {
        let mut writer = self.writer.lock().await.expect("TODO pass error to caller");
        self.send_request_with_reply_impl(request, &mut *writer)
            .await
    }

    async fn send_request_with_reply_impl<Req>(
        &self,
        request: Req,
        writer: &mut Stream::Write,
    ) -> Result<Cookie<Req::Reply>, ConnectionError>
    where
        Req: ReplyRequest,
        Req::Reply: TryParse,
    {
        let mut writer = self.writer.lock().await.expect("TODO pass error to caller");

        let (sender, receiver) = oneshot();
        self.inner
            .lock()
            .await
            .expect("TODO pass error to caller")
            .sending_reply_request(sender);

        let seqno = self.send_request_impl(request, &mut writer).await?;

        Ok(Cookie::new(receiver))
    }

    pub async fn flush(&self) -> IoResult<()> {
        self.writer
            .lock()
            .await
            .expect("TODO pass error to caller")
            .flush()
            .await
    }
}

async fn send_setup(write: &mut impl WriteStream) -> IoResult<()> {
    let setup = SetupRequest {
        byte_order: 0x6c, // TODO: Proper value
        protocol_major_version: 11,
        protocol_minor_version: 0,
        authorization_protocol_name: vec![],
        authorization_protocol_data: vec![],
    }
    .serialize();
    write.write_all(&setup[..]).await?;
    write.flush().await?;
    Ok(())
}

async fn read_setup(read: &mut impl ReadStream) -> Result<Setup, ConnectionError> {
    let mut setup = vec![0; 8];
    read.read_exact(&mut setup).await?;
    let extra_length = usize::from(u16::from_ne_bytes([setup[6], setup[7]])) * 4;
    setup.reserve_exact(extra_length);
    setup.resize(8 + extra_length, 0);
    read.read_exact(&mut setup[8..]).await?;

    // TODO: Handle failures
    assert_eq!(1, setup[0]);
    Ok(Setup::try_parse(&setup[..])?.0)
}

async fn read_packets(mut reader: impl ReadStream, state: Arc<AsyncMutex<ConnectionState>>) {
    loop {
        const MIN_PACKET_LENGTH: usize = 32;
        let mut packet = vec![0; 32];
        reader.read_exact(&mut packet[..]).await.unwrap();

        let extra_length = extra_length(&*packet);
        packet.resize(packet.len() + extra_length, 0);
        reader.read_exact(&mut packet[32..]).await.unwrap();

        state
            .lock()
            .await
            .expect("TODO pass error to caller")
            .received_packet(packet);
    }
}

fn extra_length(packet: &[u8]) -> usize {
    const REPLY: u8 = 1;

    if packet[0] == REPLY || packet[0] & 0x7f == GE_GENERIC_EVENT {
        let length = packet[4..8].try_into().unwrap();
        let length = u32::from_ne_bytes(length) as usize;
        4 * length
    } else {
        0
    }
}
