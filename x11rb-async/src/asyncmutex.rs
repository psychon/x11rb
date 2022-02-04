use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};

enum MutexState<T> {
    Data(Box<T>),
    Poison,
}

#[derive(Debug)]
pub struct PoisonError {
    _priv: (),
}

pub struct MutexGuard<'a, T> {
    mutex: &'a AsyncMutex<T>,
    data: Box<T>,
    guard: std::sync::MutexGuard<'a, MutexState<T>>,
}

impl<'a, T> MutexGuard<'a, T> {
    pub fn unlock(mut self) {
        *self.guard = MutexState::Data(self.data);

        // Wake all waiters. This can end up as a thundering herd, but is simple
        let mut waiters = self.mutex.waiters.lock().unwrap();
        waiters.drain(..).for_each(|waker| waker.wake());
    }
}

impl<'a, T> std::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data.deref()
    }
}

impl<'a, T> std::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data.deref_mut()
    }
}

pub struct LockFuture<'a, T>(&'a AsyncMutex<T>);

impl<'a, T> Future for LockFuture<'a, T> {
    type Output = Result<MutexGuard<'a, T>, PoisonError>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        // Lock waiters first to protect against parallel drops of MutexGuard
        let mut waiters = self.0.waiters.lock().unwrap();

        let mut lock = self.0.data.try_lock();
        if let Ok(mut guard) = lock {
            // We managed to lock the data, return result to caller
            let result = match std::mem::replace(&mut *guard, MutexState::Poison) {
                MutexState::Poison => Err(PoisonError { _priv: () }),
                MutexState::Data(data) => Ok(MutexGuard {
                    mutex: self.0,
                    data,
                    guard,
                }),
            };
            Poll::Ready(result)
        } else {
            // Currently locked, arrange for us to wake up later
            waiters.push_back(ctx.waker().clone());
            Poll::Pending
        }
    }
}

pub struct AsyncMutex<T> {
    data: Mutex<MutexState<T>>,
    waiters: Mutex<VecDeque<Waker>>,
}

impl<T> AsyncMutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Mutex::new(MutexState::Data(Box::new(data))),
            waiters: Default::default(),
        }
    }

    pub fn lock(&self) -> LockFuture<'_, T> {
        LockFuture(self)
    }
}
