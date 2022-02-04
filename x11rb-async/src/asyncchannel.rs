use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[derive(Debug)]
enum OneshotState<T> {
    Idle,
    Data(T),
    Waker(Waker),
}

#[derive(Debug)]
pub struct OneshotSender<T>(Arc<Mutex<OneshotState<T>>>);

impl<T> OneshotSender<T> {
    pub fn send(self, data: T) {
        let mut guard = self.0.lock().unwrap();
        let old = std::mem::replace(&mut *guard, OneshotState::Data(data));
        match old {
            OneshotState::Waker(waker) => waker.wake(),
            OneshotState::Idle => {}
            OneshotState::Data(_) => unreachable!("Sent twice on a oneshot channel"),
        }
    }
}

#[derive(Debug)]
pub struct OneshotReceiver<T>(Arc<Mutex<OneshotState<T>>>);

impl<T> Future for OneshotReceiver<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.0.lock().unwrap();
        let old = std::mem::replace(&mut *guard, OneshotState::Waker(ctx.waker().clone()));
        if let OneshotState::Data(data) = old {
            Poll::Ready(data)
        } else {
            Poll::Pending
        }
    }
}

pub fn oneshot<T>() -> (OneshotSender<T>, OneshotReceiver<T>) {
    let inner = Arc::new(Mutex::new(OneshotState::Idle));
    let inner2 = Arc::clone(&inner);
    (OneshotSender(inner), OneshotReceiver(inner2))
}

#[derive(Debug)]
enum UnboundedState<T> {
    Data(VecDeque<T>),
    Waker(Waker),
}

#[derive(Debug)]
pub struct UnboundedSender<T>(Arc<Mutex<UnboundedState<T>>>);

impl<T> UnboundedSender<T> {
    pub fn send(&mut self, data: T) {
        let mut guard = self.0.lock().unwrap();
        let state = std::mem::replace(&mut *guard, UnboundedState::Data(Default::default()));
        match state {
            UnboundedState::Waker(waker) => {
                waker.wake();
                let mut queue = VecDeque::new();
                queue.push_back(data);
                *guard = UnboundedState::Data(queue);
            }
            UnboundedState::Data(mut queue) => {
                queue.push_back(data);
                *guard = UnboundedState::Data(queue);
            }
        }
    }
}

#[derive(Debug)]
pub struct UnboundedReceiver<T>(Arc<Mutex<UnboundedState<T>>>);

impl<T> Future for UnboundedReceiver<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.0.lock().unwrap();
        match &mut *guard {
            UnboundedState::Waker(_) => {
                *guard = UnboundedState::Waker(ctx.waker().clone());
                Poll::Pending
            }
            UnboundedState::Data(ref mut queue) => match queue.pop_front() {
                None => {
                    *guard = UnboundedState::Waker(ctx.waker().clone());
                    Poll::Pending
                }
                Some(data) => Poll::Ready(data),
            },
        }
    }
}

pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let inner = Arc::new(Mutex::new(UnboundedState::Data(VecDeque::new())));
    let inner2 = Arc::clone(&inner);
    (UnboundedSender(inner), UnboundedReceiver(inner2))
}
