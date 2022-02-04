use std::future::Future;
use std::io::Result as IoResult;
use std::pin::Pin;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _, BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use x11rb::protocol::xproto::GetInputFocusRequest;
use x11rb_async::asynctraits::{ReadStream, StreamFactory, WriteStream};
use x11rb_async::Connection;

struct TokioRead(BufReader<OwnedReadHalf>);

impl ReadStream for TokioRead {
    fn read_exact<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> Pin<Box<dyn Future<Output = IoResult<()>> + 'a + Send>> {
        Box::pin(async move {
            self.0.read_exact(buf).await?;
            Ok(())
        })
    }
}

struct TokioWrite(BufWriter<OwnedWriteHalf>);

impl WriteStream for TokioWrite {
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

struct TokioStreamFactory();

impl StreamFactory<(String, u16)> for TokioStreamFactory {
    type Read = TokioRead;
    type Write = TokioWrite;

    fn connect(
        (host, port): (String, u16),
    ) -> Pin<Box<dyn Future<Output = IoResult<(Self::Read, Self::Write)>>>> {
        Box::pin(async move {
            let (read, write) = TcpStream::connect((host, port)).await?.into_split();
            Ok((
                TokioRead(BufReader::new(read)),
                TokioWrite(BufWriter::new(write)),
            ))
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (conn, reader) = Connection::<TokioStreamFactory>::connect(4).await?;
    tokio::spawn(reader);
    println!("Root window: {:?}", conn.setup().roots[0].root);

    let request = GetInputFocusRequest;
    let reply = conn.send_request_with_reply(request).await?.reply().await;
    println!("Reply: {:?}", reply);

    Ok(())
}
