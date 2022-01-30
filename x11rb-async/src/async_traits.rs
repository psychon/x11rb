#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Runtime {
    Tokio,
    AsyncStd,
}

#[derive(Debug)]
enum Sender<T> {
    ToDo(T),
}

#[derive(Debug)]
enum Receiver<T> {
    ToDo(T),
}

impl Runtime {
    fn new_unbounded_channel<T>(&self) -> (Sender<T>, Receiver<T>) {
        todo!()
    }
}

/*
pub trait ReadStream {}

pub trait WriteStream {}

pub trait Mutex {}

pub struct SendError;

pub trait ChannelSender<T> {
    async fn send(&self) -> Result<(), SendError>;
}

pub trait ChannelReceiver<T> {
    async fn recv(&mut self) -> Option<T>;
}

pub trait Channel<T> {
    type Sender: ChannelSender<T>;
    type Receiver: ChannelReceiver<T>;

    fn new_unbounded() -> (Self::Sender, Self::Receiver);
}
*/
