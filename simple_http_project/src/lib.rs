use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // use loop instead while because while block the receiver
        // loop execute and rerun the body
        // 'n is useless, only to remember
        let thread = thread::spawn(move || 'n: loop {
            let job = receiver.lock().unwrap().recv();

            match job {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(err) => {
                    println!("Worker {id} disconnected; shutting down. {err:?}");
                    break 'n;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Size must be greater than 0");
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        // type copied of method spawn
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.iter_mut() {
            println!("Shutting down worker {}", worker.id);
            if let Some(worker) = worker.thread.take() {
                worker.join().unwrap();
            }
        }
    }
}
