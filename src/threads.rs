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
    use super::Task;
    use crate::prelude::Block;
    use std::{future::IntoFuture, thread::sleep, time::Duration};

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
}
pub use spawn_task_rayon::*;

mod async_map {
    use super::{SpawnTask, Task, CPU_POOL};

    pub struct AsyncMap<'a, I, T, B, F>
    where
        I: Iterator,
        F: Fn(T) -> B,
    {
        iter: I,
        pool: &'a rayon::ThreadPool,
        fun: std::sync::Arc<F>,
        input: std::marker::PhantomData<T>,
    }

    impl<'a, I, B, F> AsyncMap<'a, I, <I as Iterator>::Item, B, F>
    where
        I: Iterator,
        F: Fn(<I as Iterator>::Item) -> B,
    {
        fn new(iter: I, pool: &'a rayon::ThreadPool, fun: F) -> Self {
            Self {
                iter,
                pool,
                fun: std::sync::Arc::new(fun),
                input: std::marker::PhantomData,
            }
        }
    }

    impl<'a, I, B, F> Iterator for AsyncMap<'a, I, <I as Iterator>::Item, B, F>
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
        fn async_map<'a>(self, f: F) -> AsyncMap<'a, I, <I as Iterator>::Item, B, F>;
        fn async_map_pool<'a>(
            self,
            pool: &'a rayon::ThreadPool,
            f: F,
        ) -> AsyncMap<'a, I, <I as Iterator>::Item, B, F>;
    }

    impl<I, B, F> AsyncMapIter<I, B, F> for I
    where
        I: Iterator,
        <I as Iterator>::Item: Send + 'static,
        F: Fn(I::Item) -> B + Send + Sync + 'static,
        B: Send + 'static,
    {
        fn async_map<'a>(self, f: F) -> AsyncMap<'a, I, <I as Iterator>::Item, B, F> {
            AsyncMap::new(self, CPU_POOL.to_ref(), f)
        }

        fn async_map_pool<'a>(
            self,
            pool: &'a rayon::ThreadPool,
            f: F,
        ) -> AsyncMap<'a, I, <I as Iterator>::Item, B, F> {
            AsyncMap::new(self, pool, f)
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
            let result = 20;
            let res = CPU_POOL.spawn_task(move || result + 1).wait();
            println!("{res:?} , took: {} micros", now.elapsed().as_micros());
        }
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
            .async_map_pool(&CPU_POOL, |t| {
                let v = t.wait();
                std::thread::sleep(Duration::from_millis(110));
                format!("  Value is: {}", v)
            })
            .wait_all();

        println!("one is ready: {result:?}")
    }
}
