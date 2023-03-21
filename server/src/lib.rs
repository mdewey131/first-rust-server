use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    
};

struct Worker {
    id: usize,
    handle: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        Self {
            id,
            handle: thread::spawn(|| {receiver;})
        }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}
/// A type alias for a trait object that holds the type of closure which ThreadPool's execute method will receive
type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let (sender, receiver) = mpsc::channel();
        // Arc types let mutltiple workers own the receiver, and Mutex ensures that only one worker gets a job from the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            // Create some workers and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver))) 
        } 

        ThreadPool { workers, sender }
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
       let job = Box::new(f);
       self.sender.send(job).unwrap(); 
    }
}

#[derive(Debug, Clone)]
pub struct PoolCreationError;
