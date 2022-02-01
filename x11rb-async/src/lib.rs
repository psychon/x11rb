pub mod reexports {
    pub use x11rb;
}

mod async_std;
mod async_traits;
mod connection;
mod connection_state;
mod cookie;
mod extension_manager;
mod tokio;

pub use connection::Connection;

mod test_experiment {
    use crate::async_traits::{Channel as _, ChannelReceiver as _, ChannelSender as _, Mutex as _};

    async fn tokio_foo() {
        use crate::tokio::*;

        let mutex = Mutex::new(42);
        let value: u8 = *mutex.lock().await.deref();
        println!("{:?}", value);

        let (send, mut recv) = Channel::new_unbounded();
        send.send(42u8);
        let value = recv.recv().await;
        let value: u8 = value.unwrap();
        println!("{:?}", value);
    }

    async fn async_std_foo() {
        use crate::async_std::*;

        let mutex = Mutex::new(42);
        let value: u8 = *mutex.lock().await.deref();
        println!("{:?}", value);

        let (send, mut recv) = Channel::new_unbounded();
        send.send(42u8);
        let value = recv.recv().await;
        let value: u8 = value.unwrap();
        println!("{:?}", value);
    }
}
