use crate::asyncchannel::OneshotReceiver;
use std::marker::PhantomData;
use x11rb::errors::ReplyError;
use x11rb::x11_utils::{TryParse, X11Error};

#[derive(Debug)]
pub struct Cookie<Reply: TryParse>(OneshotReceiver<Vec<u8>>, PhantomData<Reply>);

impl<Reply: TryParse> Cookie<Reply> {
    pub(crate) fn new(recv: OneshotReceiver<Vec<u8>>) -> Self {
        Self(recv, PhantomData)
    }

    pub async fn raw_reply(self) -> Vec<u8> {
        self.0.await
    }

    pub async fn reply(self) -> Result<Reply, ReplyError> {
        let raw = self.raw_reply().await;
        if raw[0] == 0 {
            // This is an error
            // TODO: Get access to a extension info thingie
            Err(ReplyError::X11Error(X11Error::try_parse(&raw, todo!())?))
        } else {
            // This must be the reply
            Ok(Reply::try_parse(&raw[..])?.0)
        }
    }
}
