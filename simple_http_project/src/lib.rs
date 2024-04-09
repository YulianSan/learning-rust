use std::thread;

pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        // type copied of method spawn
        F: FnOnce() + Send + 'static,
    {
    }
}
