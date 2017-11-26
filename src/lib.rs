use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i));
        }

        ThreadPool {
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker { id, thread: thread::spawn(|| {} ) }
    }
}


