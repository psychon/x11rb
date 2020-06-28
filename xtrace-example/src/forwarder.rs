use std::io::Result as IOResult;
use futures_io::{AsyncRead, AsyncWrite};
use futures_util::{AsyncReadExt, AsyncWriteExt};

/// Forward data between a reader and a writer, calling a callback on the data.
///
/// This function copies all data from `read` to `write`. At each step, a callback is invoked with
/// the data of the current packet. If `None` is returned, then the packet is complete and can be
/// forwarded to the other end. If `Some(length)` is returned, then the packet is incomplete and at
/// least `length` more bytes are needed.
pub async fn forward_with_callback<F>(
    mut read: impl AsyncRead + Unpin,
    mut write: impl AsyncWrite + Unpin,
    callback: F
) -> IOResult<()>
where
    F: Fn(&[u8]) -> Option<usize>
{
    let mut buffer = Vec::new();
    loop {
        // Ask the callback for an estimation of the size of the packet
        match callback(&buffer) {
            None => {
                // We have a full packet, forward it
                write.write_all(&buffer).await?;
                buffer.clear();
            }
            Some(extra_length) => {
                // Need to read more
                assert!(extra_length > 0);
                let old_len = buffer.len();
                buffer.resize_with(old_len + extra_length, Default::default);
                read.read_exact(&mut buffer[old_len..]).await?;
            }
        }
    }
}
