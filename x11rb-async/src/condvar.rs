use async_lock::MutexGuard;

use event_listener::Event;

/// Async condition variable
#[derive(Debug, Default)]
pub struct Condvar(Event);

impl Condvar {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn wait<'g, T>(&self, guard: MutexGuard<'g, T>) -> MutexGuard<'g, T> {
        let source = MutexGuard::source(&guard);

        // Register a listener
        let listen = self.0.listen();
        // ...and drop the mutex only afterwards so that we do not miss any wakeups
        drop(guard);
        // Now wait for a notification
        listen.await;
        source.lock().await
    }

    pub fn notify_all(&self) {
        // Wake all waiting futures
        self.0.notify_additional(usize::MAX);
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

    #[test]
    fn condvar_wakes_multiple_tasks() {
        let ex = Executor::new();
        let mutex_condvar = Arc::new((Mutex::new(0u32), Condvar::new()));

        for _ in 0..2 {
            let mutex_condvar = Arc::clone(&mutex_condvar);
            let task = async move {
                let mut guard = mutex_condvar.0.lock().await;
                *guard += 1;
                while *guard != 10 {
                    guard = mutex_condvar.1.wait(guard).await;
                }
            };
            ex.spawn(task).detach()
        }

        // Make sure the tasks are waiting
        while ex.try_tick() {}

        // Both tasks should have incremented the value
        let value = smol::block_on(ex.run(async { *mutex_condvar.0.lock().await }));
        assert_eq!(value, 2);

        // Let the tasks exit. This actually tests that notify_all() notifies all
        smol::block_on(ex.run(async {
            let mut guard = mutex_condvar.0.lock().await;
            *guard = 10;
            mutex_condvar.1.notify_all();
        }));

        // Wait for everything to exit
        while ex.try_tick() {}
    }
}
