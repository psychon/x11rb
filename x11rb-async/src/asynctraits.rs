use std::future::Future;
use std::io::Result as IoResult;
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

pub trait StreamFactory<ConnectArgument> {
    type Read: ReadStream;
    type Write: WriteStream;

    fn connect(
        target: ConnectArgument,
    ) -> Pin<Box<dyn Future<Output = IoResult<(Self::Read, Self::Write)>>>>;
}
