pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool {
            workers: Vec::with_capacity(size),
        }
    }
}