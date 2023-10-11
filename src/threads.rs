pub struct Task<T> {
    reciver: std::sync::mpsc::Receiver<T>,
    ready: std::cell::RefCell<Option<T>>,
}

impl<T> Task<T> {
    fn from_receiver(reciver: std::sync::mpsc::Receiver<T>) -> Self {
        Self {
            reciver,
            ready: std::cell::RefCell::new(None),
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

mod spawn_task_rayon {
    use super::{Task, CPU_POOL};
    use crate::prelude::Block;
    use std::{future::IntoFuture, thread::sleep, time::Duration};

    pub trait ChainTask<T>
    where
        T: Send + 'static,
    {
        fn chain_task<OP, R>(self, op: OP) -> Task<R>
        where
            R: Send + 'static,
            OP: FnOnce(T) -> R + Send + 'static;

        fn chain_task_pool<OP, R>(self, pool: &rayon::ThreadPool, op: OP) -> Task<R>
        where
            R: Send + 'static,
            OP: FnOnce(T) -> R + Send + 'static;
    }

    impl<T> ChainTask<T> for Task<T>
    where
        T: Send + 'static,
    {
        fn chain_task<OP, R>(self, op: OP) -> Task<R>
        where
            R: Send + 'static,
            OP: FnOnce(T) -> R + Send + 'static,
        {
            CPU_POOL.spawn_task(|| op(self.wait()))
        }

        fn chain_task_pool<OP, R>(self, pool: &rayon::ThreadPool, op: OP) -> Task<R>
        where
            R: Send + 'static,
            OP: FnOnce(T) -> R + Send + 'static,
        {
            pool.spawn_task(|| op(self.wait()))
        }
    }

    pub trait IntoTask<T>
    where
        T: Send + 'static,
    {
        fn into_task<OP, R>(self, op: OP) -> Task<R>
        where
            R: Send + 'static,
            OP: FnOnce(T) -> R + Send + 'static;

        fn into_task_pool<OP, R>(self, pool: &rayon::ThreadPool, op: OP) -> Task<R>
        where
            R: Send + 'static,
            OP: FnOnce(T) -> R + Send + 'static;
    }

    impl<T: Send + 'static> IntoTask<T> for T {
        fn into_task<OP, R>(self, op: OP) -> Task<R>
        where
            R: Send + 'static,
            OP: FnOnce(T) -> R + Send + 'static,
        {
            CPU_POOL.spawn_task(|| op(self))
        }

        fn into_task_pool<OP, R>(self, pool: &rayon::ThreadPool, op: OP) -> Task<R>
        where
            R: Send + 'static,
            OP: FnOnce(T) -> R + Send + 'static,
        {
            pool.spawn_task(|| op(self))
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

    pub trait CollectTasks<T> {
        fn wait_all(self) -> Vec<T>;
        fn wait_any(self) -> T;
    }

    impl<T, I> CollectTasks<T> for I
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
}
pub use spawn_task_rayon::*;

mod async_map {
    use std::marker::PhantomData;

    use super::{SpawnTask, Task, CPU_POOL};

    pub struct AsyncMap<'a, I, F>
    where
        I: Iterator,
    {
        iter: I,
        pool: &'a rayon::ThreadPool,
        fun: std::sync::Arc<F>,
    }

    impl<'a, I, B, F> AsyncMap<'a, I, F>
    where
        I: Iterator,
        F: Fn(<I as Iterator>::Item) -> B,
    {
        fn new(iter: I, pool: &'a rayon::ThreadPool, fun: F) -> Self {
            Self {
                iter,
                pool,
                fun: std::sync::Arc::new(fun),
            }
        }
    }

    impl<'a, I, B, F> Iterator for AsyncMap<'a, I, F>
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
                Some(v) => Some(self.pool.spawn_task(move || fun(v))),
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
        fn async_map<'a>(self, f: F) -> AsyncMap<'a, I, F>;
        fn async_map_pool<'a>(self, pool: &'a rayon::ThreadPool, f: F) -> AsyncMap<'a, I, F>;
    }

    impl<I, B, F> AsyncMapIter<I, B, F> for I
    where
        I: Iterator,
        <I as Iterator>::Item: Send + 'static,
        F: Fn(I::Item) -> B + Send + Sync + 'static,
        B: Send + 'static,
    {
        fn async_map<'a>(self, f: F) -> AsyncMap<'a, I, F> {
            AsyncMap::new(self, &CPU_POOL, f)
        }

        fn async_map_pool<'a>(self, pool: &'a rayon::ThreadPool, f: F) -> AsyncMap<'a, I, F> {
            AsyncMap::new(self, pool, f)
        }
    }

    pub struct AsyncChain<'a, I, T, F>
    where
        I: Iterator<Item = Task<T>>,
    {
        iter: I,
        pool: &'a rayon::ThreadPool,
        fun: std::sync::Arc<F>,
        input: PhantomData<T>,
    }

