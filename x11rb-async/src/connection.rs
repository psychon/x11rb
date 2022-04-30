use async_lock::{Mutex, MutexGuard};
use futures_util::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::sync::Arc;

use x11rb_protocol::{
    connect::Connect,
    connection::{Connection as ProtocolConnectionInner, ReplyFdKind},
    errors::ConnectError,
    id_allocator::IdAllocator,
    packet_reader::PacketReader,
    protocol::{
        xproto::{QueryExtensionReply, QueryExtensionRequest, Setup, GET_INPUT_FOCUS_REQUEST},
        Event,
    },
    x11_utils::{ReplyRequest, Request, TryParse, VoidRequest},
    SequenceNumber,
};

use crate::extension_manager::{ExtensionManager, ExtensionResult};
use crate::Condvar;

pub struct Connection<Writer: AsyncWrite + Unpin> {
    inner: Arc<Mutex<ConnectionInner>>,
    packet_condition: Arc<Condvar>,

    writer: Mutex<Option<Writer>>,

    id_allocator: Mutex<IdAllocator>,
    extension_manager: Mutex<ExtensionManager>,
    setup: Setup,
}

impl<Writer: AsyncWrite + Unpin> Connection<Writer> {
    pub async fn connect_streams_with_auth_data<R>(
        mut read: R,
        mut write: Writer,
        protocol_name: Vec<u8>,
        protocol_data: Vec<u8>,
    ) -> Result<(Self, impl std::future::Future<Output = ()> + Send), ConnectError>
    where
        R: AsyncRead + Unpin + Send,
    {
        // TODO: Switch to Connect::new(), or even better: Make that the caller's problem
        let (mut connect, setup_request) =
            Connect::with_authorization(protocol_name, protocol_data);
        write.write_all(&setup_request).await?;
        write.flush().await?;

        loop {
            let buffer = connect.buffer();
            let buffer_len = buffer.len();
            read.read_exact(buffer).await?;
            if connect.advance(buffer_len) {
                break;
            }
        }
        let setup = connect.into_setup()?;

        let id_allocator = Mutex::new(IdAllocator::new(
            setup.resource_id_base,
            setup.resource_id_mask,
        )?);

        let inner = Arc::new(Mutex::new(ConnectionInner {
            inner: ProtocolConnectionInner::new(),
            io_error: None,
        }));
        let packet_condition = Arc::new(Condvar::new());

        let inner2 = Arc::clone(&inner);
        let packet_condition2 = Arc::clone(&packet_condition);
        let reader = read_packets(read, inner2, packet_condition2);

        let connection = Connection {
            inner,
            packet_condition,
            extension_manager: Default::default(),
            id_allocator,
            setup,
            writer: Mutex::new(Some(write)),
        };

        Ok((connection, reader))
    }

    pub fn setup(&self) -> &Setup {
        &self.setup
    }

    pub async fn generate_id(&self) -> Option<u32> {
        let mut id_allocator = self.id_allocator.lock().await;
        // TODO: Use XC-MISC to get some free IDs
        // TODO: Change return type to some Result<>
        id_allocator.generate_id()
    }

    async fn get_writer(&self) -> Result<WriterWrapper<'_, Writer>, ConnectError> {
        WriterWrapper::new(self.writer.lock().await)
    }

    async fn get_major_opcode<Req: Request>(&self) -> Result<u8, ConnectError> {
        let opcode = match Req::EXTENSION_NAME {
            None => 0,
            Some(ext) => {
                let mut ext_mgr = self.extension_manager.lock().await;
                loop {
                    match ext_mgr.extension_information(ext) {
                        ExtensionResult::Present(info) => break info.major_opcode,
                        // TODO: Proper error code: UnsupportedExtension(ext)
                        ExtensionResult::Missing => return Err(ConnectError::UnknownError),
                        ExtensionResult::NotYetChecked => {
                            let request = QueryExtensionRequest {
                                name: ext.as_bytes().into(),
                            };
                            let (buf, _) =
                                <QueryExtensionRequest as Request>::serialize(request, 0);
                            let seqno = self
                                .send_request_impl(buf, ReplyFdKind::ReplyWithoutFDs)
                                .await?;

                            // TODO: Flush

                            let mut inner = self.inner.lock().await;
                            let reply = loop {
                                if let Some(reply) = inner.inner.poll_for_reply_or_error(seqno) {
                                    break reply.0;
                                }
                                inner = self.packet_condition.wait(inner).await;
                            };
                            let reply = QueryExtensionReply::try_parse(reply.as_ref())?.0;
                            ext_mgr.insert_if_unknown(ext, &reply);
                        }
                    }
                }
            }
        };
        Ok(opcode)
    }

    // TODO: Proper error enum / return type
    async fn serialize_request<Req: Request>(&self, request: Req) -> Result<Vec<u8>, ConnectError> {
        let (buf, fds) = request.serialize(self.get_major_opcode::<Req>().await?);
        assert!(fds.is_empty(), "TODO: Implement FD passing");
        Ok(buf)
    }

    // TODO: Proper error enum / return type
    async fn send_sync(
        &self,
        inner: &mut MutexGuard<'_, ConnectionInner>,
    ) -> Result<(), ConnectError> {
        let length = 1u16.to_ne_bytes();
        let request = [
            GET_INPUT_FOCUS_REQUEST,
            0, /* pad */
            length[0],
            length[1],
        ];

        inner
            .inner
            .send_request(ReplyFdKind::ReplyWithoutFDs)
            .expect("Sending a request with a response should never be blocked by syncs");

        let mut writer = self.get_writer().await?;
        writer.write_all(&request).await?;
        writer.unlock();
        Ok(())
    }

    // TODO: Proper error enum / return type
    async fn send_request_impl(
        &self,
        request: Vec<u8>,
        kind: ReplyFdKind,
    ) -> Result<SequenceNumber, ConnectError> {
        let mut inner = self.inner.lock().await;
        let seqno = loop {
            match inner.inner.send_request(kind) {
                Some(seqno) => break seqno,
                None => {
                    self.send_sync(&mut inner).await?;
                }
            }
        };

        let mut writer = self.get_writer().await?;
        drop(inner);
        writer.write_all(&request).await?;

        writer.unlock();
        Ok(seqno)
    }

    // TODO: ConnectError -> ConnectionError
    pub async fn send_request_with_reply<Req>(
        &self,
        request: Req,
    ) -> Result<Cookie<Req::Reply>, ConnectError>
    where
        Req: ReplyRequest,
        Req::Reply: TryParse,
    {
        let buf = self.serialize_request(request).await?;
        let seqno = self
            .send_request_impl(buf, ReplyFdKind::ReplyWithoutFDs)
            .await?;
        Ok(Cookie::new(
            Arc::clone(&self.inner),
            Arc::clone(&self.packet_condition),
            seqno,
        ))
    }

    // TODO: ConnectError -> ConnectionError
    pub async fn send_request_without_reply<Req>(&self, request: Req) -> Result<(), ConnectError>
    where
        Req: VoidRequest,
    {
        let buf = self.serialize_request(request).await?;
        self.send_request_impl(buf, ReplyFdKind::NoReply).await?;
        Ok(())
    }

    pub async fn wait_for_raw_event(&self) -> Result<Vec<u8>, ConnectError> {
        let mut guard = self.inner.lock().await;
        loop {
            if let Some((event, _)) = guard.inner.poll_for_event_with_sequence() {
                break Ok(event);
            }
            guard = self.packet_condition.wait(guard).await;
        }
    }

    pub async fn wait_for_event(&self) -> Result<Event, ConnectError> {
        let event = self.wait_for_raw_event().await?;
        let ext_mgr = self.extension_manager.lock().await;
        Ok(Event::parse(event.as_ref(), &*ext_mgr)?)
    }
}

