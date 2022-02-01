use crate::async_traits;
use std::future::Future;
use std::io::Result as IoResult;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::pin::Pin;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net;
use tokio::sync::{self, mpsc};

pub struct TcpRead(net::tcp::OwnedReadHalf);

impl async_traits::ReadStream for TcpRead {
    fn read_exact<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> Pin<Box<dyn Future<Output = IoResult<()>> + 'a>> {
        Box::pin(async move {
            self.0.read_exact(buf).await?;
            Ok(())
        })
    }
}

pub struct TcpWrite(net::tcp::OwnedWriteHalf);

impl async_traits::WriteStream for TcpWrite {
    fn write_all<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = IoResult<()>> + 'a>> {
        Box::pin(self.0.write_all(buf))
    }

    fn flush(&mut self) -> Pin<Box<dyn Future<Output = IoResult<()>> + '_>> {
        Box::pin(self.0.flush())
    }
}

pub struct TcpStream;

impl async_traits::TcpStream for TcpStream {
    type Read = TcpRead;
    type Write = TcpWrite;

    fn connect(
        host: &str,
        port: u16,
    ) -> Pin<Box<dyn Future<Output = IoResult<(Self::Read, Self::Write)>> + '_>> {
        Box::pin(async move {
            let stream = net::TcpStream::connect((host, port)).await?;
            let (read, write) = stream.into_split();
            Ok((TcpRead(read), TcpWrite(write)))
        })
    }
}

pub struct Mutex<T>(sync::Mutex<T>);

impl<T> async_traits::Mutex<T> for Mutex<T> {
    fn new(t: T) -> Self {
        Mutex(sync::Mutex::new(t))
    }

    fn lock(&self) -> Pin<Box<dyn Future<Output = Box<dyn DerefMut<Target = T> + '_>> + '_>> {
        Box::pin(async move {
            let guard = self.0.lock().await;
            Box::new(guard) as Box<dyn DerefMut<Target = T>>
        })
    }
}

pub struct OneshotSender<T>(sync::oneshot::Sender<T>);

impl<T> async_traits::OneshotSender<T> for OneshotSender<T> {
    fn send(self, message: T) -> Pin<Box<Future<Output = Result<(), async_traits::SendError>>>> {
        let result = self.0.send(message).map_err(|_| async_traits::SendError);
        Box::pin(async move { result })
    }
}

pub struct OneshotReceiver<T>(sync::oneshot::Receiver<T>);

impl<T: 'static> async_traits::OneshotReceiver<T> for OneshotReceiver<T> {
    fn recv(self) -> Pin<Box<dyn Future<Output = Option<T>>>> {
        Box::pin(async move { self.0.await.ok() })
    }
}

pub struct UnboundedSender<T>(mpsc::UnboundedSender<T>);

impl<T> async_traits::ChannelSender<T> for UnboundedSender<T> {
    fn send(
        &self,
        message: T,
    ) -> Pin<Box<Future<Output = Result<(), async_traits::SendError>> + '_>> {
        let result = self.0.send(message).map_err(|_| async_traits::SendError);
        Box::pin(async move { result })
    }
}

pub struct UnboundedReceiver<T>(mpsc::UnboundedReceiver<T>);

impl<T> async_traits::ChannelReceiver<T> for UnboundedReceiver<T> {
    fn recv(&mut self) -> Pin<Box<dyn Future<Output = Option<T>> + '_>> {
        Box::pin(self.0.recv())
    }
}

pub struct Channel<T>(PhantomData<T>);

impl<T: 'static> async_traits::Channel<T> for Channel<T> {
    type OneshotSender = OneshotSender<T>;
    type OneshotReceiver = OneshotReceiver<T>;

    type UnboundedSender = UnboundedSender<T>;
    type UnboundedReceiver = UnboundedReceiver<T>;

    fn new_oneshot() -> (Self::OneshotSender, Self::OneshotReceiver) {
        let (send, recv) = sync::oneshot::channel();
        (OneshotSender(send), OneshotReceiver(recv))
    }

    fn new_unbounded() -> (Self::UnboundedSender, Self::UnboundedReceiver) {
        let (send, recv) = mpsc::unbounded_channel();
        (UnboundedSender(send), UnboundedReceiver(recv))
    }
}
