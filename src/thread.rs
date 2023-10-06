use crate::prelude::Block;
use std::future::IntoFuture;

#[repr(transparent)]
pub struct Task<T> {
    reciver: std::sync::mpsc::Receiver<T>,
}

impl<T> Task<T> {

    fn from_receiver(reciver: std::sync::mpsc::Receiver<T>) -> Self {
        Self { reciver }
    }

    #[inline]
    pub fn wait(self) -> T {
        self.reciver.recv().expect("Error in task")
    }

    #[inline]
    pub fn wait_for(self, timeout: std::time::Duration) -> Option<T> {
        self.reciver.recv_timeout(timeout).ok()
    }
}

pub trait SpawnTask<T>
where
    T: Send + 'static,
{
    fn spawn_task<OP>(&self, op: OP) -> Task<T>
    where
        OP: FnOnce() -> T + Send + 'static;

    fn spawn_task_async<A>(&self, op: A) -> Task<T>
    where
        A: IntoFuture<Output = T> + Send + 'static;
}

impl<T> SpawnTask<T> for rayon::ThreadPool
where
    T: Send + 'static,
{
    fn spawn_task<TASK>(&self, op: TASK) -> Task<T>
    where
        TASK: FnOnce() -> T + Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        self.spawn(move || {
            let _ = tx.send(op());
        });
        Task::from_receiver(rx)
    }

    #[inline(always)]
    fn spawn_task_async<ASYNC>(&self, op: ASYNC) -> Task<T>
    where
        ASYNC: IntoFuture<Output = T> + Send + 'static,
    {
        self.spawn_task(|| op.block())
    }
}

pub trait SpawnScopeTask<'scope, T>
where
    T: Send + 'scope,
{
    fn spawn_task<TASK>(&self, op: TASK) -> Task<T>
    where
        TASK: FnOnce(&rayon::Scope<'scope>) -> T + Send + 'scope;

    fn spawn_task_async<ASYNC>(&self, op: ASYNC) -> Task<T>
    where
        ASYNC: IntoFuture<Output = T> + Send + 'scope;
}

impl<'scope, T> SpawnScopeTask<'scope, T> for rayon::Scope<'scope>
where
    T: Send + 'scope,
{
    fn spawn_task<BODY>(&self, op: BODY) -> Task<T>
    where
        BODY: FnOnce(&rayon::Scope<'scope>) -> T + Send + 'scope,
    {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        self.spawn(move |scope| {
            let _ = tx.send(op(scope));
        });
        Task::from_receiver(rx)
    }

    fn spawn_task_async<ASYNC>(&self, op: ASYNC) -> Task<T>
    where
        ASYNC: IntoFuture<Output = T> + Send + 'scope,
    {
        self.spawn_task(|_| op.block())
    }
}

#[cfg(test)]
mod tests {
    use super::{SpawnScopeTask, SpawnTask};
    use crate::Lazy;
    use std::time::Instant;

    static POOL: Lazy<rayon::ThreadPool> = Lazy::new(|| {
        let threads = std::thread::available_parallelism()
            .map(|cpus| cpus.get())
            .unwrap_or(8);

        println!("threads: {threads}");
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads / 2)
            .build()
            .expect("Unable to create thread pool")
    });

    #[test]
    fn test() {
        POOL.instance();
        for _ in 0..1000 {
            let now = Instant::now();
            let result = 20;
            let res = POOL.spawn_task(move || result + 1).wait();
            println!("{res:?} , took: {} micros", now.elapsed().as_micros());
        }
    }

    #[tokio::test]
    async fn test_async() {
        POOL.instance();
        for _ in 0..1000 {
            let now = Instant::now();
            let result = 20;
            let res = POOL.spawn_task_async(async move { result + 1 }).wait();
            println!("{res:?} , took: {} micros", now.elapsed().as_micros());
        }
    }

    #[test]
    fn test_scope() {
        POOL.scope(|scope| {
            for _ in 0..1000 {
                let now = Instant::now();
                let res = scope.spawn_task(|_| 1 + 1).wait();
                println!("{res:?} , took: {} micros", now.elapsed().as_micros());
            }
        })
    }

    #[test]
    fn test_scope_async() {
        POOL.scope(|scope| {
            let now = Instant::now();
            let res = scope.spawn_task_async(async { 10 }).wait();
            println!("{res:?} , took: {}micro", now.elapsed().as_micros());
        });
    }
}

// impl<T> std::future::Future for Task<T> {
//     type Output = T;

//     fn poll(
//         self: std::pin::Pin<&mut Self>,
//         _: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Self::Output> {
//         match self.reciver.recv() {
//             Ok(res) => std::task::Poll::Ready(res),
//             Err(err) => panic!("Task failed: {err:?}"),
//         }
//     }
// }