    impl<'a, I, T, B, F> AsyncChain<'a, I, T, F>
    where
        I: Iterator<Item = Task<T>>,
        F: Fn(T) -> B,
    {
        fn new(iter: I, pool: &'a rayon::ThreadPool, fun: F) -> Self {
            Self {
                iter,
                pool,
                fun: std::sync::Arc::new(fun),
                input: PhantomData,
            }
        }
    }

    impl<'a, I, T, B, F> Iterator for AsyncChain<'a, I, T, F>
    where
        I: Iterator<Item = Task<T>>,
        <I as Iterator>::Item: Send + 'static,
        F: Fn(T) -> B + Send + Sync + 'static,
        B: Send + 'static,
        T: Send + 'static,
    {
        type Item = Task<B>;

        fn next(&mut self) -> Option<Self::Item> {
            let fun = self.fun.clone();
            match self.iter.next() {
                Some(v) => Some(self.pool.spawn_task(move || fun(v.wait()))),
                None => None,
            }
        }
    }

    pub trait AsyncChainIter<I, T, B, F>
    where
        I: Iterator<Item = Task<T>>,
        F: Fn(T) -> B + Send + Sync + 'static,
        B: Send + 'static,
        T: Send + 'static,
    {
        fn async_chain<'a>(self, f: F) -> AsyncChain<'a, I, T, F>;
    }

    impl<I, T, B, F> AsyncChainIter<I, T, B, F> for I
    where
        I: Iterator<Item = Task<T>>,
        F: Fn(T) -> B + Send + Sync + 'static,
        B: Send + 'static,
        T: Send + 'static,
    {
        fn async_chain<'a>(self, f: F) -> AsyncChain<'a, I, T, F> {
            AsyncChain::new(self, &CPU_POOL, f)
        }
    }
}
pub use async_map::*;

pub static CPU_POOL: crate::Lazy<rayon::ThreadPool> = crate::Lazy::new(|| {
    let cpus = std::thread::available_parallelism()
        .map(|num| num.get())
        .unwrap_or(8);

    build_pool(cpus / 2)
});

pub static IO_POOL: crate::Lazy<rayon::ThreadPool> = crate::Lazy::new(|| {
    let cpus = std::thread::available_parallelism()
        .map(|num| num.get())
        .unwrap_or(8);

    build_pool(cpus * 4)
});

pub fn build_pool(threads: usize) -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("Unable to create thread pool")
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test() {
        for _ in 0..1000 {
            let now = Instant::now();
            let res = 20.into_task(|v| v + 1).wait();
            println!("{res:?} , took: {} micros", now.elapsed().as_micros());
        }
    }

    #[test]
    fn test_chain() {
        let res1 = 20.into_task(|v| v + 1).wait().into_task(|v| v + 1).wait();
        let res2 = 20.into_task(|v| v + 1).chain_task(|v| v + 1).wait();
        assert_eq!(res1, res2)
    }

    #[test]
    fn test_chain_lazy() {
        IO_POOL.instance();
        CPU_POOL.instance();
        let now = Instant::now();

        for i in 0..1000 {
            i.into_task(|v| v)
                .chain_task_pool(&IO_POOL, |v| v)
                .chain_task(|v| {
                    std::thread::sleep(Duration::from_millis(20));
                    println!("non blocking until call wait() {v}")
                });
        }

        let spawn_took = now.elapsed().as_millis();
        println!("took: {spawn_took} ms");
        assert!(spawn_took < 20);
        // should print execution message after 20ms
        std::thread::sleep(Duration::from_millis(30));
    }

    #[tokio::test]
    async fn test_async() {
        for _ in 0..1000 {
            let now = Instant::now();
            let result = 20;
            let res = CPU_POOL.spawn_task_async(async move { result + 1 }).wait();
            println!("{res:?} , took: {} micros", now.elapsed().as_micros());
        }
    }

    #[test]
    fn test_scope() {
        CPU_POOL.scope(|scope| {
            for _ in 0..1000 {
                let now = Instant::now();
                let res = scope.spawn_task(|_| 1 + 1).wait();
                println!("{res:?} , took: {} micros", now.elapsed().as_micros());
            }
        })
    }

    #[test]
    fn test_scope_async() {
        CPU_POOL.scope(|scope| {
            let now = Instant::now();
            let res = scope.spawn_task_async(async { 10 }).wait();
            println!("{res:?} , took: {}micro", now.elapsed().as_micros());
        });
    }

    #[test]
    fn test_collect_tasks() {
        let mut tasks: Vec<Task<i32>> = Vec::new();

        for i in 0..30_000 {
            tasks.push(IO_POOL.spawn_task(move || {
                std::thread::sleep(Duration::from_nanos(250_000));
                i + 1
            }))
        }

        println!("all push");

        for t in &tasks {
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

    #[test]
    fn test_async_map_iter_wait_any_on_specific_pool() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(6)
            .build()
            .unwrap();

        let result: Vec<String> = (0..6)
            .into_iter()
            .rev()
            .async_map_pool(&pool, |t| {
                std::thread::sleep(Duration::from_millis(100));
                t
            })
            .async_chain(|v| {
                std::thread::sleep(Duration::from_millis(110));
                format!("  Value is: {}", v)
            })
            .wait_all();

        println!("one is ready: {result:?}")
    }
}
