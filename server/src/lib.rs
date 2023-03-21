use std::thread;

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize) -> Self {
        Self {
            id,
            handle: thread::spawn(|| {})
        }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of therads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // Create some workers and store them in the vector
            workers.push(Worker::new(id)) 
        } 
        ThreadPool { workers }
    }
    /// Another way to create a new ThreadPool which uses Result
    /// 
    /// This has the advantage of not causing an unrecoverable state if 0 is passed as the pool size
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        match size {
            0 => {return Err(PoolCreationError)},
            _ => {
                let mut threads: Vec<thread::JoinHandle<()>> = Vec::with_capacity(size);
                return Ok(ThreadPool { threads })
            }
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        
    }
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;
