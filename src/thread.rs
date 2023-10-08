use crate::prelude::Block;
use std::{cell::RefCell, future::IntoFuture, thread::sleep, time::Duration};

pub struct Task<T> {
    reciver: std::sync::mpsc::Receiver<T>,
    ready: RefCell<Option<T>>,
}

impl<T> Task<T> {
    fn from_receiver(reciver: std::sync::mpsc::Receiver<T>) -> Self {
        Self {
            reciver,
            ready: RefCell::new(None),
        }
    }

    pub fn is_ready(&self) -> bool {
        if self.ready.borrow().is_some() {
            return true;
        }

        match self.reciver.try_recv() {
            Ok(v) => {
                self.ready.replace(Some(v));
                true
            }
            Err(_) => false,
        }
    }

    #[inline]
    pub fn wait(self) -> T {
        if let Some(r) = self.ready.take() {
            return r;
        }
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

    #[inline(always)]
    fn spawn_task_async<ASYNC>(&self, op: ASYNC) -> Task<T>
    where
        ASYNC: IntoFuture<Output = T> + Send + 'scope,
    {
        self.spawn_task(|_| op.block())
    }
}

pub trait CollectAll<T> {
    fn wait_all(self) -> Vec<T>;
    fn wait_any(self) -> T;
}

impl<T, I> CollectAll<T> for I
where
    I: std::iter::IntoIterator<Item = Task<T>>,
{
    fn wait_all(self) -> Vec<T> {
        let tasks: Vec<Task<T>> = self.into_iter().collect();
        tasks.into_iter().map(|t| t.wait()).collect()
    }

    fn wait_any(self) -> T {
        let mut tasks: Vec<Task<T>> = self.into_iter().collect();
        let loop_break = Duration::from_micros(500);
        loop {
            for (i, t) in tasks.iter().enumerate() {
                if t.is_ready() {
                    return tasks.swap_remove(i).wait();
                }
            }
            sleep(loop_break);
        }
    }
}

pub struct AsyncMap<I, B, F>
where
    I: Iterator,
    F: Fn(<I as Iterator>::Item) -> B,
{
    iter: I,
    fun: std::sync::Arc<F>,
}

impl<I, B, F> AsyncMap<I, B, F>
where
    I: Iterator,
    F: Fn(<I as Iterator>::Item) -> B,
{
    fn new(iter: I, fun: F) -> Self {
        Self {
            iter,
            fun: std::sync::Arc::new(fun),
        }
    }
}

impl<I, B, F> Iterator for AsyncMap<I, B, F>
where
    I: Iterator,
    <I as Iterator>::Item: Send + 'static,
    F: Fn(I::Item) -> B + Send + Sync + 'static,
    B: Send + 'static,
{
    type Item = Task<B>;

    fn next(&mut self) -> Option<Self::Item> {
        let fun = self.fun.clone();
        match self.iter.next() {
            Some(v) => Some(DEFAULT_POOL.spawn_task(move || fun(v))),
            None => None,
        }
    }
}

pub trait AsyncMapIter<I, B, F>
where
    I: Iterator,
    <I as Iterator>::Item: Send + 'static,
    F: Fn(I::Item) -> B + Send + Sync + 'static,
    B: Send + 'static,
{
    fn async_map(self, f: F) -> AsyncMap<I, B, F>;
}

impl<I, B, F> AsyncMapIter<I, B, F> for I
where
    I: Iterator,
    <I as Iterator>::Item: Send + 'static,
    F: Fn(I::Item) -> B + Send + Sync + 'static,
    B: Send + 'static,
{
    fn async_map(self, f: F) -> AsyncMap<I, B, F> {
        AsyncMap::new(self, f)
    }
}

pub static DEFAULT_POOL: crate::Lazy<rayon::ThreadPool> = crate::Lazy::new(build_default_pool);

pub fn build_default_pool() -> rayon::ThreadPool {
    let threads = std::thread::available_parallelism()
        .map(|num| num.get())
        .unwrap_or(8);

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads / 2)
        .build()
        .expect("Unable to create thread pool")
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test() {
        DEFAULT_POOL.instance();
        for _ in 0..1000 {
            let now = Instant::now();
            let result = 20;
            let res = DEFAULT_POOL.spawn_task(move || result + 1).wait();
            println!("{res:?} , took: {} micros", now.elapsed().as_micros());
        }
    }

    #[tokio::test]
    async fn test_async() {
        DEFAULT_POOL.instance();
        for _ in 0..1000 {
            let now = Instant::now();
            let result = 20;
            let res = DEFAULT_POOL
                .spawn_task_async(async move { result + 1 })
                .wait();
            println!("{res:?} , took: {} micros", now.elapsed().as_micros());
        }
    }

    #[test]
    fn test_scope() {
        DEFAULT_POOL.scope(|scope| {
            for _ in 0..1000 {
                let now = Instant::now();
                let res = scope.spawn_task(|_| 1 + 1).wait();
                println!("{res:?} , took: {} micros", now.elapsed().as_micros());
            }
        })
    }

    #[test]
    fn test_scope_async() {
        DEFAULT_POOL.scope(|scope| {
            let now = Instant::now();
            let res = scope.spawn_task_async(async { 10 }).wait();
            println!("{res:?} , took: {}micro", now.elapsed().as_micros());
        });
    }

    #[test]
    fn test_collect_tasks() {
        let mut tasks: Vec<Task<i32>> = Vec::new();

        for i in 0..15 {
            tasks.push(DEFAULT_POOL.spawn_task(move || {
                std::thread::sleep(Duration::from_nanos(250_000));
                i + 1
            }))
        }

        for t in &tasks {
            std::thread::sleep(Duration::from_nanos(1));
            println!("is ready: {}", t.is_ready())
        }

        let finised = tasks.wait_all();
        println!("is ready all: {finised:?}")
    }

    #[test]
    fn test_async_map_iter() {
        let tasks: Vec<i32> = (0..8)
            .into_iter()
            .async_map(|t| {
                std::thread::sleep(Duration::from_millis(100));
                t * t
            })
            .wait_all();

        println!("is ready all: {tasks:?}")
    }

    #[test]
    fn test_async_map_iter_wait_any() {
        let result: i32 = (0..1000)
            .into_iter()
            .rev()
            .async_map(|t| {
                std::thread::sleep(Duration::from_millis(100));
                t
            })
            .wait_any();

        println!("one is ready: {result}")
    }
}
