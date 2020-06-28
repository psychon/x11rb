use std::io::Result as IOResult;
use futures_io::{AsyncRead, AsyncWrite};

#[derive(Debug, Default)]
pub struct Connection {
}

impl Connection {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn forward_client(&self, client: impl AsyncRead, server: impl AsyncWrite) -> IOResult<()> {
        Ok(())
    }

    pub async fn forward_server(&self, server: impl AsyncRead, client: impl AsyncWrite) -> IOResult<()> {
        Ok(())
    }
}
