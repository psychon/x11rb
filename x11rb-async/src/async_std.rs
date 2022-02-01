use crate::async_traits;
use async_std::channel;
use async_std::io::{ReadExt as _, WriteExt as _};
use async_std::net;
use async_std::sync;
use std::future::Future;
use std::io::Result as IoResult;
use std::marker::PhantomData;
use std::ops::DerefMut;
use std::pin::Pin;

pub struct TcpRead(net::TcpStream);

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

pub struct TcpWrite(net::TcpStream);

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
            let stream2 = stream.clone();
            Ok((TcpRead(stream), TcpWrite(stream2)))
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

pub struct Sender<T>(channel::Sender<T>);

impl<T> async_traits::ChannelSender<T> for Sender<T> {
    fn send(
        &self,
        message: T,
    ) -> Pin<Box<dyn Future<Output = Result<(), async_traits::SendError>> + '_>> {
        Box::pin(async move {
            self.0
                .send(message)
                .await
                .map_err(|_| async_traits::SendError)
        })
    }
}

impl<T: 'static> async_traits::OneshotSender<T> for Sender<T> {
    fn send(
        self,
        message: T,
    ) -> Pin<Box<dyn Future<Output = Result<(), async_traits::SendError>>>> {
        Box::pin(async move {
            self.0
                .send(message)
                .await
                .map_err(|_| async_traits::SendError)
        })
    }
}

pub struct Receiver<T>(channel::Receiver<T>);

impl<T> async_traits::ChannelReceiver<T> for Receiver<T> {
    fn recv(&mut self) -> Pin<Box<dyn Future<Output = Option<T>> + '_>> {
        Box::pin(async move { self.0.recv().await.ok() })
    }
}

impl<T: 'static> async_traits::OneshotReceiver<T> for Receiver<T> {
    fn recv(self) -> Pin<Box<dyn Future<Output = Option<T>>>> {
        Box::pin(async move { self.0.recv().await.ok() })
    }
}

pub struct Channel<T>(PhantomData<T>);

impl<T: 'static> async_traits::Channel<T> for Channel<T> {
    type OneshotSender = Sender<T>;
    type OneshotReceiver = Receiver<T>;

    type UnboundedSender = Sender<T>;
    type UnboundedReceiver = Receiver<T>;

    fn new_oneshot() -> (Self::OneshotSender, Self::OneshotReceiver) {
        let (send, recv) = channel::bounded(1);
        (Sender(send), Receiver(recv))
    }

    fn new_unbounded() -> (Self::UnboundedSender, Self::UnboundedReceiver) {
        let (send, recv) = channel::unbounded();
        (Sender(send), Receiver(recv))
    }
}