struct ConnectionInner {
    inner: ProtocolConnectionInner,
    // TODO: Do something with this error when it is set
    io_error: Option<std::io::Error>,
}

pub struct Cookie<Reply: TryParse> {
    inner: Arc<Mutex<ConnectionInner>>,
    packet_condition: Arc<Condvar>,
    seqno: SequenceNumber,
    _phantom: PhantomData<Reply>,
}

impl<Reply: TryParse> Cookie<Reply> {
    fn new(
        inner: Arc<Mutex<ConnectionInner>>,
        packet_condition: Arc<Condvar>,
        seqno: SequenceNumber,
    ) -> Self {
        Self {
            inner,
            packet_condition,
            seqno,
            _phantom: PhantomData,
        }
    }

    pub async fn raw_reply(self) -> Result<Vec<u8>, ConnectError> {
        // TODO: Flush if the request was not yet sent
        let mut inner = self.inner.lock().await;
        loop {
            if let Some(reply) = inner.inner.poll_for_reply_or_error(self.seqno) {
                // TODO: Check for and handle X11 errors
                break Ok(reply.0);
            }
            inner = self.packet_condition.wait(inner).await;
        }
    }

    pub async fn reply(self) -> Result<Reply, ConnectError> {
        Ok(Reply::try_parse(self.raw_reply().await?.as_ref())?.0)
    }
}

struct WriterWrapper<'conn, Writer: AsyncWrite + Unpin>(Option<MutexGuard<'conn, Option<Writer>>>);

impl<'conn, Writer: AsyncWrite + Unpin> WriterWrapper<'conn, Writer> {
    fn new(guard: MutexGuard<'conn, Option<Writer>>) -> Result<Self, ConnectError> {
        if guard.is_none() {
            // TODO: Better error
            return Err(ConnectError::UnknownError);
        }
        Ok(Self(Some(guard)))
    }

    async fn write_all(&mut self, bytes: &[u8]) -> Result<(), std::io::Error> {
        self.0
            .as_mut()
            .unwrap()
            .deref_mut()
            .as_mut()
            .unwrap()
            .write_all(bytes)
            .await
    }

    fn unlock(mut self) {
        self.0.take();
    }
}

impl<'conn, Writer: AsyncWrite + Unpin> Drop for WriterWrapper<'conn, Writer> {
    fn drop(&mut self) {
        if let Some(mut guard) = self.0.take() {
            *guard = None;
        }
    }
}

async fn read_packets(
    reader: impl AsyncRead + Unpin,
    state: Arc<Mutex<ConnectionInner>>,
    packet_condition: Arc<Condvar>,
) {
    async fn do_read_packets(
        mut read: impl AsyncRead + Unpin,
        state: &Arc<Mutex<ConnectionInner>>,
        packet_condition: Arc<Condvar>,
    ) -> Result<(), std::io::Error> {
        let mut reader = PacketReader::new();
        loop {
            let buffer = reader.buffer();
            let buffer_len = buffer.len();
            read.read_exact(buffer).await?;
            if let Some(packet) = reader.advance(buffer_len) {
                state.lock().await.inner.enqueue_packet(packet);
                packet_condition.notify_all();
            }
        }
    }

    if let Err(err) = do_read_packets(reader, &state, packet_condition).await {
        state.lock().await.io_error = Some(err);
    }
}
