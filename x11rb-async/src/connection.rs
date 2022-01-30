use std::convert::TryInto;
use std::io::Result as IoResult;
use std::marker::Unpin;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt as _, AsyncWriteExt as _, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc::{channel, UnboundedReceiver};
use tokio::sync::Mutex;
use x11rb::connection::SequenceNumber;
use x11rb::errors::ConnectionError;
use x11rb::protocol::xproto::{
    GetInputFocusRequest, QueryExtensionReply, QueryExtensionRequest, Setup, GE_GENERIC_EVENT,
};
use x11rb::x11_utils::{ExtensionInformation, ReplyRequest, Request, TryParse, VoidRequest};

use crate::connection_state::ConnectionState;
use crate::cookie::Cookie;
use crate::extension_manager::{ExtState, ExtensionManager};

#[derive(Debug)]
pub struct Connection {
    writer: Mutex<BufWriter<OwnedWriteHalf>>,
    reader: Mutex<UnboundedReceiver<Vec<u8>>>,

    inner: Arc<Mutex<ConnectionState>>,
    extensions: Mutex<ExtensionManager>,

    setup: Setup,
}

impl Connection {
    fn foo_start_reader(reader: OwnedReadHalf, state: Arc<Mutex<ConnectionState>>) {
        tokio::spawn(read_packets(BufReader::new(reader), state));
    }

    async fn request_ext_state(
        &self,
        extension_name: &str,
        writer: &mut BufWriter<OwnedWriteHalf>,
    ) -> Result<ExtState, ConnectionError> {
        let (buf, fds) = QueryExtensionRequest {
            name: extension_name.as_bytes().into(),
        }
        .serialize(0);
        debug_assert!(fds.is_empty(), "QueryExtension does not have any requests");

        let (sender, mut receiver) = channel(1);
        let sender = sender.reserve_owned().await.unwrap();
        {
            // Lock the inner state, but unlock it before trying to send anything
            self.inner.lock().await.sending_reply_request(sender);
        }

        writer.write_all(&buf).await?;

        let reply = receiver.recv().await.expect("The reading task broke?!");
        let reply = QueryExtensionReply::try_parse(&reply)?.0;
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
        writer: &mut BufWriter<OwnedWriteHalf>,
    ) -> Result<SequenceNumber, ConnectionError>
    where
        Req: Request,
    {
        let opcode = match Req::EXTENSION_NAME {
            None => 0,
            Some(ext) => {
                let mut extensions = self.extensions.lock().await;
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
            let mut inner = self.inner.lock().await;
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
        let mut writer = self.writer.lock().await;

        if self.inner.lock().await.need_sync() {
            self.send_request_with_reply_impl(GetInputFocusRequest, &mut *writer)
                .await?;
        }

        self.inner.lock().await.last_sequence_send += 1;
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
        let mut writer = self.writer.lock().await;
        self.send_request_with_reply_impl(request, &mut *writer)
            .await
    }

    async fn send_request_with_reply_impl<Req>(
        &self,
        request: Req,
        writer: &mut BufWriter<OwnedWriteHalf>,
    ) -> Result<Cookie<Req::Reply>, ConnectionError>
    where
        Req: ReplyRequest,
        Req::Reply: TryParse,
    {
        let mut writer = self.writer.lock().await;

        let (sender, receiver) = channel(1);
        let sender = sender.reserve_owned().await.unwrap();
        self.inner.lock().await.sending_reply_request(sender);

        let seqno = self.send_request_impl(request, &mut writer).await?;

        Ok(Cookie::new(receiver))
    }

    pub async fn flush(&self) -> IoResult<()> {
        self.writer.lock().await.flush().await
    }
}

async fn read_packets(mut reader: impl AsyncRead + Unpin, state: Arc<Mutex<ConnectionState>>) {
    loop {
        const MIN_PACKET_LENGTH: usize = 32;
        let mut packet = vec![0; 32];
        reader.read_exact(&mut packet[..]).await.unwrap();

        let extra_length = extra_length(&*packet);
        packet.resize(packet.len() + extra_length, 0);
        reader.read_exact(&mut packet[32..]).await.unwrap();

        state.lock().await.received_packet(packet);
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
