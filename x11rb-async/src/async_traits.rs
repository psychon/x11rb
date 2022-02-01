use std::future::Future;
use std::io::Result as IoResult;
use std::ops::DerefMut;
use std::pin::Pin;

pub trait ReadStream {
    fn read_exact<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> Pin<Box<dyn Future<Output = IoResult<()>> + 'a>>;
}

pub trait WriteStream {
    fn write_all<'a>(
        &'a mut self,
        buf: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = IoResult<()>> + 'a>>;

    fn flush(&mut self) -> Pin<Box<dyn Future<Output = IoResult<()>> + '_>>;
}

pub trait TcpStream {
    type Read: ReadStream;
    type Write: WriteStream;

    fn connect(
        host: &str,
        port: u16,
    ) -> Pin<Box<dyn Future<Output = IoResult<(Self::Read, Self::Write)>> + '_>>;
}

pub trait Mutex<T> {
    fn new(t: T) -> Self;

    fn lock(&self) -> Pin<Box<dyn Future<Output = Box<dyn DerefMut<Target = T> + '_>> + '_>>;
}

pub struct SendError;

pub trait ChannelSender<T> {
    fn send(&self, message: T) -> Pin<Box<dyn Future<Output = Result<(), SendError>> + '_>>;
}

pub trait ChannelReceiver<T> {
    fn recv(&mut self) -> Pin<Box<dyn Future<Output = Option<T>> + '_>>;
}

pub trait OneshotSender<T> {
    fn send(self, message: T) -> Pin<Box<dyn Future<Output = Result<(), SendError>>>>;
}

pub trait OneshotReceiver<T: 'static> {
    fn recv(self) -> Pin<Box<dyn Future<Output = Option<T>>>>;
}

pub trait Channel<T: 'static> {
    type OneshotSender: OneshotSender<T>;
    type OneshotReceiver: OneshotReceiver<T>;

    type UnboundedSender: ChannelSender<T>;
    type UnboundedReceiver: ChannelReceiver<T>;

    fn new_oneshot() -> (Self::OneshotSender, Self::OneshotReceiver);
    fn new_unbounded() -> (Self::UnboundedSender, Self::UnboundedReceiver);
}
