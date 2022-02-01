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

pub struct Sender<T>(mpsc::UnboundedSender<T>);

impl<T> async_traits::ChannelSender<T> for Sender<T> {
    fn send(&self, message: T) -> Result<(), async_traits::SendError> {
        self.0.send(message).map_err(|_| async_traits::SendError)
    }
}

pub struct Receiver<T>(mpsc::UnboundedReceiver<T>);

impl<T> async_traits::ChannelReceiver<T> for Receiver<T> {
    fn recv(&mut self) -> Pin<Box<dyn Future<Output = Option<T>> + '_>> {
        Box::pin(self.0.recv())
    }
}

pub struct Channel<T>(PhantomData<T>);

impl<T> async_traits::Channel<T> for Channel<T> {
    type Sender = Sender<T>;
    type Receiver = Receiver<T>;

    fn new_unbounded() -> (Self::Sender, Self::Receiver) {
        let (send, recv) = mpsc::unbounded_channel();
        (Sender(send), Receiver(recv))
    }
}
