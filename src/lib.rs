pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for id in 0..size {
            threads.push(Worker::new(id));
        }
        
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);  
        self.workers[0].execute(job);
    }
}
