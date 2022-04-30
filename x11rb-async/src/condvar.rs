use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_lock::MutexGuard;

use event_listener::{Event, EventListener};

/// Async condition variable
#[derive(Debug, Default)]
pub struct Condvar(Event);

impl Condvar {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn wait<'g, T>(&self, guard: MutexGuard<'g, T>) -> MutexGuard<'g, T> {
        let source = MutexGuard::source(&guard);
        WaitOnce(self.0.listen(), Some(guard)).await;
        source.lock().await
    }

    pub fn notify_all(&self) {
        // Wake all waiting futures
        self.0.notify_additional(usize::MAX);
    }
}

// Helper struct that drops the mutex guard *after* we registered for notification
struct WaitOnce<'g, T>(EventListener, Option<MutexGuard<'g, T>>);

impl<T> Future for WaitOnce<'_, T> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let result = Pin::new(&mut self.0).poll(cx);
        self.1.take();
        result
    }
}

#[cfg(test)]
mod test {
    use async_lock::Mutex;
    use smol::Executor;
    use std::sync::Arc;

    use super::Condvar;

    #[test]
    fn notify_does_nothing_without_waiter() {
        Condvar::new().notify_all();
    }

    #[test]
    fn wakes_waiter() {
        let ex = Executor::new();
        let condvar = Arc::new(Condvar::new());
        let condvar2 = Arc::clone(&condvar);

        let task = ex.spawn(async move {
            let mutex = Mutex::new(42);

            let mut guard = mutex.lock().await;
            assert_eq!(42, *guard);
            *guard = 100;

            let guard = condvar2.wait(guard).await;
            assert_eq!(100, *guard);
        });

        // Make sure the task is waiting
        ex.try_tick();

        // Wake the task up and wait for it dto be done
        condvar.notify_all();
        smol::block_on(ex.run(task));
    }

    #[test]
    fn allows_multiple_users() {
        // In this test, a variable is incremented from 0 to 1 by five tasks, each one doing every
        // fifth step

        const TARGET: u32 = 100;
        const WORKER_COUNT: u32 = 5;

        let ex = Executor::new();
        let mutex = Arc::new(Mutex::new(0));
        let condvar = Arc::new(Condvar::new());
        for index in 0..WORKER_COUNT {
            let mutex = Arc::clone(&mutex);
            let condvar = Arc::clone(&condvar);
            let task = async move {
                let mut guard = mutex.lock().await;
                while *guard < TARGET {
                    if *guard % WORKER_COUNT == index {
                        *guard += 1;
                        condvar.notify_all();
                    } else {
                        guard = condvar.wait(guard).await;
                    }
                }
            };
            ex.spawn(task).detach();
        }
        smol::block_on(ex.run(async move {
            let mut guard = mutex.lock().await;
            while *guard != TARGET {
                guard = condvar.wait(guard).await;
            }
        }));
    }
}
